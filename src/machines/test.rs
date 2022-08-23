use crate::common::{CoreEv, Movement, Out};
use super::{Runnable, Ctx, machine::Machine, runner::Runner};
use super::super::RunRef;
use Movement::*;
use CoreEv::*;

#[test]
fn masking_obscures() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, (None, Key(101, Down)));

    runner.run(&mut sink, (None, Key(1, Down)));
    runner.run(&mut sink, (None, Key(2, Down)));
    assert!(&sink.buff.contains(&(None, RunnerOut::Core(Key(2, Down))Down)));
    assert_eq!(sink.buff.len(), 1);

    runner.run(&mut sink, (None, Key(2, Up)));
    runner.run(&mut sink, (None, Key(1, Up)));
    assert!(&sink.buff.contains(&(None, Key(2, Up))));
    assert_eq!(sink.buff.len(), 2);
}

#[test]
fn unmasking_reemits() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = Ctx::new();


    runner.run(&mut sink, Key(101, Down, None));

    runner.run(&mut sink, Key(1, Down, None));
    assert_eq!(sink.buff.len(), 0,
            "real key suppressed");

    runner.run(&mut sink, Key(101, Up, None));
    assert!(&sink.buff.contains(&Key(1, Down, None)),
            "on unmask, key finally percolates");


    runner.run(&mut sink, Key(101, Down, None));
    assert!(&sink.buff.contains(&Key(1, Up, None)),
            "on mask, key is released");

    runner.run(&mut sink, Key(1, Up, None));
    assert_eq!(sink.buff.len(), 2,
            "real release suppressed");

    runner.run(&mut sink, Key(101, Up, None));
    assert_eq!(sink.buff.len(), 2,
            "nothing to do as already apparently released");
}


struct Masker {
}

impl Runnable<(), CoreEv, Out> for Masker
{
    fn run(&mut self, x: &mut Ctx<(), Out>, ev: (Option<()>, CoreEv)) {

        match ev {
            Key(i, Down, _) if i > 100 => {
                x.mask(&[i-100]);
            },
            Key(i, Up, _) if i > 100 => {
                x.unmask(&[i-100]);
            },
            _ => {
                x.emit(ev);
            }
        }
    }
}

