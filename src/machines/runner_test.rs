use super::*;
use crate::fac;
use runner::Runner;
use velcro::hash_map;

struct TestMachine {
    count: u16,
}

impl Runnable<()> for TestMachine {
    fn run(&mut self, ev: (), sink: &mut Sink<()>) -> () {
        for i in 0..self.count {
            sink.extend(Some(()));
        }
    }
}

#[test]
fn machines_run_in_sequence() {
    let mut runner = Runner::new(
        hash_map![
            "1": fac(|| TestMachine { count: 3 }),
            "2": fac(|| TestMachine { count: 4 }),
            "3": fac(|| TestMachine { count: 5 }),
            "4": fac(|| TestMachine { count: 6 }),
            "5": fac(|| TestMachine { count: 7 }),
        ],
        vec!["1", "2", "3", "4", "5"]
    );

    let mut sink = VecDeque::new();
    runner.run((), &mut sink);
    runner.run((), &mut sink);

    assert_eq!(sink.len(), 2 * 3 * 4 * 5 * 6 * 7)
}
