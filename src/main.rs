extern crate libc;
extern crate bitmaps;
extern crate typenum;

use common::*;
use std::collections::VecDeque;
use bitmaps::*;
use typenum::consts::U1024;


#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;


mod common;
mod null;


pub fn main() {
    let mut state = State {
        count: 0,
        keys: Bitmap::new()
    };
    
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            run(windows::WinKb { })
        } else if #[cfg(unix)] {
            unix::run(&mut state, handle).unwrap();
        } else {
            null::run(&mut state, handle).unwrap();
        }
    }
}


struct State {
    count: u32,
    keys: Bitmap<U1024>
}

use common::{Update::*,Movement::*};

fn handle<TRaw : std::fmt::Debug>(
    state: &mut State,
    buff: &mut VecDeque<Update<TRaw>>,
    update: Update<TRaw>
) -> NextDue {
    
    println!("{} {:?}", state.count, update);
    state.count += 1;
    
    match &update {
        Key(code, movement, raw) => {
            match movement {
                Up => state.keys.set(*code as usize, false),
                Down => state.keys.set(*code as usize, true),
            };

            match raw {
                Some(_) => buff.push_back(update),
                None => {}
            };
        }

        _ => {}
    }
    
    0
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2+2, 5);
    }

}
