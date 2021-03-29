use std::collections::VecDeque;

use bitmaps::{Bitmap, Bits};
use std::fmt::Debug;
use typenum::*;

use crate::{Event, common::{Movement::*, *}, sink::Sink};
use Update::*;
use crate::Action::*;
use super::Machine;


#[derive(PartialEq, Copy, Clone, Debug)]
enum Mode {
    Root,
    Shift,
    Alt,
    AltShift,
    AltShiftSpace,
    AltShiftJ,
    AltShiftK
}



fn gather_map<T, T2: Bits>(event: &Update<T>, map: &mut Bitmap<T2>) {
    if let Key(code, movement, _) = event {
        match movement {
            Up => map.set(*code as usize, false),
            Down => map.set(*code as usize, true),
        };
    }
}

pub struct Machine1<TRaw> {
    count: u32,
    in_map: Bitmap<U1024>,
    out_map: Bitmap<U1024>,
    mask_map: Bitmap<U1024>,
    buff: VecDeque<Update<TRaw>>,
    mode: Mode,
}

impl<TRaw> Machine1<TRaw> where TRaw: Debug {
    pub fn new() -> Machine1<TRaw> {
        Machine1 {
            count: 0,
            in_map: Bitmap::new(),
            out_map: Bitmap::new(),
            mask_map: Bitmap::new(),
            buff: VecDeque::new(),
            mode: Mode::Root
        }
    }

    fn mask<TSink: Sink<Update<TRaw>>>(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, true);

            if !prev && self.out_map.get(*c as usize) {
                sink.emit(Key(*c, Up, None));
            }
        }
    }

    fn unmask<TSink: Sink<Update<TRaw>>>(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, false);

            if prev && self.in_map.get(*c as usize) && !self.out_map.get(*c as usize) {
                sink.emit(Key(*c, Down, None));
            }
        }
    }


    fn print(&self, event: Event<TRaw>) {
        use Event::*;
        use Update::*;
        
        let new_in_code = if let In(Key(c, _, _)) = event { *c } else { 0 as u16 };
        let new_out_code = if let Out(Key(c, _, _)) = event { *c } else { 0 as u16 };

        print!("{:?}\t\t\t", self.mode);

        print!("[");
        let mut first = true;
        for c in self.in_map.into_iter() {
            if !first {
              print!(", ");
            }

            if c == new_in_code as usize {
                print!("\x1b[0;31m{:?}\x1b[0m", c);
            } else {
                print!("{:?}", c);
            }

            first = false;
        }
        print!("]\t\t");
        
        print!("[");
        let mut first = true;
        for c in self.out_map.into_iter() {
            if !first {
              print!(", ");
            }

            if c == new_out_code as usize {
                print!("\x1b[0;32m{:?}\x1b[0m", c);
            } else {
                print!("{:?}", c);
            }

            first = false;
        }
        print!("]\t\t");
        println!();
    }
    
}

impl<TRaw: Debug, TSink: Sink<Update<TRaw>>> Machine<Update<TRaw>, TSink> for Machine1<TRaw> {

    fn run(&mut self, ev: Update<TRaw>, sink: &mut TSink) -> () {
        use Mode::*;
        use Event::*;

        self.count += 1;

        gather_map(&ev, &mut self.in_map);

        let prev_mode = self.mode;

        if let Key(_, _, _) = ev {
            self.print(In(&ev));
        }

        let next_mode = match (prev_mode, &ev) {
            (Root, Key(42, Down, _)) => Shift,
            (Root, Key(56, Down, _)) => Alt,

            (Shift, Key(42, Up, _)) => Root,
            (Shift, Key(56, Down, _)) => AltShift,

            (Alt, Key(56, Up, _)) => Root,
            (Alt, Key(42, Down, _)) => AltShift,

            (AltShift, Key(42, Up, _)) => Alt,
            (AltShift, Key(56, Up, _)) => Shift,
            (AltShift, Key(36, Down, _)) => AltShiftJ,
            (AltShift, Key(37, Down, _)) => AltShiftK,
            (AltShift, Key(57, Down, _)) => AltShiftSpace,

            (AltShiftSpace, Key(42, Up, _)) => Root,
            (AltShiftSpace, Key(56, Up, _)) => Root,
            (AltShiftSpace, Key(57, Up, _)) => AltShift,

            (AltShiftJ, Key(42, Up, _)) => Root,
            (AltShiftJ, Key(56, Up, _)) => Root,
            (AltShiftJ, Key(36, Up, _)) => AltShift,

            (AltShiftK, Key(42, Up, _)) => Root,
            (AltShiftK, Key(56, Up, _)) => Root,
            (AltShiftK, Key(37, Up, _)) => AltShift,

            _ => prev_mode
        };

        let action = match (prev_mode, next_mode, &ev) {

            (_, AltShiftSpace, Key(57, Down, _)) => {
                self.mask(&[42, 56], sink);
                sink.emit(Key(28, Down, None));
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                sink.emit(Key(28, Up, None));
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                self.mask(&[42, 56], sink);
                sink.emit(Key(108, Down, None));
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                sink.emit(Key(108, Up, None));
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                self.mask(&[42, 56], sink); //should do this on entry/exit rather than each keypress
                sink.emit(Key(103, Down, None));
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                sink.emit(Key(103, Up, None));
                self.unmask(&[42, 56], sink);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                if let Key(_, _, raw) = &ev {
                    match raw {
                        Some(_) => sink.emit(ev),
                        None => {}
                    }
                }
            },
            Take => {}
        };

        for out_event in &self.buff {
            gather_map(&out_event, &mut self.out_map);
            self.print(Out(out_event));
        }

        self.mode = next_mode;
    }
}

