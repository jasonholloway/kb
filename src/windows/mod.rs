#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM, UINT};
use winapi::um::winuser::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, WH_KEYBOARD_LL, MSG, WM_QUIT, KBDLLHOOKSTRUCT, HC_ACTION, WM_KEYDOWN, WM_KEYUP
};
use std::cell::RefCell;
use crate::Handler;
use crate::{Update::*,Movement::*};

type KeyTup = (i32, WPARAM, LPARAM);

unsafe extern "system" fn key_hook(code: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(null_mut(), code, wParam, lParam);
    }
    
    HANDLER.with(|h| {
        if let Some(handler) = (*h).borrow_mut().as_mut() {
            let kb = lParam as *const KBDLLHOOKSTRUCT;
            let flags = (*kb).flags;
            let scanCode = (*kb).scanCode;
            let extra = (*kb).dwExtraInfo;

            // println!("kb {} {} {}", scanCode, flags, extra);

            let upDown = if flags & 0x80 == 0x80 { Up } else { Down };
            let key = Key(scanCode as u16, upDown, Some((code, wParam, lParam)));

            let (_next, drain) = handler.handle(key);
        }
    });

    return CallNextHookEx(null_mut(), code, wParam, lParam);
}

thread_local! {
    static HANDLER: RefCell<Option<Handler<KeyTup>>> = RefCell::new(None);
}

pub fn run<'a>(create_handler: fn() -> Handler<KeyTup>) -> Result<(), Error> {
    HANDLER.with(|h| {
        *h.borrow_mut() = Some(create_handler());
    });

    let _h = unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(key_hook), null_mut(), 0) };
    if _h.is_null() {
        return Err(Error::last_os_error());
    }

    unsafe {
        let mut msg: MSG = std::mem::MaybeUninit::uninit().assume_init();
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
