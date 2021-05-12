use crate::common::{MachineEv, Ev, Ev::*, Mode, Movement::*};
use super::{Runnable, Ctx, Sink};

pub struct Machine<TRaw,TBody> {
    pub context: Ctx<TRaw,MachineEv>,
    pub body: TBody,
    pub mode: Mode
}

impl<TRaw,TBody> Machine<TRaw,TBody>
where
    TRaw: std::fmt::Debug,
    TBody: Runnable<TRaw,Ev,MachineEv>
{
    pub fn new(body: TBody) -> Machine<TRaw,TBody> {
        Machine {
            context: Ctx::new(),
            body,
            mode: Mode::Root
        }
    }

    fn run_handle(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: Ev) {
        self.body.run(&mut self.context, ev);

        //there's a bug here in that all passed-through evs will be handled the same as freshly-minted ones
        use crate::common::MachineEv::*;

        while let Some(emit) = self.context.buff.pop_front() {
            match &emit {
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

                Ev(ev2) => {
                    self.context.maps.track_out(&ev2);
                    x.emit(*ev2);
                },

                PassThru(ev2) => {
                    self.context.maps.track_out(&ev2);
                    x.emit(*ev2);
                } ,
                Now(_) => {},
                Die => {} //to pass back out to runner?
            }
        }

    }
}

impl<TRaw,TBody> Runnable<TRaw,Ev,MachineEv> for Machine<TRaw,TBody>
{
    fn run(&mut self, x: &mut Ctx<TRaw,MachineEv>, ev: (Option<TRaw>,Ev)) -> () {

        self.context.maps.track_in(&ev);

        if let Key(c, _, _) = ev {
            if self.context.maps.mask.get(c.into()) {
                return;
            }
        }

        self.run_handle(x, ev)
    }
}
