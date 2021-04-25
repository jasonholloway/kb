use crate::common::{Ev,Movement};
use super::{Runnable, Ctx, machine::Machine, runner::Runner};
use super::super::RunRef;
use Ev::*;
use Movement::*;

#[test]
fn masking() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = Ctx::new();

    runner.run(&mut sink, Key(101, Down, None));

    runner.run(&mut sink, Key(1, Down, None));
    runner.run(&mut sink, Key(2, Down, None));
    assert_eq!(sink.buff.len(), 1);

    runner.run(&mut sink, Key(2, Up, None));
    runner.run(&mut sink, Key(1, Up, None));
    assert_eq!(sink.buff.len(), 2);

    runner.run(&mut sink, Key(101, Up, None));

    runner.run(&mut sink, Key(1, Down, None));
    runner.run(&mut sink, Key(2, Down, None));
    assert_eq!(sink.buff.len(), 4);
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

