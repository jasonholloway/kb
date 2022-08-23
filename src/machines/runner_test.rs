use crate::{common::{RunnerEv, RunnerOut}, machines::{Runnable, Ctx}};
use crate::common::{CoreEv,Movement};
use CoreEv::*;
use Movement::*;

use super::Runner;
use super::super::RunRef;

#[test]
fn percolates() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("2", TestMachine { count: 2 }),
            RunRef::new("3", TestMachine { count: 3 }),
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Tick);

    assert_eq!(sink.buff.len(), 2 * 3)
}

#[test]
fn accumulates_in_sink_buffer() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("2", TestMachine { count: 2 }),
            RunRef::new("3", TestMachine { count: 3 }),
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Tick);
    runner.run(&mut sink, Tick);

    assert_eq!(sink.buff.len(), (2 * 3) + (2 * 3))
}

#[test]
fn empty_set_opaque() {
    let mut runner = Runner::<CoreEv<()>>::new(vec![]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Tick);
    runner.run(&mut sink, Tick);
    runner.run(&mut sink, Tick);

    assert_eq!(sink.buff.len(), 0)
}

#[test]
fn spawns_machines() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("alf", Redoubler { i: 0, depth: 2 }),
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Key(3, Down, None));

    assert_eq!(sink.buff.len(), 16)
}

#[test]
fn spawns_machines_2() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("baz", Redoubler { i: 10, depth: 1 }),
            RunRef::new("maz", Redoubler { i: 20, depth: 1 }),
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Key(10, Down, None));

    assert_eq!(sink.buff.len(), 128)
}

#[test]
fn machines_die_too() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("Morris", Mayfly { life: 3 }),
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Key(1, Down, None));
    runner.run(&mut sink, Key(2, Down, None));
    runner.run(&mut sink, Key(3, Down, None));
    runner.run(&mut sink, Key(4, Down, None));
    runner.run(&mut sink, Key(5, Down, None));

    dbg!(&sink.buff);

    assert_eq!(sink.buff.len(), 3)
}

struct Mayfly {
    life: u8,
}

impl Runnable<(), CoreEv, RunnerOut> for Mayfly
{
    fn run(&mut self, x: &mut Ctx<(), RunnerOut>, ev: ((), CoreEv)) {
        if self.life > 0 {
            println!("reemitting");
            x.emit(ev);
            self.life -= 1;
        }
        else{
            x.emit(RunnerOut::Runner(RunnerEv::Die));
        }
    }
}


struct Redoubler {
    i: u8,
    depth: u16
}

impl Runnable<(), CoreEv, RunnerOut> for Redoubler
{
    fn run(&mut self, x: &mut Ctx<(), RunnerOut>, ev: ((), CoreEv)) {
        println!("D{:?} I{:?} E{:?}", self.depth, self.i, &ev);

        x.emit(ev);
        x.emit(Key(self.depth, Down, None));

        if self.depth > 0 {
            x.emit((None, RunnerOut::Runner(RunnerEv::Spawn("bob"))));

            // RunRef {
            //     tag: "baz",
            //     inner: Box::new(Redoubler { i: self.i + 1, depth: self.depth - 1 })
            // }))
        }
    }
}



struct TestMachine {
    count: u16,
}

impl Runnable<(), CoreEv, RunnerOut> for TestMachine
{
    fn run(&mut self, x: &mut Ctx<(), RunnerOut>, ev: ((), CoreEv)) {

        for i in 0..self.count {
            dbg!(i);
            x.emit(Tick)
        }
    }
}
