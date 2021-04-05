#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{
    lead_machine::LeadMachine, mode_machine::ModeMachine, print_keys::PrintKeys, Machine, Runner,
};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};
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


fn create_runner<TRaw>() -> Runner<
    Update<TRaw>,
    HashMap<&'static str, fn() -> Box<dyn Machine<Update<TRaw>, VecDeque<Update<TRaw>>>>>,
>
where
    TRaw: 'static + Debug,
{
    let r = Runner::new(
        hash_map![
            "blah": create_mode_machine
        ],
        &["blah"]
    );

    r
    // Runner::<Update<TRaw>, _>::new(vec![
    //     Box::from(PrintKeys::new(1, 31)),
    //     Box::from(ModeMachine::new()),
    //     Box::from(LeadMachine::new()),
    //     Box::from(PrintKeys::new(3, 32)),
    // ])
}

pub enum Action {
    Skip,
    Take,
}

pub enum Event<'a, R> {
    In(&'a Update<R>),
    Out(&'a Update<R>),
}
