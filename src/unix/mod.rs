mod dev_info;
mod timer;
mod glob;

#[cfg(test)]
mod test;

use crate::{Movement::*, machines::{Runnable, Ctx}};
use crate::common::Ev::*;
use evdev_rs::enums::*;
use evdev_rs::*;
use std::convert::TryFrom;
use std::time::Duration;
use std::time::SystemTime;
use std::{
    error,
    fs::File,
    io::{Error, ErrorKind::*},
};
use glob::Glob;

enum Mode {
    Read,
    Sync,
}


pub fn run<'a, TRun>(dev_pattern: &str, runnable: &mut TRun) -> Result<(), Error>
where
    TRun: Runnable<InputEvent>
{

    let dev_path = find_file(dev_pattern).unwrap();
    dbg!(&dev_path);
    
    let mut source = open_device(&dev_path).unwrap();
    source.grab(GrabMode::Grab).unwrap();

    let sink = UInputDevice::create_from_device(&source).unwrap();

    timer::catch_alrm().unwrap();
    let _timer = timer::set_itimer(Duration::from_millis(40)).unwrap();


    
    // let mut buff = VecDeque::new();
    let mut x = Ctx::new();
    let mut mode: Mode = Mode::Read;

    loop {
        let res = match mode {
            Mode::Read => source
                .next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
                .and_then(|(status, ev)| {
                    //event_info(&ev);
                    match status {
                        ReadStatus::Success => match ev.event_code {
                            EventCode::EV_KEY(_) => {
                                let code = ev.as_raw().code;

                                let update = match ev.value {
                                    0 => Key(code, Up, Some(ev)),
                                    1 => Key(code, Down, Some(ev)),
                                    2 => Key(code, Down, Some(ev)),
                                    _ => {
                                        return Err(Error::new(InvalidData, "strange event value"))
                                    }
                                };

                                runnable.run(&mut x, update);

                                if !x.buff.is_empty() {
                                    for emit in x.buff.drain(0..) {
                                        match emit.ev() {
                                            Key(_, _, Some(raw)) => {
                                                sink.write_event(&raw).unwrap();
                                            }

                                            Key(c, m, None) => {
                                                sink.write_event(&InputEvent {
                                                    time: TimeVal::try_from(SystemTime::now())
                                                        .unwrap(),
                                                    event_type: EventType::EV_KEY,
                                                    event_code: EventCode::EV_KEY(
                                                        int_to_ev_key(c as u32).unwrap(),
                                                    ),
                                                    value: match m {
                                                        Up => 0,
                                                        Down => 1,
                                                    },
                                                })
                                                .unwrap();
                                            }

                                            _ => {}
                                        }
                                    }

                                    sink.write_event(&InputEvent {
                                        time: TimeVal::try_from(SystemTime::now()).unwrap(),
                                        event_type: EventType::EV_SYN,
                                        event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                                        value: 0,
                                    })
                                    .unwrap();
                                }

                                Ok(Mode::Read)
                            }

                            EventCode::EV_MSC(EV_MSC::MSC_SCAN) => Ok(Mode::Read),

                            _ => {
                                sink.write_event(&ev).unwrap();
                                Ok(Mode::Read)
                            }
                        },

                        ReadStatus::Sync => Ok(Mode::Sync),
                    }
                }),

            Mode::Sync => source.next_event(ReadFlag::SYNC).and_then(|(status, _)| {
                println!("SYNC!");
                match status {
                    ReadStatus::Sync => Ok(Mode::Sync),
                    _ => Ok(Mode::Read),
                }
            }),
        };

        match res {
            Result::Err(err) => match err.raw_os_error() {
                Some(libc::EINTR) => {
                    runnable.run(&mut x, Tick);

                   if !x.buff.is_empty() {
                        for emit in x.buff.drain(0..) {
                            match emit.ev() {
                                Key(_, _, Some(raw)) => {
                                    sink.write_event(&raw).unwrap();
                                }

                                Key(c, m, None) => {
                                    sink.write_event(&InputEvent {
                                        time: TimeVal::try_from(SystemTime::now()).unwrap(),
                                        event_type: EventType::EV_KEY,
                                        event_code: EventCode::EV_KEY(
                                            int_to_ev_key(c as u32).unwrap(),
                                        ),
                                        value: match m {
                                            Up => 0,
                                            Down => 1,
                                        },
                                    })
                                    .unwrap();
                                }

                                _ => {}
                            }
                        }

                        sink.write_event(&InputEvent {
                            time: TimeVal::try_from(SystemTime::now()).unwrap(),
                            event_type: EventType::EV_SYN,
                            event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                            value: 0,
                        })
                        .unwrap();
                    }

                    continue;
                }

                Some(libc::EAGAIN) => continue,

                _ => {
                    println!("ERROR! {}", err);
                    break;
                }
            },

            Result::Ok(next) => {
                mode = next;
                continue;
            }
        }
    }

    Ok(())
}

fn open_device(path: &str) -> Result<Device, Error> {
    return Device::new_from_fd(File::open(&path).unwrap());
}

pub fn find_file(pattern: &str) -> Result<String, Box<dyn error::Error>> {
    Glob::glob(pattern)
        .and_then(|res| {
            dbg!(&res.paths);

            let found = res.paths.first()
                .map(|s| s.to_string());

            match found {
                Some(s) => Ok(s),
                None => Err(Error::new(NotFound, "can't glob device file").into())
            }
        })
}
