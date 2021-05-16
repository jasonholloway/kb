use crate::common::{CoreEv, Out, MachineEv, MachineOut, Mode, Movement::*};
use super::{Runnable, Ctx};

pub struct Machine<TRaw,TBody> {
    pub context: Ctx<TRaw,Out>,
    pub body: TBody,
    pub mode: Mode
}

impl<TRaw,TBody> Machine<TRaw,TBody>
where
    TBody: Runnable<TRaw,CoreEv,Out>
{
    pub fn new(body: TBody) -> Machine<TRaw,TBody> {
        Machine {
            context: Ctx::new(),
            body,
            mode: Mode::Root
        }
    }

    fn run_handle(&mut self, inp: (Option<TRaw>,CoreEv)) {
        {
            self.body.run(&mut self.context, inp);
        }

        //TODO
        //there's a bug here in that all passed-through evs will be handled the same as freshly-minted ones
        let x = &mut self.context;

        use CoreEv::*;
        use MachineEv::*;
        use Out::*;

        while let Some((d, ev)) = x.buff.pop_front() {
            match &ev {
                Machine(MaskOn(c)) => {
                    let is_maskable = !x.maps.mask.set(*c as usize, true);

                    if is_maskable && x.maps.post.get(*c as usize) {
                        let ev2 = Key(*c, Up);
                        x.maps.track_out(&ev2);
                        x.emit((None, Core(ev2)));
                    }
                },

                Machine(MaskOff(c)) => {
                    let unmaskable = x.maps.mask.set(*c as usize, false);

                    if unmaskable {
                        match (x.maps.pre.get(*c as usize), x.maps.post.get(*c as usize)) {
                            (true, false) => {
                                let ev2 = Key(*c, Down);
                                x.maps.track_out(&ev2);
                                x.emit((None, Core(ev2)));
                            },
                            (false, true) => {
                                let ev2 = Key(*c, Up);
                                x.maps.track_out(&ev2);
                                x.emit((None, Core(ev2)));
                            },
                            _ => {}
                        }
                    }
                },

                Machine(Now(_)) => {},

                Core(peek) => {
                    x.maps.track_out(&peek);
                    x.emit((d, ev));
                },

                Runner(_) => {
                    x.emit((d, ev));
                }
            }
        }

    }
}

impl<TRaw,TBody> Runnable<TRaw,CoreEv,MachineOut> for Machine<TRaw,TBody>
    where TBody: Runnable<TRaw,CoreEv,Out>
{
    fn run(&mut self, x: &mut Ctx<TRaw,MachineOut>, (d, ev): (Option<TRaw>,CoreEv)) -> ()
    {
        self.context.maps.track_in(&ev);

        if let CoreEv::Key(c, _) = ev {
            if self.context.maps.mask.get(c.into()) {
                return;
            }
        }

        self.run_handle((d, ev))
    }
}
