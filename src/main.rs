#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{MachineFac, Runnable, big_machine::BigMachine, lead_machine::LeadMachine, mode_machine::ModeMachine, print_keys::PrintKeys, runner::Runner};
use std::{collections::HashMap, fmt::Debug};
use velcro::hash_map;

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

fn create_runner<'a, TRaw>() -> Runner<Update<TRaw>, HashMap<&'a str, MachineFac<Update<TRaw>>>>
where
    TRaw: 'static + Debug,
{
    Runner::new(
        hash_map![
            "print1": fac(|| PrintKeys::new(1, 31)),
            "print2": fac(|| PrintKeys::new(4, 35)),
            "big": fac(|| BigMachine::new()),
            "modes": fac(|| ModeMachine::new()),
            "leads": fac(|| LeadMachine::new())
        ],
        vec!["print1", "modes", "leads", "print2"],
    )
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
