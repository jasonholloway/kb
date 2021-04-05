use bitmaps::Bitmap;
use typenum::*;

pub struct KeyMaps {
    pub inp: Bitmap<U1024>,
    pub outp: Bitmap<U1024>,
    pub mask: Bitmap<U1024>,
}

impl KeyMaps {
    pub fn new() -> KeyMaps {
        KeyMaps {
            inp: Bitmap::new(),
            outp: Bitmap::new(),
            mask: Bitmap::new(),
        }
    }
}
