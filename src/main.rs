extern crate bitmaps;
extern crate typenum;

#[cfg(unix)]
extern crate libc;

use common::*;
use bitmaps::*;
use evdev_rs::InputEvent;
use typenum::consts::U1024;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;


mod common;
mod null;


pub fn main() {
		let mut handler = Handler {
				count: 0,
				keys: Bitmap::new(),
				buff: VecDeque::new(),
				next: |_, _| {}
		};
    
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            windows::run(&mut state, handle).unwrap();
        } else if #[cfg(unix)] {
            unix::run(&mut handler).unwrap();
        } else {
            null::run(&mut handler, &buff).unwrap();
        }
    }
}


type Behaviour<TRaw>
		= fn (state: &mut Handler<TRaw>, update: &Update<TRaw>) -> ();


use common::{Update::*,Movement::*};
use std::fmt::Debug;
use std::collections::vec_deque::*;



trait Sink<T> {
		fn emit(item: &T);
}


pub struct Handler<TRaw> {
    count: u32,
    keys: Bitmap<U1024>,
		next: Behaviour<TRaw>,
		buff: VecDeque<Update<TRaw>>
}



impl<TRaw> Handler<TRaw>
where
		TRaw: Debug
{

		fn handle(&mut self, update: Update<TRaw>) -> (NextDue, Drain<Update<TRaw>>)
		{
				self.count += 1;
				println!("{} {:?}", self.count, update);
				println!("{:?}", self.keys.into_iter().collect::<Vec<usize>>());

				match &update {
						Key(code, movement, raw) => {
								match movement {
										Up => self.keys.set(*code as usize, false),
										Down => self.keys.set(*code as usize, true),
								};

								match raw {
										Some(_) => self.buff.push_back(update),
										None => {}
								};
						}

						_ => {}
				}

				// (self.next)(self, buff, update);

				let keys = self.keys;

				if keys.get(42) && keys.get(56) && keys.get(57) {

						self.buff.push_back(Key(28, Down, None));
						self.buff.push_back(Key(28, Up, None));

						// but really it's all about the transitions between states
						// and each state links to another on certain detected key events
						// each state is a handler function

				}

				(0, self.buff.drain(..))
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
