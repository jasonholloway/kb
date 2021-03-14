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
				let d = evdev_rs::Device::new_from_fd(file).unwrap();
				dev_info::dev_info(&d);


				let mut mode: Mode = Mode::Read;
				
				loop {
						match mode {
								Mode::Read => {
										match d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING) {
												Result::Ok((status, ev)) => {
														match status {
																ReadStatus::Success => {
																		if let EventType::EV_KEY = ev.event_type {
																				event_info(&ev)
																		}
																		
																		mode = Mode::Read;
																}
																ReadStatus::Sync => {
																		mode = Mode::Sync;
																}
														}
												}

												Result::Err(err) => {
														match err.raw_os_error() {
																Some(libc::EAGAIN) => { continue }
																_ => { break }
														}
												}
										};
								}

								Mode::Sync => {
										match d.next_event(ReadFlag::SYNC) {
												Result::Ok((status, _)) => {
														match status {
																ReadStatus::Sync => {}
																_ => {
																		mode = Mode::Read;
																}
														}
												}

												Result::Err(err) => {
														match err.raw_os_error() {
																Some(libc::EAGAIN) => { continue }
																_ => { break }
														}
												}
										}
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
