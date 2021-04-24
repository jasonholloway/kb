#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use std::collections::VecDeque;

use super::{RunRef, Runnable, Sink};

#[derive(Debug)]
pub enum Ev<TCtx,TUp> {
    Ev(TUp),
    Spawn(RunRef<(),Ev<TCtx,TUp>>),
    Die
}

//below TUp needs purging/replacing with TEv
pub struct Runner<TEv>
{
    pending: VecDeque<RunRef<(),TEv>>,
    seen: VecDeque<RunRef<(),TEv>>,
    buff1: VecDeque<TEv>,
    buff2: VecDeque<TEv>,
    buff3: VecDeque<TEv>,
}

impl<TEv> Runner<TEv>
{
    pub fn new(active: Vec<RunRef<(),TEv>>) -> Runner<TEv> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            buff3: VecDeque::new(),
        }
    }
}

impl<TCtx,TUp> Runnable<(),Ev<TCtx,TUp>> for Runner<Ev<TCtx,TUp>>
where
    TUp: std::fmt::Debug,
{
    fn run(&mut self, x: &mut (), ev: Ev<TCtx,TUp>, sink: &mut Sink<Ev<TCtx,TUp>>) {
        let mut buff1 = &mut self.buff1;
        let mut buff2 = &mut self.buff2;
        let mut buff3 = &mut self.buff3;

        let mut pending = &mut self.pending;
        let mut seen = &mut self.seen;

        if pending.is_empty() {
            return;
        }

        buff1.push_back(ev);

        while let Some(mut m) = pending.pop_front() {

            let mut requeue = true;

            for e1 in buff1.drain(0..) {
                m.inner.run(x, e1, buff2);

                for e2 in buff2.drain(0..) {
                    match e2 {
                        Ev::Ev(_) => {
                            buff3.push_back(e2);
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

            buff1.extend(buff3.drain(0..));

            if requeue {
                seen.push_back(m);
            }
        }

        pending.extend(seen.drain(0..));
        sink.extend(buff1.drain(0..));
    }
}
