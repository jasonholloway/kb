use crate::common::{Keys, Handler, Runtime, KeyEvent};
use std::io::Error;


pub struct NullKb {
}


impl Keys for NullKb {

    type TRuntime = NullRuntime;
    type TRaw = ();
    
    fn install(&self, _: Handler<()>) -> Result<NullRuntime, Error> {
        todo!()
    }
}



pub struct NullRuntime {
}

impl Runtime<NullKb> for NullRuntime {

    fn inject(&self, _ev: KeyEvent<()>) -> () {
        todo!()
    }
}
    
