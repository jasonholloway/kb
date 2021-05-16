#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use std::collections::VecDeque;

use super::{Ctx, RunRef, Runnable};
use crate::common::{CoreEv, MachineOut, RunnerEv, RunnerOut};

pub struct Runner<TRaw>
{
    pending: VecDeque<RunRef<TRaw,CoreEv,MachineOut>>,
    seen: VecDeque<RunRef<TRaw,CoreEv,MachineOut>>,
    buff1: VecDeque<(Option<TRaw>,CoreEv)>,
    buff2: VecDeque<(Option<TRaw>,CoreEv)>,
    context: Ctx<TRaw,MachineOut>
}

impl<TRaw> Runner<TRaw>
{
    pub fn new(active: Vec<RunRef<TRaw,CoreEv,MachineOut>>) -> Runner<TRaw> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            context: Ctx::new()
        }
    }
}

impl<TRaw> Runnable<TRaw,CoreEv,RunnerOut> for Runner<TRaw>
{
    fn run(&mut self, x: &mut Ctx<TRaw,RunnerOut>, inp: (Option<TRaw>,CoreEv))
    {
        let mut buff1 = &mut self.buff1;
        let mut buff2 = &mut self.buff2;
        let mut pending = &mut self.pending;
        let mut seen = &mut self.seen;

        if pending.is_empty() {
            return;
        }

        buff1.push_back(inp);

        while let Some(mut m) = pending.pop_front() {

            let mut requeue = true;

            for e1 in buff1.drain(0..) {
                m.inner.run(&mut self.context, e1);

                for (d, e2) in self.context.buff.drain(0..) {
                    // let e2 = emit;
                    match e2 {
                        MachineOut::Core(ev) => {
                            buff2.push_back((d, ev));
                        },
                        MachineOut::Runner(RunnerEv::Spawn(name)) => {
                            // pending.push_front(name);
                        },
                        MachineOut::Runner(RunnerEv::Die) => {
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

        for (d, ev) in buff1.drain(0..) {
            x.emit((d, RunnerOut::Core(ev)))
        }
    }
}
