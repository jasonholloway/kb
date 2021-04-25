use crate::common::{Ev,Movement};
use super::{Runnable, Ctx, machine::Machine, runner::Runner};
use super::super::RunRef;
use Ev::*;
use Movement::*;

#[test]
fn masking_obscures() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Key(101, Down, None));

    runner.run(&mut sink, Key(1, Down, None));
    runner.run(&mut sink, Key(2, Down, None));
    assert!(&sink.buff.contains(&Key(2, Down, None)));
    assert_eq!(sink.buff.len(), 1);

    runner.run(&mut sink, Key(2, Up, None));
    runner.run(&mut sink, Key(1, Up, None));
    assert!(&sink.buff.contains(&Key(2, Up, None)));
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

impl Runnable<Ev<()>> for Masker
{
    fn run(&mut self, x: &mut Ctx<Ev<()>>, ev: Ev<()>) {

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

