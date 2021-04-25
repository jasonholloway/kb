use crate::common::{Ev,Ev::*,Movement::*};
use super::{Runnable, Ctx};

pub struct Machine<TRaw,TBody> {
    pub context: Ctx<Ev<TRaw>>,
    pub body: TBody
}

impl<TRaw,TBody> Machine<TRaw,TBody>
where
    TRaw: std::fmt::Debug,
    TBody: Runnable<Ev<TRaw>>
{
    pub fn new(body: TBody) -> Machine<TRaw,TBody> {
        Machine {
            context: Ctx::new(),
            body
        }
    }

    fn run_handle(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) {
        self.body.run(&mut self.context, ev);

        //there's a bug here in that all passed-through evs will be handled  the same...

        while let Some(ev2) = self.context.buff.pop_front() {
            match &ev2 {
                MaskOn(c) => {
                    let is_maskable = !self.context.maps.mask.set(*c as usize, true);

                    if is_maskable && self.context.maps.post.get(*c as usize) {
                        self.run_handle(x, Key(*c, Up, None));
                    }
                },
                MaskOff(c) => {
                    let unmaskable = self.context.maps.mask.set(*c as usize, false);

                    if unmaskable {
                        match (self.context.maps.pre.get(*c as usize), self.context.maps.post.get(*c as usize)) {
                            (true, false) => {
                                self.run_handle(x, Key(*c, Down, None));
                            },
                            (false, true) => {
                                self.run_handle(x, Key(*c, Up, None));
                            },
                            _ => {}
                        }
                    }
                },
                _ => {
                    self.context.maps.track_out(&ev2);
                    x.emit(ev2);
                }
            }
        }

    }
}

impl<TRaw: std::fmt::Debug, TBody> Runnable<Ev<TRaw>> for Machine<TRaw,TBody>
where
    TBody: Runnable<Ev<TRaw>>
{
    fn run(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) -> () {

        self.context.maps.track_in(&ev);

        if let Key(c, _, _) = ev {
            if self.context.maps.mask.get(c.into()) {
                return;
            }
        }

        self.run_handle(x, ev)
    }
}
