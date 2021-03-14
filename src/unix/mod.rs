use crate::common::*;
use std::{fs::File, io::Error};
use evdev_rs::*;
use evdev_rs::enums::*;
use Response::*;

use self::dev_info::event_info;

mod dev_info;


pub struct UnixKb {

}


enum Mode { Read, Sync }

impl Setup for UnixKb {
    type TRuntime = UnixRuntime;
    type TRaw = ();

    fn install(&self, handler: Handler<Self::TRaw>) -> Result<Self::TRuntime, Error> {

				let file = File::open("/dev/input/by-path/platform-i8042-serio-0-event-kbd").unwrap();
				let source = evdev_rs::Device::new_from_fd(file).unwrap();
				dev_info::dev_info(&source);

				let _sink = evdev_rs::UInputDevice::create_from_device(&source).unwrap();

				let mut mode: Mode = Mode::Read;
				
				loop {
						let res = match mode {
								Mode::Read => source
										.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
										.map(|(status, ev)| {
												match status {
														ReadStatus::Success => {
																if let EventType::EV_KEY = ev.event_type {
																		event_info(&ev);
																}
																Mode::Read
														}

														ReadStatus::Sync => Mode::Sync
												}
										}),

								Mode::Sync => source
										.next_event(ReadFlag::SYNC)
										.map(|(status, _)| {
												match status {
														ReadStatus::Sync => Mode::Sync,

														_ => Mode::Read
												}
										})
							};
				
						match res {
								Result::Err(err) => {
										match err.raw_os_error() {
												Some(libc::EAGAIN) => continue,
												_ => break
										}
								}

								Result::Ok(next) => {
										mode = next;
										continue;
								}
						}
				}
				

				let resp = handler(KeyEvent::Down(0, None));

				match resp {
						Skip => {}
						Grab => {}
				}
				
				Ok(UnixRuntime {})
    }
}



pub struct UnixRuntime {
}

impl Runtime<UnixKb> for UnixRuntime {

    fn inject(&self, _ev: KeyEvent<()>) -> () {
    }
}

impl Drop for UnixRuntime {
    fn drop(&mut self) {
    }
}
