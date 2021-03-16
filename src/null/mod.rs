use crate::common::{Setup, Handler, Runtime, KeyEvent};
use std::io::Error;


pub struct NullKb {
}


impl Setup for NullKb {

    type TRuntime = NullRuntime;
    type TRaw = ();
    
    fn install<TState>(&self, state: TState,_handler: Handler<TState, ()>) -> Result<NullRuntime, Error> {
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
    
