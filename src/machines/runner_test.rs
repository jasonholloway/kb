use std::collections::VecDeque;

use crate::machines::{CanEmit, Runnable};

use super::{Runner,Ev};
use super::super::RunRef;
use Ev::*;

#[test]
fn percolates() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("2", TestMachine { count: 2 }),
            RunRef::new("3", TestMachine { count: 3 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev(()));

    assert_eq!(sink.len(), 2 * 3)
}

#[test]
fn accumulates_in_sink_buffer() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("2", TestMachine { count: 2 }),
            RunRef::new("3", TestMachine { count: 3 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev(()));
    runner.run(&mut sink, Ev(()));

    assert_eq!(sink.len(), (2 * 3) + (2 * 3))
}

#[test]
fn empty_set_opaque() {
    let mut runner = Runner::<Ev<(),()>>::new(vec![]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev(()));
    runner.run(&mut sink, Ev(()));
    runner.run(&mut sink, Ev(()));

    assert_eq!(sink.len(), 0)
}

#[test]
fn spawns_machines() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("alf", Redoubler { i: 0, depth: 2 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev::Ev(3));

    assert_eq!(sink.len(), 16)
}

#[test]
fn spawns_machines_2() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("baz", Redoubler { i: 10, depth: 1 }),
            RunRef::new("maz", Redoubler { i: 20, depth: 1 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev::Ev(10));

    assert_eq!(sink.len(), 128)
}

#[test]
fn machines_die_too() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("Morris", Mayfly { life: 3 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev::Ev(1));
    runner.run(&mut sink, Ev::Ev(2));
    runner.run(&mut sink, Ev::Ev(3));
    runner.run(&mut sink, Ev::Ev(4));
    runner.run(&mut sink, Ev::Ev(5));

    dbg!(&sink);

    assert_eq!(sink.len(), 3)
}

struct Mayfly {
    life: u8,
}

impl<TCtx> Runnable<TCtx, Ev<(),u8>> for Mayfly
    where TCtx: CanEmit<Ev<(),u8>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<(),u8>) {
        if self.life > 0 {
            println!("reemitting");
            x.emit(ev);
            self.life -= 1;
        }
        else{
            x.emit(Ev::Die {});
        }
    }
}


struct Redoubler {
    i: u8,
    depth: u8
}

impl<TCtx> Runnable<TCtx,Ev<(),u8>> for Redoubler
    where TCtx: CanEmit<Ev<(),u8>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<(),u8>) {
        println!("D{:?} I{:?} E{:?}", self.depth, self.i, &ev);

        x.emit(ev);
        x.emit(Ev::Ev(self.depth));

        if self.depth > 0 {
            x.emit(Ev::Spawn(RunRef {
                tag: "baz",
                inner: Box::new(Redoubler { i: self.i + 1, depth: self.depth - 1 })
            }))
        }
    }
}



struct TestMachine {
    count: u16,
}

impl<TCtx> Runnable<TCtx,Ev<(),()>> for TestMachine
    where TCtx: CanEmit<Ev<(),()>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<(),()>) {

        for i in 0..self.count {
            dbg!(i);
            x.emit(Ev(()))
        }
    }
}
