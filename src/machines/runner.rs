#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use std::collections::VecDeque;

use super::{CanEmit, RunRef, Runnable};

#[derive(Debug)]
pub enum Ev<TCtx,TUp> {
    Ev(TUp),
    Spawn(RunRef<RunCtx<Ev<TCtx,TUp>>,Ev<TCtx,TUp>>),
    Die
}






pub struct RunCtx<TEv> {
    buff: VecDeque<TEv>,
}

//below TUp needs purging/replacing with TEv
pub struct Runner<TEv>
{
    pending: VecDeque<RunRef<RunCtx<TEv>,TEv>>,
    seen: VecDeque<RunRef<RunCtx<TEv>,TEv>>,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
    context: RunCtx<TEv>
}

impl<TEv> Runner<TEv>
{
    pub fn new(active: Vec<RunRef<RunCtx<TEv>,TEv>>) -> Runner<TEv> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            context: RunCtx {
                buff: VecDeque::new()
            }
        }
    }
}

impl<TEv> CanEmit<TEv> for RunCtx<TEv> {
    fn emit(&mut self, ev: TEv) {
        self.buff.push_back(ev)
    }

    fn emit_many<T: IntoIterator<Item=TEv>>(&mut self, evs: T) {
        self.buff.extend(evs)
    }
}


impl<TOuterCtx,TCtx,TUp> Runnable<TOuterCtx,Ev<TCtx,TUp>> for Runner<Ev<TCtx,TUp>>
where
    TOuterCtx: CanEmit<Ev<TCtx,TUp>>,
    TUp: std::fmt::Debug,
{
    fn run(&mut self, x: &mut TOuterCtx, ev: Ev<TCtx,TUp>) {
        let mut buff1 = &mut self.buff1;
        let mut buff2 = &mut self.buff2;
        let mut pending = &mut self.pending;
        let mut seen = &mut self.seen;

        if pending.is_empty() {
            return;
        }

        buff1.push_back(ev);

        while let Some(mut m) = pending.pop_front() {

            let mut requeue = true;

            for e1 in buff1.drain(0..) {
                m.inner.run(&mut self.context, e1);

                for e2 in self.context.buff.drain(0..) {
                    match e2 {
                        Ev::Ev(_) => {
                            buff2.push_back(e2);
                        },
                        Ev::Spawn(m2) => {
                            pending.push_front(m2);
                        },
                        Ev::Die => {
                            requeue = false;
                        }
                    }
                }
            }

            buff1.extend(buff2.drain(0..));

            if requeue {
                seen.push_back(m);
            }
        }

        pending.extend(seen.drain(0..));

        x.emit_many(buff1.drain(0..));
    }
}
