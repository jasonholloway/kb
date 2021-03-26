extern crate bitmaps;
extern crate typenum;

#[cfg(unix)]
extern crate libc;

use common::*;
use bitmaps::*;
use typenum::*; 

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;


mod common;
mod null;

fn create_handler<T>() -> Handler<T> {
    Handler {
        count: 0,
        in_map: Bitmap::new(),
        out_map: Bitmap::new(),
        mask_map: Bitmap::new(),
        buff: VecDeque::new(),
        mode: Mode::Root
    }
}

pub fn main() {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            windows::run(create_handler).unwrap();
        } else if #[cfg(unix)] {
            unix::run(create_handler).unwrap();
        } else {
            null::run(&mut handler, &buff).unwrap();
        }
    }
}



use common::{Update::*,Movement::*};
use std::fmt::Debug;
use std::collections::vec_deque::*;


trait Sink<T> {
    fn emit(item: &T);
}

enum Action {
    Skip,
    Take
}


pub struct Handler<TRaw> {
    count: u32,
    in_map: Bitmap<U1024>,
    out_map: Bitmap<U1024>,
    mask_map: Bitmap<U1024>,
    buff: VecDeque<Update<TRaw>>,
    mode: Mode
}

impl<TRaw> Handler<TRaw>
where TRaw: Debug
{

    fn mask_add(&mut self, codes: &[u16]) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, true);

            if !prev && self.out_map.get(*c as usize) {
                self.buff.push_back(Key(*c, Up, None));
            }
        }
    }

    fn mask_reset(&mut self, codes: &[u16]) {
        for c in codes {
            let prev = self.mask_map.set(*c as usize, false);

            if prev && self.in_map.get(*c as usize) && !self.out_map.get(*c as usize) {
                self.buff.push_back(Key(*c, Down, None));
            }
        }
    }

    
    fn handle(&mut self, update: Update<TRaw>) -> (NextDue, Drain<Update<TRaw>>)
    {
        self.count += 1;

        gather_map(&update, &mut self.in_map);

        use Action::*;
        use Mode::*;
        use Event::*;

        let prev_mode = self.mode;

        if let Key(_, _, _) = update {
            self.print(In(&update));
        }

        let next_mode = match (prev_mode, &update) {
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

        let action = match (prev_mode, next_mode, &update) {

            (_, AltShiftSpace, Key(57, Down, _)) => {
                self.mask_add(&[42, 56]);
                self.buff.push_back(Key(28, Down, None));
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                self.buff.push_back(Key(28, Up, None));
                self.mask_reset(&[42, 56]);
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                self.mask_add(&[42, 56]);
                self.buff.push_back(Key(108, Down, None));
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                self.buff.push_back(Key(108, Up, None));
                self.mask_reset(&[42, 56]);
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                self.mask_add(&[42, 56]); //should do this on entry/exit rather than each keypress
                self.buff.push_back(Key(103, Down, None));
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                self.buff.push_back(Key(103, Up, None));
                self.mask_reset(&[42, 56]);
                Take
            },

            _ => Skip
        };

        match action {
            Skip => {
                if let Key(_, _, raw) = &update {
                    match raw {
                        Some(_) => self.buff.push_back(update),
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
        (0, self.buff.drain(..))
    }

    fn print(&self, event: Event<TRaw>) {
        use Event::*;
        
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


enum Event<'a, R> {
    In(&'a Update<R>),
    Out(&'a Update<R>)
}



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


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2+2, 5);
    }

}
