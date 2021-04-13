use super::*;
use runner::Runner;

#[test]
fn machines_run_in_sequence() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("1", TestMachine { count: 3 }),
            RunRef::new("2", TestMachine { count: 4 }),
            RunRef::new("3", TestMachine { count: 5 }),
            RunRef::new("4", TestMachine { count: 6 }),
            RunRef::new("5", TestMachine { count: 7 }),
        ]);

    let mut sink = VecDeque::new();

    runner.run((), &mut sink);
    runner.run((), &mut sink);

    assert_eq!(sink.len(), 2 * 3 * 4 * 5 * 6 * 7)
}

#[test]
fn empty_events_passed_through() {
    let mut runner = Runner::new(vec![]);

    let mut sink = VecDeque::new();

    runner.run((), &mut sink);

    assert_eq!(sink.len(), 1)
}

#[test]
fn machines_add_to_working_set() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("alf", Alf {}),
            RunRef::new("baz", Baz {}),
        ]);

    let mut sink = VecDeque::new();

    runner.run(Ev::Ev(1), &mut sink);
    runner.run(Ev::Ev(1), &mut sink);
    runner.run(Ev::Ev(1), &mut sink);

    assert_eq!(sink.len(), 1)
}




struct Alf {
}

impl Runnable<Ev<u8>> for Alf {
    fn run(&mut self, ev: Ev<u8>, sink: &mut Sink<Ev<u8>>) {
        sink.push_back(ev);
        sink.push_back(Ev::Ev(2));
        sink.push_back(Ev::Spawn(RunRef { tag: "baz", inner: Box::new(Baz {}) }))
    }
}


struct Baz {
}

impl Runnable<Ev<u8>> for Baz {
    fn run(&mut self, ev: Ev<u8>, sink: &mut Sink<Ev<u8>>) {
        sink.push_back(ev);
        sink.push_back(Ev::Ev(3));
    }
}



#[derive(Debug)]
enum Ev<T> {
    Ev(T),
    Spawn(RunRef<Ev<T>>),
    Die
}


struct TestMachine {
    count: u16,
}

impl Runnable<()> for TestMachine {
    fn run(&mut self, ev: (), sink: &mut Sink<()>) {
        for i in 0..self.count {
            sink.extend(Some(()));
        }
    }
}
