use std::collections::VecDeque;

use super::{RunRef, Runnable, Sink};

#[derive(Debug)]
pub enum Ev<TUp> {
    Ev(TUp),
    Spawn(RunRef<TUp>),
    Die
}

pub struct Runner<TUp>
{
    pending: VecDeque<RunRef<TUp>>,
    seen: VecDeque<RunRef<TUp>>,
    buff1: VecDeque<Ev<TUp>>,
    buff2: VecDeque<Ev<TUp>>,
    buff3: VecDeque<Ev<TUp>>,
}

impl<T> Runner<T>
{
    pub fn new(active: Vec<RunRef<T>>) -> Runner<T> {
        Runner {
            pending: VecDeque::from(active),
            seen: VecDeque::new(),
            buff1: VecDeque::new(),
            buff2: VecDeque::new(),
            buff3: VecDeque::new(),
        }
    }
}

impl<TUp> Runnable<TUp> for Runner<TUp>
where
    TUp: std::fmt::Debug,
{
    fn run(&mut self, ev: Ev<TUp>, sink: &mut Sink<Ev<TUp>>) {
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
                m.inner.run(e1, buff2);

                for e2 in buff2.drain(0..) {
                    match e2 {
                        Ev::Ev(_) => {
                            buff3.push_back(e2);
                        },
                        Ev::Spawn(m2) => {
                            pending.push_front(m2)
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
