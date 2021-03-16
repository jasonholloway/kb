use crate::common::*;
use std::{fs::File, io::Error};
use std::time::{SystemTime};
use std::convert::TryFrom;
use std::time::Duration;
use evdev_rs::*;
use evdev_rs::enums::*;
use Response::*;

use self::dev_info::event_info;

mod dev_info;
mod timer;

pub struct UnixKb {

}


enum Mode { Read, Sync }

impl Setup for UnixKb {
    type TRuntime = UnixRuntime;
    type TRaw = ();

    fn install(&self, handler: Handler<Self::TRaw>) -> Result<Self::TRuntime, Error> {

        let mut source = open_device("/dev/input/by-path/platform-i8042-serio-0-event-kbd")
            .unwrap();

        source.grab(GrabMode::Grab).unwrap();
        // dev_info::dev_info(&source);
        
        let sink = UInputDevice::create_from_device(&source).unwrap();

        timer::catch_alrm().unwrap();
        let _timer = timer::set_itimer(Duration::from_millis(40)).unwrap();

        let mut mode: Mode = Mode::Read;
        
        loop {
            let res = match mode {
                Mode::Read => source
                    .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
                    .map(|(status, ev)| {
                        match status {
                            ReadStatus::Success => {
                                println!("C: {}", unsafe { timer::ALRMS });
                                
                                match ev.event_code {
                                    EventCode::EV_MSC(EV_MSC::MSC_SCAN) => Mode::Read,

                                    EventCode::EV_KEY(_) => {
                                        event_info(&ev);

                                        sink.write_event(&ev).unwrap();

                                        sink.write_event(&InputEvent {
                                            time: TimeVal::try_from(SystemTime::now()).unwrap(),
                                            event_type: EventType::EV_SYN,
                                            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                                            value: 0
                                        }).unwrap(); // 

                                        // std::thread::sleep(std::time::Duration::from_millis(20));

                                        Mode::Read
                                    }, 

                                    _ => {
                                        sink.write_event(&ev).unwrap();
                                        Mode::Read
                                    }
                                }
                            }

                            ReadStatus::Sync => Mode::Sync
                        }
                    }),

                Mode::Sync => source
                    .next_event(ReadFlag::SYNC)
                    .map(|(status, _)| {
                        println!("SYNC!");
                        match status {
                            ReadStatus::Sync => Mode::Sync,

                            _ => Mode::Read
                        }
                    })
              };
        
            match res {
                Result::Err(err) => {

                    match err.raw_os_error() {
                        Some(libc::EAGAIN) => {
                            continue
                        },

                        Some(libc::EINTR) => {
                            //should process pending jobs here
                            continue;
                        },
                        
                        _ => {
                            println!("ERROR! {}", err);
                            break;
                        }
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

pub fn open_device(path: &str) -> Result<Device, Error>
{
    return Device::new_from_fd(File::open(&path).unwrap());
}
