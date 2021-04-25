use crate::common::Ev;
use super::{Runnable, Ctx};
use Ev::*;

pub struct Machine<TRaw,TBody> {
    pub context: Ctx<Ev<TRaw>>,
    pub body: TBody
}

impl<TRaw, TBody> Runnable<Ev<TRaw>> for Machine<TRaw,TBody>
where
    TBody: Runnable<Ev<TRaw>>
{
    fn run(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) -> () {
        use crate::common::Movement::*;

        self.context.maps.track_in(&ev);
        
        self.body.run(&mut self.context, ev);

        //todo
        //emits following mask ops should be played through local machine
        //though this again would require secondary buffer

        while let Some(ev2) = self.context.buff.pop_front() {
            match &ev2 {
                MaskOn(c) => {
                    let is_maskable = !self.context.maps.mask.set(*c as usize, true);

                    if is_maskable && self.context.maps.post.get(*c as usize) {
                        x.emit(Key(*c, Up, None));
                    }
                },
                MaskOff(c) => {
                    let unmaskable = self.context.maps.mask.set(*c as usize, false);

                    if unmaskable && self.context.maps.pre.get(*c as usize) && !self.context.maps.post.get(*c as usize)
                    {
                        x.emit(Key(*c, Down, None));
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

impl<TRaw,TBody> Machine<TRaw,TBody> {
    pub fn new(body: TBody) -> Machine<TRaw,TBody> {
        Machine {
            context: Ctx::new(),
            body
        }
    }
}

