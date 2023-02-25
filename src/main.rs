use std::fs::File;

use evdev_rs::enums::{EventCode, EV_KEY};
use evdev_rs::{Device, DeviceWrapper, ReadFlag};

// /dev/input/by-id/usb-Cooler_Master_Technology_Inc._MASTERKEYS_PRO_S_with_intelligent_RGB-event-kbd
fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    println!("{:?}", args);

    if args.len() != 2 {
        let n = args.nth(0).unwrap();
        println!("Usage: `{} DEVICE`, eg. `{} /dev/input/event13`", n, n);
        std::process::exit(1);
    }
    let device = &args.nth(1).unwrap();

    // Connect to real keyboard
    let f = File::open(device)?;
    let d = Device::new_from_file(f)?;

    if let Some(n) = d.name() {
        println!(
            "Connected to device: '{}' ({:04x}:{:04x})",
            n,
            d.vendor_id(),
            d.product_id()
        );
    }
    loop {
        let (_status, event) = d.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)?;
        // Map these to mouse events
        println!("Event: {:?}", event.event_code);

        // let e = match event.event_code {
        //     EventCode::EV_KEY(EV_KEY::KEY_RIGHT) => println!("Right"),
        //     EventCode::EV_KEY(EV_KEY::KEY_LEFT) => println!("Left"),
        //     EventCode::EV_KEY(EV_KEY::KEY_UP) => println!("Up"),
        //     EventCode::EV_KEY(EV_KEY::KEY_DOWN) => println!("Down"),
        //     _ => None,
        // };
    }
    return Result::Ok(());
}
