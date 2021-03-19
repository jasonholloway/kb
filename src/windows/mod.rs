#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::um::winuser::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, WH_KEYBOARD_LL, MSG, WM_QUIT, KBDLLHOOKSTRUCT, HC_ACTION
};

unsafe extern "system" fn key_handler(code: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if code == HC_ACTION {
      let message = wParam as u32;
      let kb = lParam as *const KBDLLHOOKSTRUCT;
      let flags = (*kb).flags;
      let scanCode = (*kb).scanCode;
      let extra = (*kb).dwExtraInfo;

      // if scanCode == 91 {
      //     println!("yum yum!");
      //     return 1;
      // }

      println!("kb {} {} {}", scanCode, flags, extra);
    }

    CallNextHookEx(null_mut(), code, wParam, lParam)
}

use crate::Handler;

pub fn run<TState>(_state: &mut TState, _handler: Handler<TState, ()>) -> Result<(), Error> {
    let _h = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(key_handler), null_mut(), 0) };
    if _h.is_null() {
        return Err(Error::last_os_error());
    }

    unsafe {
        let mut msg: MSG = std::mem::uninitialized();
        let mut i = 0;

        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            print!("message {}", i);
            i += 1;

            if msg.message == WM_QUIT {
                break;
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        print!("done {}", i);

        UnhookWindowsHookEx(_h)
    };

    return Ok(());
}
