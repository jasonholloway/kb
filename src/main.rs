#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{RunRef, machine::{Ctx, Machine}, print_keys::PrintKeys, runner::{Ev, Runner}};
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
            const DEV_FILE_GLOB: &str = "/dev/input/by-path/*-event-kbd";
            unix::run(DEV_FILE_GLOB, &mut create_runner()).unwrap();
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
        RunRef::new("print1", Machine::new(PrintKeys::new(1, 31))),
        RunRef::new("print2", Machine::new(PrintKeys::new(1, 33))),
        // RunRef::new("big", Machine::new(BigMachine::new())),
        // RunRef::new("mode", Machine::new(ModeMachine::new())),
        // RunRef::new("lead", Machine::new(LeadMachine::new()))
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
