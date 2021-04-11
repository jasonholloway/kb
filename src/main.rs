#![allow(dead_code)]

extern crate bitmaps;
extern crate typenum;
extern crate velcro;

#[cfg(unix)]
extern crate libc;

use common::*;
use machines::{LookupFac, MachineFac, Runner};
use std::{
    collections::{HashMap},
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

fn print_keys_fac<TRaw>() -> MachineFac<Update<TRaw>>
{
    todo!()
}

fn dynamic_machine_fac<TRaw>() -> MachineFac<Update<TRaw>>
{
    todo!()
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
            "print": print_keys_fac(),
            "blah": dynamic_machine_fac()
        ],
        vec![
            "print", "blah"
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
