extern crate libc;

use common::*;


#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;


mod common;
mod null;


pub fn main() {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
          run(windows::WinKb { })
        } else if #[cfg(unix)] {
          run(unix::UnixKb { })
        } else {
          run(null::NullKb { })
        }
    }
}

fn run<K: Setup>(keys: K) {
    let runtime = keys.install(State {}, handle).unwrap();

    runtime.inject(KeyEvent::Up(0, None));
}



struct State {}


fn handle<TRaw>(state: State, update: Update<TRaw>) -> Response {
    use common::{Update::*,KeyEvent::*};
    
    match update {
        Key(Up(_, _)) => {}
            
        Key(Down(_, _)) => {}

        Tick => {}
    }
    
    Response::Skip
}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_something() {

        let kb = (null::NullKb {});
        let rt = kb.install(|ev| Response::Skip).unwrap();

        

        
        assert_eq!(2+2, 5);
    }

}
