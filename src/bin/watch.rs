const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc900;

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

    let mut buf = vec![0; 20];

    loop {
        let len = device.read(&mut buf).unwrap();

        println!(
            "{}",
            &buf[0..len]
                .iter()
                .map(|n| format!("{:02x}", n))
                .collect::<Vec<String>>()
                .join(":")
        );
    }
}
