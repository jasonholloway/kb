use super::*;
use runner::{Runner,Ev};

#[test]
fn percolates() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("2", TestMachine { count: 2 }),
            RunRef::new("3", TestMachine { count: 3 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(Ev(()), &mut sink);

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

    runner.run(Ev(()), &mut sink);
    runner.run(Ev(()), &mut sink);

    assert_eq!(sink.len(), (2 * 3) + (2 * 3))
}

#[test]
fn empty_set_opaque() {
    let mut runner = Runner::new(vec![]);

    let mut sink = VecDeque::new();

    runner.run(Ev(()), &mut sink);
    runner.run(Ev(()), &mut sink);
    runner.run(Ev(()), &mut sink);

    assert_eq!(sink.len(), 0)
}

#[test]
fn spawns_machines() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("alf", Redoubler { i: 0, depth: 2 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(Ev::Ev(3), &mut sink);

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

    runner.run(Ev::Ev(10), &mut sink);

    assert_eq!(sink.len(), 128)
}

#[test]
fn machines_die_too() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("Morris", Mayfly { life: 3 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run(Ev::Ev(1), &mut sink);
    runner.run(Ev::Ev(2), &mut sink);
    runner.run(Ev::Ev(3), &mut sink);
    runner.run(Ev::Ev(4), &mut sink);
    runner.run(Ev::Ev(5), &mut sink);

    dbg!(&sink);

    assert_eq!(sink.len(), 3)
}

struct Mayfly {
    life: u8,
}

impl Runnable<u8> for Mayfly {
    fn run(&mut self, ev: Ev<u8>, sink: &mut Sink<Ev<u8>>) {
        if self.life > 0 {
            println!("reemitting");
            sink.push_back(ev);
            self.life -= 1;
        }
        else{
            sink.push_back(Ev::Die {});
        }
    }
}


struct Redoubler {
    i: u8,
    depth: u8
}

impl Runnable<u8> for Redoubler {
    fn run(&mut self, ev: Ev<u8>, sink: &mut Sink<Ev<u8>>) {
        println!("D{:?} I{:?} E{:?}", self.depth, self.i, &ev);

        sink.push_back(ev);
        sink.push_back(Ev::Ev(self.depth));

        if self.depth > 0 {
          sink.push_back(Ev::Spawn(RunRef { tag: "baz", inner: Box::new(Redoubler { i: self.i + 1, depth: self.depth - 1 }) }))
        }
    }
}



struct TestMachine {
    count: u16,
}

impl Runnable<()> for TestMachine {
    fn run(&mut self, ev: Ev<()>, sink: &mut Sink<Ev<()>>) {

        for i in 0..self.count {
            dbg!(i);
            sink.push_back(Ev(()));
        }
    }
}
