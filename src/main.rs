#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{MachineFac, RunRef, Runnable, big_machine::BigMachine, lead_machine::LeadMachine, mode_machine::ModeMachine, print_keys::PrintKeys, runner::Runner};
use std::{fmt::Debug};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

mod common;
mod machines;
mod null;
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

fn create_runner<'a, TRaw>() -> Runner<Update<TRaw>>
where
    TRaw: 'static + Debug,
{
    Runner::new(vec![
        RunRef::new("print1", PrintKeys::new(1, 31)),
        RunRef::new("print2", PrintKeys::new(4, 35)),
        RunRef::new("big", BigMachine::new()),
        RunRef::new("mode", ModeMachine::new()),
        RunRef::new("lead", LeadMachine::new())
    ])
}

pub enum Action {
    Skip,
    Take,
}

pub enum Event<'a, R> {
    In(&'a Update<R>),
    Out(&'a Update<R>),
}



pub fn fac<TEv, TMac, TFn>(f: TFn) -> MachineFac<TEv>
where
    TMac: 'static + Runnable<TEv>,
    TFn: 'static + Fn() -> TMac,
{
    Box::new(move || Box::new(f()))
}
