use std::io;

const VENDOR_ID: u16 = 0x046d;
const PRODUCT_ID: u16 = 0xc900;

use hidapi::{HidDevice, HidError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadEvaluatePrintError {
    #[error("reading from stdin")]
    Stdin(#[from] io::Error),
    #[error("odd digits on stdin")]
    OddDigits,
    #[error("parsing from hex")]
    Parse,
    #[error("writing to device")]
    Write(HidError),
    #[error("reading from device")]
    Read(HidError),
}

fn read_evaluate_print(
    line: std::io::Result<String>,
    device: &HidDevice,
) -> Result<(), ReadEvaluatePrintError> {
    let line = line?;
    let mut send_buf = Vec::new();
    let mut send_iter = line.bytes().filter(|c| c.is_ascii_hexdigit());
    while let Some(d1) = send_iter.next() {
        let d2 = send_iter.next().ok_or(ReadEvaluatePrintError::OddDigits)?;
        send_buf.push(
            (char::to_digit(d1 as char, 16).ok_or(ReadEvaluatePrintError::Parse)? * 16
                + char::to_digit(d2 as char, 16).ok_or(ReadEvaluatePrintError::Parse)?)
                as u8,
        );
    }
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
