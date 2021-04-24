use std::collections::VecDeque;

use crate::common::{Update,Movement};

use super::{CanEmit, CanMask, Runnable, Sink, machine::Machine, runner::Runner};
use super::super::{RunRef,Ev};
use Ev::*;
use Update::*;
use Movement::*;

#[test]
fn stuff() {
    let mut runner = Runner::new(
        vec![
            RunRef::new("m1", Machine::new(Masker {}))
        ]);

    let mut sink = VecDeque::new();

    runner.run(&mut (), Ev(Key(1, Down, None)), &mut sink);
    runner.run(&mut (), Ev(Key(1, Up, None)), &mut sink);

    assert_eq!(sink.len(), 2 * 3)
}


struct Masker {
}

impl<TCtx> Runnable<TCtx,Ev<TCtx,Update<()>>> for Masker
    where TCtx: CanEmit<Ev<TCtx,Update<()>>> + CanMask<Ev<TCtx,Update<()>>>
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<TCtx,Update<()>>, sink: &mut Sink<Ev<TCtx,Update<()>>>) {

        x.mask(&[1], sink);

        x.emit(ev, sink);

        x.unmask(&[1], sink);
    }
}

