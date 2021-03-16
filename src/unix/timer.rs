use libc::{c_int, c_void, timeval, itimerval, ITIMER_REAL, sighandler_t};
use std::ptr::null_mut;
use std::io::Error;
use std::time::Duration;
use std::convert::TryFrom;
use std::mem::MaybeUninit;

pub static mut SIGALRM_COUNT: i64 = 0;

extern fn handler(_: c_int) {
    unsafe { SIGALRM_COUNT += 1 };
}
    

pub fn catch_alrm() -> Result<(), Error> {
    let rc = unsafe {
        let spec: libc::sigaction = libc::sigaction {
            sa_sigaction: handler as *mut c_void as sighandler_t,
            sa_mask: MaybeUninit::zeroed().assume_init(),
            sa_flags: 0,
            sa_restorer: None
        };

        libc::sigaction(libc::SIGALRM, &spec, null_mut())
    };

    if rc < 0 {
        Err(Error::last_os_error())
    }
    else {
        Ok(())
    }
}


pub fn set_itimer(interval: Duration) -> Result<Timer, Error> {

    let which = ITIMER_REAL;
    let usecs = i64::try_from(interval.as_micros()).unwrap();

    let timer = itimerval {
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
        setitimer(which, &timer, null_mut())
    };

    if rc < 0 {
        Err(Error::last_os_error())
    }
    else {
        Ok(Timer {
            which
        })
    }
}

extern "C" {
    fn setitimer(which: c_int,
                  new_value: *const itimerval,
                  old_value: *mut itimerval) -> c_int;
}

pub struct Timer {
    which: i32
}

impl Drop for Timer {
    fn drop(&mut self) {
        unsafe {
            setitimer(self.which, null_mut(), null_mut())
        };
    }
}
