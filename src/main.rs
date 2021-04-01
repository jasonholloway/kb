#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{Runner, big_machine::BigMachine, mode_machine::ModeMachine, print_keys::PrintKeys};
use std::fmt::Debug;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

mod common;
mod null;
mod machines;
mod sink;


pub fn main() {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            windows::run(create_handler).unwrap();
        } else if #[cfg(unix)] {
            unix::run(&mut create_runner()).unwrap();
        } else {
            null::run(&mut handler, &buff).unwrap();
        }
    }
}


fn create_runner<TRaw: 'static + Debug>() -> Runner<Update<TRaw>> {
    Runner::<Update<TRaw>>::new(vec!(
        Box::from(PrintKeys::new(1, 31)),
        Box::from(ModeMachine::new()),
        Box::from(BigMachine::new()),
        Box::from(PrintKeys::new(3, 32)),
    ))
}


pub enum Action {
    Skip,
    Take
}

pub enum Event<'a, R> {
    In(&'a Update<R>),
    Out(&'a Update<R>)
}

