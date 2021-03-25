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
            unix::run(&mut handler).unwrap();
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


pub struct Handler<TRaw> {
    count: u32,
    keys: Bitmap<U1024>,
    buff: VecDeque<Update<TRaw>>,
    mode: Mode
}





impl<TRaw> Handler<TRaw>
where
    TRaw: Debug
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
        }

        println!("{:?}", self.keys.into_iter().collect::<Vec<usize>>());


        use Mode::*;
        let current_mode = self.mode;

        match (current_mode, &update) {
            (AltShiftJ, Key(36, Up, _)) => {
                println!("~DOWN!")
            },
            (AltShiftK, Key(37, Up, _)) => {
                self.buff.push_back(Key(103, Up, None));
                println!("~UP!")
            },
            (AltShiftSpace, Key(57, Up, _)) => {
                self.buff.push_back(Key(28, Up, None));
                println!("~RETURN!")
            },

            _ => {}
        }

        let next_mode = match (current_mode, &update) {
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

            _ => current_mode
        };

        if next_mode != self.mode {
            println!("{:?}", next_mode);
            self.mode = next_mode;
        }

        match (self.mode, &update) {
            (AltShiftJ, Key(36, Down, _)) => {
                println!("DOWN!")
            },
            (AltShiftK, Key(37, Down, _)) => {
                self.buff.push_back(Key(103, Down, None));
                
                println!("UP!")
            },
            (AltShiftSpace, Key(57, Down, _)) => {
                self.buff.push_back(Key(28, Down, None));
                println!("RETURN!")
            },

            _ => {}
        }
        

        if let Key(_, _, raw) = &update {
            match raw {
                Some(_) => self.buff.push_back(update),
                None => {}
            }
        }

        // let keys = self.keys;

        // if keys.get(42) && keys.get(56) && keys.get(57) {

        //    self.buff.push_back(Key(28, Down, None));
        //    self.buff.push_back(Key(28, Up, None));
        // }

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
