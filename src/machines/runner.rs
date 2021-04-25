#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use std::collections::VecDeque;

use super::{RunRef, Runnable, Ctx};
use crate::common::{Ev,Ev::*};

pub struct Runner<TEv>
{
    pending: VecDeque<RunRef<TEv>>,
    seen: VecDeque<RunRef<TEv>>,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
    context: Ctx<TEv>
}

impl<TEv> Runner<TEv>
{
    pub fn new(active: Vec<RunRef<TEv>>) -> Runner<TEv> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            context: Ctx::new()        }
    }
}

impl<TRaw> Runnable<Ev<TRaw>> for Runner<Ev<TRaw>>
{
    fn run(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) {
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
                        Spawn(m2) => {
                            pending.push_front(m2);
                        },
                        Die => {
                            requeue = false;
                        },
                        _ => {
                            buff2.push_back(e2);
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
