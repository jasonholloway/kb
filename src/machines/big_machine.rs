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


pub struct Machine1 {
    count: u32,
    in_map: Bitmap<U1024>,
    out_map: Bitmap<U1024>,
    mask_map: Bitmap<U1024>,
    mode: Mode,
}

impl Machine1 {
    pub fn new() -> Machine1 {
        Machine1 {
            count: 0,
            in_map: Bitmap::new(),
            out_map: Bitmap::new(),
            mask_map: Bitmap::new(),
            mode: Mode::Root
        }
    }

    fn mask<TRaw, TSink: Sink<Update<TRaw>>>(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, true);

            if !prev && self.out_map.get(*c as usize) {
                sink.emit(Key(*c, Up, None));
            }
        }
    }

    fn unmask<TRaw, TSink: Sink<Update<TRaw>>>(&mut self, codes: &[u16], sink: &mut TSink) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, false);

            if prev && self.in_map.get(*c as usize) && !self.out_map.get(*c as usize) {
                sink.emit(Key(*c, Down, None));
            }
        }
    }

    fn emit<TRaw, TSink: Sink<Update<TRaw>>>(&mut self, ev: Update<TRaw>, sink: &mut TSink) {
        gather_map(&ev, &mut self.out_map);
        sink.emit(ev);
    }
}

impl<TRaw: Debug, TSink: Sink<Update<TRaw>>> Machine<Update<TRaw>, TSink> for Machine1 {

    fn run<'a>(&mut self, ev: Update<TRaw>, sink: &'a mut TSink) -> () {
        use Mode::*;

        self.count += 1;
        
        gather_map(&ev, &mut self.in_map);

        let prev_mode = self.mode;

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
                self.emit(Key(28, Down, None), sink);
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                self.emit(Key(28, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                self.mask(&[42, 56], sink);
                self.emit(Key(108, Down, None), sink);
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                self.emit(Key(108, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                self.mask(&[42, 56], sink); //should do this on entry/exit rather than each keypress
                self.emit(Key(103, Down, None), sink);
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                self.emit(Key(103, Up, None), sink);
                self.unmask(&[42, 56], sink);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                if let Key(_, _, raw) = &ev {
                    match raw {
                        Some(_) => self.emit(ev, sink),
                        None => {}
                    }
                }
            },
            Take => {}
        };

        if next_mode != self.mode {
            println!("\t\t{:?}", next_mode);
        }

        self.mode = next_mode;
    }
}



pub struct PrintKeys {
    out_map: Bitmap<U1024>,
    tabs: u8,
    colour: u8
}

impl PrintKeys {
    pub fn new(tabs: u8, colour: u8) -> PrintKeys {
        PrintKeys {
            out_map: Bitmap::new(),
            tabs,
            colour
        }
    }

    fn print<TRaw>(&self, ev: &Update<TRaw>) {
        use Update::*;
        
        let new_code = if let Key(c, _, _) = ev { *c } else { 0 as u16 };

        print!("{}", (0..self.tabs).map(|_| '\t').collect::<String>());
        
        print!("[");
        let mut first = true;
        for c in self.out_map.into_iter() {
            if !first {
              print!(", ");
            }

            if c == new_code as usize {
                print!("\x1b[0;{:?}m{:?}\x1b[0m", self.colour, c);
            } else {
                print!("{:?}", c);
            }

            first = false;
        }
        print!("]\t\t");
        println!();
    }
}

impl<TRaw: Debug, TSink: Sink<Update<TRaw>>> Machine<Update<TRaw>, TSink> for PrintKeys {

    fn run(&mut self, ev: Update<TRaw>, sink: &mut TSink) -> () {
        gather_map(&ev, &mut self.out_map);

        if let Key(_, _, _) = ev {
            self.print(&ev);
        }

        sink.emit(ev);
    }

}

