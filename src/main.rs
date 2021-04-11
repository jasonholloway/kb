#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{MachineFac, Runner, big_machine::BigMachine, print_keys::PrintKeys};
use std::{collections::{HashMap}, fmt::Debug};
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

fn create_runner<'a, TRaw>() -> Runner<
    Update<TRaw>,
    HashMap<&'a str, MachineFac<Update<TRaw>>>
>
where
    TRaw: 'static + Debug,
{
    Runner::new(
        hash_map![
            "print1": print_keys_fac(),
            "print2": print_keys_fac(),
            "blah": dynamic_machine_fac(),
            "big": big_machine_fac()
        ],
        vec![
            "print1", "big", "print2"
        ])
}

fn print_keys_fac<TRaw: Debug>() -> MachineFac<Update<TRaw>>
{
    Box::new(|| Box::new(PrintKeys::new(1, 32)))
}

fn dynamic_machine_fac<TRaw: Debug>() -> MachineFac<Update<TRaw>>
{
    Box::new(|| Box::new(PrintKeys::new(1, 32)))
}

fn big_machine_fac<TRaw: Debug>() -> MachineFac<Update<TRaw>>
{
    Box::new(|| Box::new(BigMachine::new()))
}


pub enum Action {
    Skip,
    Take,
}

pub enum Event<'a, R> {
    In(&'a Update<R>),
    Out(&'a Update<R>),
}
