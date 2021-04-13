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
    active: Vec<RunRef<TUp>>,
    buff1: VecDeque<Ev<TUp>>,
    buff2: VecDeque<Ev<TUp>>,
    buff3: VecDeque<Ev<TUp>>,
}

impl<T> Runner<T>
{
    pub fn new(active: Vec<RunRef<T>>) -> Runner<T> {
        Runner {
            active,
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

        buff1.push_back(ev);

        for m in self.active.iter_mut() {
            for e in buff1.drain(0..) {

                m.inner.run(e, buff2);

                for e2 in buff2.drain(0..) {

                    match e2 {
                        Ev::Ev(_) => buff3.push_back(e2),
                        Ev::Spawn(_) => (),
                        Ev::Die => ()
                    }
                }
            }

            buff1.extend(buff3.drain(0..));
        }

        sink.extend(buff1.drain(0..));
    }
}
