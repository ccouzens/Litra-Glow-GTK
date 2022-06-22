use std::{thread, time};

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc900;
const LIGHT_ON: u8 = 0x01;
const LIGHT_OFF: u8 = 0x00;
const MIN_BRIGHTNESS: u8 = 0x14;
const MAX_BRIGHTNESS: u8 = 0xfa;
const ONE_SECOND: time::Duration = time::Duration::from_secs(1);

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

    device
        .write(&[
            0x11, 0xff, 0x04, 0x1c, LIGHT_ON, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ])
        .unwrap();
    println!("Light on!");

    thread::sleep(ONE_SECOND);

    for temp in (2700..=6500u16).step_by(100) {
        device
            .write(&[
                0x11,
                0xff,
                0x04,
                0x9c,
                (((temp >> 8) & 0xFF) as u8),
                ((temp & 0xFF) as u8),
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ])
            .unwrap();

        println!("Temperature {temp}° K");
        thread::sleep(time::Duration::from_millis(100));
    }

    thread::sleep(ONE_SECOND);

    for brightness in MIN_BRIGHTNESS..=MAX_BRIGHTNESS {
        device
            .write(&[
                0x11, 0xff, 0x04, 0x4c, 0x00, brightness, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ])
            .unwrap();
        println!("Brightness {brightness}");
        thread::sleep(time::Duration::from_millis(20));
    }

    thread::sleep(ONE_SECOND);

    device
        .write(&[
            0x11, 0xff, 0x04, 0x1c, LIGHT_OFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ])
        .unwrap();
    println!("Light off!");
}
