use typenum::*;
use bitmaps::Bitmap;

use super::{Runnable, Ctx};
use crate::common::{Ev,Ev::*};
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

    fn print<TRaw>(&self, bits: &Bitmap<U1024>, ev: &Ev<TRaw>)
    {
        let new_code = if let Key(c, _, _) = ev { *c } else { 0 as u16 };

        print!("{}", (0..self.tabs).map(|_| '\t').collect::<String>());

        print!("[");
        let mut first = true;
        for c in bits.into_iter() {
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

impl<TRaw> Runnable<Ev<TRaw>> for PrintKeys
where
    TRaw: Debug
{
    fn run(&mut self, x: &mut Ctx<Ev<TRaw>>, ev: Ev<TRaw>) {
        let maps = &x.maps;
        
        if let Key(_, _, _) = ev {
            self.print(&maps.pre, &ev);
        }

        x.emit(ev);
    }
}
