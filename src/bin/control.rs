use std::{io, num::ParseIntError};

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc900;

use hidapi::{HidDevice, HidError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadEvaluatePrintError {
    #[error("reading from stdin")]
    Stdin(#[from] io::Error),
    #[error("parsing from hex")]
    Parse(#[from] ParseIntError),
    #[error("writing to device")]
    Write(HidError),
    #[error("reading from device")]
    Read(HidError),
}

fn read_evaluate_print(
    line: std::io::Result<String>,
    device: &HidDevice,
) -> Result<(), ReadEvaluatePrintError> {
    let send_buf = line?
        .split(":")
        .map(|hex| u8::from_str_radix(hex, 16))
        .collect::<Result<Vec<u8>, _>>()?;
    device
        .write(&send_buf)
        .map_err(ReadEvaluatePrintError::Read)?;
    let mut read_buf = vec![0; 20];
    let len = device
        .read(&mut read_buf)
        .map_err(ReadEvaluatePrintError::Write)?;

    println!(
        "{}",
        &read_buf[0..len]
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<Vec<String>>()
            .join(":")
    );

    Ok(())
}

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let device = api.open(VENDOR_ID, PRODUCT_ID).unwrap();

    for maybe_line in io::stdin().lines() {
        match read_evaluate_print(maybe_line, &device) {
            Ok(()) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
