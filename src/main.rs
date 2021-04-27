#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

#[cfg(test)]
extern crate spectral;


use common::*;
use machines::{RunRef, big_machine::BigMachine, lead_machine::LeadMachine, machine::Machine, mode_machine::ModeMachine, print_keys::PrintKeys, runner::Runner};
use std::fmt::Debug;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

mod common;
mod machines;
mod null;


pub fn main() {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            windows::run(create_handler).unwrap();
        } else if #[cfg(unix)] {
            unix::run(
                "/dev/input/by-path/*-event-kbd",
                &mut create_runner()
            ).unwrap();
        } else {
            null::run(&mut handler, &buff).unwrap();
        }
    }
}

fn create_runner<'a, TRaw>() -> Runner<TRaw>
where
    TRaw: 'static + Debug,
{
    Runner::new(vec![
        RunRef::new("printBefore", Machine::new(PrintKeys::new(1, 31))),
        // RunRef::new("big", Machine::new(BigMachine::new())),
        RunRef::new("mode", Machine::new(ModeMachine::new())),
        // RunRef::new("lead", Machine::new(LeadMachine::new())),
        RunRef::new("printAfter", Machine::new(PrintKeys::new(4, 32))),
    ])
}

pub enum Action {
    Skip,
    Take,
}

pub enum Event<'a, R> {
    In(&'a Ev<R>),
    Out(&'a Ev<R>),
}
