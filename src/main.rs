#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

#[cfg(test)]
extern crate spectral;


use common::*;
use machines::{RunRef, lead_machine::LeadMachine, machine::{Ctx, Machine}, mode_machine::ModeMachine, print_keys::PrintKeys, runner::{Ev, Runner}};
use std::fmt::Debug;

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
            unix::run(
                "/dev/input/by-path/*-event-kbd",
                &mut create_runner()
            ).unwrap();
        } else {
            null::run(&mut handler, &buff).unwrap();
        }
    }
}

fn create_runner<'a, TRaw>() -> Runner<Ev<Ctx,Update<TRaw>>>
where
    TRaw: 'static + Debug,
{
    Runner::new(vec![
        RunRef::new("printBefore", Machine::new(PrintKeys::new(1, 31))),
        // RunRef::new("print2", Machine::new(PrintKeys::new(1, 33))),
        // RunRef::new("big", Machine::new(BigMachine::new())),
        RunRef::new("mode", Machine::new(ModeMachine::new())),
        RunRef::new("lead", Machine::new(LeadMachine::new())),
        RunRef::new("printAfter", Machine::new(PrintKeys::new(4, 32))),
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



// pub fn fac<TCtx, TEv, TMac, TFn>(f: TFn) -> MachineFac<TEv>
// where
//     TMac: 'static + Runnable<TCtx,TEv>,
//     TFn: 'static + Fn() -> TMac,
// {
//     Box::new(move || Box::new(f()))
// }
