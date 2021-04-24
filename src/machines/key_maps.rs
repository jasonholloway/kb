use bitmaps::{Bitmap, Bits};
use typenum::*;

use crate::common::Update;
use crate::common::Movement;

pub struct KeyMaps {
    pub pre: Bitmap<U1024>,
    pub post: Bitmap<U1024>,
    pub mask: Bitmap<U1024>,
}

impl KeyMaps {
    pub fn new() -> KeyMaps {
        KeyMaps {
            pre: Bitmap::new(),
            post: Bitmap::new(),
            mask: Bitmap::new(),
        }
    }

    pub fn track_in<TRaw>(&mut self, up: &Update<TRaw>) {
        Self::gather_map(up, &mut self.pre);
    }

    pub fn track_out<TRaw>(&mut self, up: &Update<TRaw>) {
        Self::gather_map(up, &mut self.post);
    }

    fn gather_map<T, T2: Bits>(event: &Update<T>, map: &mut Bitmap<T2>) -> () {
        use Movement::*;
        use Update::*;

        if let Key(code, movement, _) = event {
            match movement {
                Up => map.set(*code as usize, false),
                Down => map.set(*code as usize, true),
            };
        }
    }

    
}
