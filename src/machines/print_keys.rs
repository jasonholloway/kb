use super::{CanEmit, HasMaps, Runnable, Sink, runner::{Ev,Ev::*}};
use crate::{common::Update, Update::*};
use std::fmt::Debug;


pub struct PrintKeys {
    tabs: u8,
    colour: u8,
}

impl PrintKeys {
    pub fn new(tabs: u8, colour: u8) -> PrintKeys {
        PrintKeys {
            tabs,
            colour
        }
    }

    fn print<TCtx, TRaw>(&self, x: &mut TCtx, ev: &Update<TRaw>)
        where TCtx: HasMaps
    {
        let new_code = if let Key(c, _, _) = ev { *c } else { 0 as u16 };

        print!("{}", (0..self.tabs).map(|_| '\t').collect::<String>());

        print!("[");
        let mut first = true;
        for c in x.maps().outp.into_iter() {
            if !first {
                print!(", ");
            }

            if c == new_code as usize {
                print!("\x1b[0;{:?}m{:?}\x1b[0m", self.colour, c);
            } else {
                print!("{:?}", c);
            }

            first = false;
        }
        print!("]\t\t");
        println!();
    }
}

impl<TCtx, TRaw> Runnable<TCtx, Ev<TCtx, Update<TRaw>>> for PrintKeys
where
    TCtx: CanEmit<Ev<TCtx,Update<TRaw>>> + HasMaps,
    TRaw: Debug
{
    fn run(&mut self, x: &mut TCtx, ev: Ev<TCtx,Update<TRaw>>, sink: &mut Sink<Ev<TCtx,Update<TRaw>>>) {
        match ev {
            Ev(up) => {
                if let Key(_, _, _) = up {
                    self.print(x, &up);
                }

                x.emit(Ev(up), sink);
            }
            _ => ()
        }
        
    }
}
