use bitmaps::Bitmap;
use std::fmt::Debug;
use typenum::*;
use super::{Machine, gather_map};
use crate::{Update::*, common::Update, sink::Sink};


pub struct PrintKeys {
    out_map: Bitmap<U1024>,
    tabs: u8,
    colour: u8
}

impl PrintKeys {
    pub fn new(tabs: u8, colour: u8) -> PrintKeys {
        PrintKeys {
            out_map: Bitmap::new(),
            tabs,
            colour
        }
    }

    fn print<TRaw>(&self, ev: &Update<TRaw>) {
        
        let new_code = if let Key(c, _, _) = ev { *c } else { 0 as u16 };

        print!("{}", (0..self.tabs).map(|_| '\t').collect::<String>());
        
        print!("[");
        let mut first = true;
        for c in self.out_map.into_iter() {
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

impl<TRaw, TSink> Machine<Update<TRaw>, TSink>
		for PrintKeys
where
    TRaw: Debug,
    TSink: Sink<Update<TRaw>> {

    fn run(&mut self, ev: Update<TRaw>, sink: &mut TSink) -> () {
        gather_map(&ev, &mut self.out_map);

        if let Key(_, _, _) = ev {
            self.print(&ev);
        }

        sink.emit(ev);
    }
}

