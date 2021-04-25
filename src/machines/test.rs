use std::collections::VecDeque;
use crate::common::{Update,Movement};
use super::{CanEmit, CanMask, Runnable, machine::Machine, runner::Runner};
use super::super::{RunRef,Ev};
use Ev::*;
use Update::*;
use Movement::*;

#[test]
fn masking() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut sink, Ev(Key(101, Down, None)));

    runner.run(&mut sink, Ev(Key(1, Down, None)));
    runner.run(&mut sink, Ev(Key(2, Down, None)));
    assert_eq!(sink.len(), 1);

    runner.run(&mut sink, Ev(Key(2, Up, None)));
    runner.run(&mut sink, Ev(Key(1, Up, None)));
    assert_eq!(sink.len(), 2);

    runner.run(&mut sink, Ev(Key(101, Up, None)));

    runner.run(&mut sink, Ev(Key(1, Down, None)));
    runner.run(&mut sink, Ev(Key(2, Down, None)));
    assert_eq!(sink.len(), 4);
}


struct Masker {
}

impl<TCtx> Runnable<TCtx,Ev<TCtx,Update<()>>> for Masker
    where TCtx: CanEmit<Ev<TCtx,Update<()>>> + CanMask<Ev<TCtx,Update<()>>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<TCtx,Update<()>>) {

        match ev {
            Ev(Key(i, Down, _)) if i > 100 => {
                x.mask(&[i-100]);
            },
            Ev(Key(i, Up, _)) if i > 100 => {
                x.unmask(&[i-100]);
            },
            _ => {
                x.emit(ev);
            }
        }
    }
}


impl<TEv> CanEmit<TEv> for VecDeque<TEv> {
    fn emit(&mut self, ev: TEv) {
        self.push_back(ev)
    }

    fn emit_many<T: IntoIterator<Item=TEv>>(&mut self, evs: T) {
        self.extend(evs)
    }
}
