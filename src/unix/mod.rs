use std::{collections::VecDeque, fs::File, io::{Error,ErrorKind::*}};
use std::time::SystemTime;
use std::convert::TryFrom;
use std::time::Duration;
use evdev_rs::*;
use evdev_rs::enums::*;
use crate::{Movement::*, Update::*, common::Update, machines::Runnable};

mod dev_info;
mod timer;

enum Mode { Read, Sync }

const DEV_PATH: &str = "/dev/input/event18";
// "/dev/input/by-path/platform-i8042-serio-0-event-kbd"

pub fn run<'a, TRun: Runnable<Update<InputEvent>>>(runnable: &mut TRun) -> Result<(), Error>
{
    let mut buff = VecDeque::new();
    
    let mut source = open_device(DEV_PATH)
        .unwrap();

    source.grab(GrabMode::Grab).unwrap();

    let sink = UInputDevice::create_from_device(&source).unwrap();

    timer::catch_alrm().unwrap();
    let _timer = timer::set_itimer(Duration::from_millis(40)).unwrap();

    let mut mode: Mode = Mode::Read;

    loop {
        let res = match mode {
            Mode::Read => source
                .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
                .and_then(|(status, ev)| {
                    //event_info(&ev);
                    match status {
                        ReadStatus::Success => {
                            match ev.event_code {

                                EventCode::EV_KEY(_) => {

                                    let code = ev.as_raw().code;

                                    let update = match ev.value {
                                        0 => Key(code, Up, Some(ev)),
                                        1 => Key(code, Down, Some(ev)),
                                        2 => Key(code, Down, Some(ev)),
                                        _ => return Err(Error::new(InvalidData, "strange event value"))
                                    };

                                    runnable.run(update, &mut buff);

                                    if !buff.is_empty() {
                                        for e in buff.drain(0..) {
                                            match e {
                                                Key(_, _, Some(raw)) => {
                                                    sink.write_event(&raw).unwrap();
                                                },

                                                Key(c, m, None) => {
																										sink.write_event(&InputEvent {
																										    time: TimeVal::try_from(SystemTime::now()).unwrap(),
																										    event_type: EventType::EV_KEY,
																										    event_code: EventCode::EV_KEY(int_to_ev_key(c as u32).unwrap()),
																										    value: match m { Up => 0, Down => 1 }
																										}).unwrap();
                                                },

                                                _ => {}
                                            }
                                        }

                                        sink.write_event(&InputEvent {
                                            time: TimeVal::try_from(SystemTime::now()).unwrap(),
                                            event_type: EventType::EV_SYN,
                                            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                                            value: 0
                                        }).unwrap();
                                    }

                                    Ok(Mode::Read)
                                }, 

                                EventCode::EV_MSC(EV_MSC::MSC_SCAN) => Ok(Mode::Read),

                                _ => {
                                    sink.write_event(&ev).unwrap();
                                    Ok(Mode::Read)
                                }
                            }
                        }

                        ReadStatus::Sync => Ok(Mode::Sync)
                    }
                }),

            Mode::Sync => source
                .next_event(ReadFlag::SYNC)
                .and_then(|(status, _)| {
                    println!("SYNC!");
                    match status {
                        ReadStatus::Sync => Ok(Mode::Sync),
                        _ => Ok(Mode::Read)
                    }
                })
          };

        match res {
            Result::Err(err) => {

                match err.raw_os_error() {
                    Some(libc::EINTR) => {
                        use crate::Update::*;

                        runnable.run(Tick, &mut buff);

                        if !buff.is_empty() {
                            for e in buff.drain(0..) {
                                match e {
                                    Key(_, _, Some(raw)) => {
                                        sink.write_event(&raw).unwrap();
                                    },

                                    Key(c, m, None) => {
																				sink.write_event(&InputEvent {
																						time: TimeVal::try_from(SystemTime::now()).unwrap(),
																						event_type: EventType::EV_KEY,
																						event_code: EventCode::EV_KEY(int_to_ev_key(c as u32).unwrap()),
																						value: match m { Up => 0, Down => 1 }
																				}).unwrap();
                                    },

                                    _ => {}
                                }
                            }

                            sink.write_event(&InputEvent {
                                time: TimeVal::try_from(SystemTime::now()).unwrap(),
                                event_type: EventType::EV_SYN,
                                event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                                value: 0
                            }).unwrap();
                        }
                        
                        continue;
                    },

                    Some(libc::EAGAIN) => {
                        continue
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

    Ok(())
}


fn open_device(path: &str) -> Result<Device, Error>
{
    return Device::new_from_fd(File::open(&path).unwrap());
}
