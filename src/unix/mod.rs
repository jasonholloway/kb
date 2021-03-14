use crate::common::*;
use std::io::Error;


pub struct UnixKb {

}

impl Keys for UnixKb {
    type TRuntime = UnixRuntime;
    type TRaw = ();

    fn install(&self, handler: Handler<Self::TRaw>) -> Result<Self::TRuntime, Error> {


				let resp = handler(KeyEvent::Down(0, None));

				match resp {
						Response::Skip => {}
						Response::Grab => {}
				}
				
				Ok(UnixRuntime {})
    }
}



pub struct UnixRuntime {
}

impl Runtime<UnixKb> for UnixRuntime {

    fn inject(&self, _ev: KeyEvent<()>) -> () {
        todo!()
    }
}

impl Drop for UnixRuntime {
    fn drop(&mut self) {
    }
}


