// use evdev_rs::{Device, InputEvent, enums::{EV_ABS, EV_KEY, EV_LED, EV_REL, EventCode, EventType, InputProp}};

// pub fn dev_info(d: &Device) {
// 		println!(
// 				"Input device ID: bus 0x{:x} vendor 0x{:x} product 0x{:x}",
// 				d.bustype(),
// 				d.vendor_id(),
// 				d.product_id()
// 		);
// 		println!("Evdev version: {:x}", d.driver_version());
// 		println!("Input device name: \"{}\"", d.name().unwrap_or(""));
// 		println!("Phys location: {}", d.phys().unwrap_or(""));
// 		println!("Uniq identifier: {}", d.uniq().unwrap_or(""));

// 		print_props(&d);
// 		print_bits(&d);
// }

// fn print_abs_bits(dev: &Device, axis: &EV_ABS) {
//     let code = EventCode::EV_ABS(axis.clone());

//     if !dev.has(&code) {
//         return;
//     }

//     let abs = dev.abs_info(&code).unwrap();

//     println!("\tValue\t{}", abs.value);
//     println!("\tMin\t{}", abs.minimum);
//     println!("\tMax\t{}", abs.maximum);
//     if abs.fuzz != 0 {
//         println!("\tFuzz\t{}", abs.fuzz);
//     }
//     if abs.flat != 0 {
//         println!("\tFlat\t{}", abs.flat);
//     }
//     if abs.resolution != 0 {
//         println!("\tResolution\t{}", abs.resolution);
//     }
// }

// fn print_code_bits(dev: &Device, ev_code: &EventCode, max: &EventCode) {
//     for code in ev_code.iter() {
//         if code == *max {
//             break;
//         }
//         if !dev.has(&code) {
//             continue;
//         }

//         println!("    Event code: {}", code);
//         match code {
//             EventCode::EV_ABS(k) => print_abs_bits(dev, &k),
//             _ => (),
//         }
//     }
// }

// fn print_bits(dev: &Device) {
//     println!("Supported events:");

//     for ev_type in EventType::EV_SYN.iter() {
//         if dev.has(&ev_type) {
//             println!("  Event type: {} ", ev_type);
//         }

//         match ev_type {
//             EventType::EV_KEY => print_code_bits(
//                 dev,
//                 &EventCode::EV_KEY(EV_KEY::KEY_RESERVED),
//                 &EventCode::EV_KEY(EV_KEY::KEY_MAX),
//             ),
//             EventType::EV_REL => print_code_bits(
//                 dev,
//                 &EventCode::EV_REL(EV_REL::REL_X),
//                 &EventCode::EV_REL(EV_REL::REL_MAX),
//             ),
//             EventType::EV_ABS => print_code_bits(
//                 dev,
//                 &EventCode::EV_ABS(EV_ABS::ABS_X),
//                 &EventCode::EV_ABS(EV_ABS::ABS_MAX),
//             ),
//             EventType::EV_LED => print_code_bits(
//                 dev,
//                 &EventCode::EV_LED(EV_LED::LED_NUML),
//                 &EventCode::EV_LED(EV_LED::LED_MAX),
//             ),
//             _ => (),
//         }
//     }
// }

// fn print_props(dev: &Device) {
//     println!("Properties:");

//     for input_prop in InputProp::INPUT_PROP_POINTER.iter() {
//         if dev.has(&input_prop) {
//             println!("  Property type: {}", input_prop);
//         }
//     }
// }

// pub fn event_info(ev: &InputEvent) {
//     match ev.event_code {
//         EventCode::EV_SYN(_) => println!(
//             "Event: time {}.{}, ++++++++++++++++++++ {} +++++++++++++++",
//             ev.time.tv_sec,
//             ev.time.tv_usec,
//             ev.event_type
//         ),
//         _ => println!(
//             "Event: time {}.{}, type {} , code {} , value {}",
//             ev.time.tv_sec,
//             ev.time.tv_usec,
//             ev.event_type,
//             ev.event_code,
//             ev.value
//         ),
//     }
// }

// pub fn dropped_event_info(ev: &InputEvent) {
//     print!("SYNC DROPPED: ");
//     event_info(ev);
// }
