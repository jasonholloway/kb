use bitmaps::{Bitmap, Bits};
use typenum::*;

use crate::common::{CoreEv, Movement};

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

    pub fn track_in(&mut self, up: &CoreEv) {
        Self::gather_map(up, &mut self.pre);
    }

    pub fn track_out(&mut self, up: &CoreEv) {
        Self::gather_map(up, &mut self.post);
    }

    fn gather_map<TBits: Bits>(event: &CoreEv, map: &mut Bitmap<TBits>) -> () {
        use Movement::*;

        if let CoreEv::Key(code, movement) = event {
            match movement {
                Up => map.set(*code as usize, false),
                Down => map.set(*code as usize, true),
            };
        }
    }

    
}
