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
    let runtime = keys.install(handle).unwrap();

    runtime.inject(KeyEvent::Up(0, None));
}


fn handle<TRaw>(ev: KeyEvent<TRaw>) -> Response {

    match ev {
        KeyEvent::Up(_, _) => {}
            
        KeyEvent::Down(_, _) => {}
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
