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
        keys: Bitmap::new(),
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
    keys: Bitmap<U1024>,
    buff: VecDeque<Update<TRaw>>,
    mode: Mode
}

impl<TRaw> Handler<TRaw>
where TRaw: Debug
{

    fn handle(&mut self, update: Update<TRaw>) -> (NextDue, Drain<Update<TRaw>>)
    {
        self.count += 1;
        // println!("{} {:?}", self.count, update);

        if let Key(code, movement, _) = update {
            match movement {
                Up => self.keys.set(code as usize, false),
                Down => self.keys.set(code as usize, true),
            };

            println!("{:?} {:?}", self.keys.into_iter().collect::<Vec<usize>>(), self.mode);
        }

        use Action::*;
        use Mode::*;

        let prev_mode = self.mode;

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

        if next_mode != self.mode { println!("{:?}", next_mode); }

        let action = match (prev_mode, next_mode, &update) {

            (_, AltShiftSpace, Key(57, Down, _)) => {
                self.buff.push_back(Key(42, Up, None));
                self.buff.push_back(Key(56, Up, None));

                self.buff.push_back(Key(28, Down, None));
                println!("RETURN!");

                self.buff.push_back(Key(42, Down, None));
                self.buff.push_back(Key(56, Down, None));
                Take
            },
            (AltShiftSpace, _, Key(57, Up, _)) => {
                self.buff.push_back(Key(28, Up, None));
                println!("~RETURN!");
                Take
            },


            (_, AltShiftJ, Key(36, Down, _)) => {
                self.buff.push_back(Key(42, Up, None));
                self.buff.push_back(Key(56, Up, None));

                self.buff.push_back(Key(108, Down, None));
                println!("DOWN!");

                self.buff.push_back(Key(42, Down, None));
                self.buff.push_back(Key(56, Down, None));
                Take
            },
            (AltShiftJ, _, Key(36, Up, _)) => {
                self.buff.push_back(Key(108, Up, None));
                println!("~DOWN!"); 
                Take
            },


            (_, AltShiftK, Key(37, Down, _)) => {
                self.buff.push_back(Key(42, Up, None));
                self.buff.push_back(Key(56, Up, None));

                self.buff.push_back(Key(103, Down, None));
                println!("UP!");

                self.buff.push_back(Key(42, Down, None));
                self.buff.push_back(Key(56, Down, None));
                Take
            },
            (AltShiftK, _, Key(37, Up, _)) => {
                self.buff.push_back(Key(103, Up, None));
                println!("~UP!");
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
        
        self.mode = next_mode;

        (0, self.buff.drain(..))
    }
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




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2+2, 5);
    }

}
