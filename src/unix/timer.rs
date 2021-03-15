use libc::{c_int, timeval, itimerval, ITIMER_REAL};
use std::ptr::null_mut;
use std::io::Error;
use std::time::Duration;
use std::convert::TryFrom;

pub struct ITimer {
    which: i32
}

impl Drop for ITimer {
    fn drop(&mut self) {
        unsafe {
            setitimer(self.which, null_mut(), null_mut())
        };
    }
}


pub fn set_itimer(interval: Duration) -> Result<ITimer, Error> {

    let which = ITIMER_REAL;
    let usecs = i64::try_from(interval.as_micros()).unwrap();

    let mut timer = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: usecs
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: usecs
        }
    };
    
    let rc = unsafe {
        setitimer(which, &mut timer, null_mut())
    };

    if rc < 0 {
        Err(Error::last_os_error())
    }
    else {
        Ok(ITimer {
            which: ITIMER_REAL
        })
    }
}


extern "cdecl" {
    fn setitimer(which: c_int,
                  new_value: *mut itimerval,
                  old_value: *mut itimerval) -> c_int;
}
