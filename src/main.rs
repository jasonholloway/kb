#[cfg(windows)]
extern crate winapi;

mod common;
mod windows;

pub fn main() {
    run(windows::WinKeys { });
}

fn run<K: common::Keys>(keys: K) {
    keys.install().unwrap();
}

