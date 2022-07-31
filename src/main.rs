use std::rc::Rc;

use gtk::{prelude::*, ToggleButton};
use gtk::{Application, ApplicationWindow};
use hidapi::HidDevice;

const APP_ID: &str = "com.github.ccouzens.litra-glow-gtk";

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let device = Rc::new(api.open(0x046d, 0xc900).unwrap());

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(move |app: &Application| build_ui(device.clone(), app));

    // Run the application
    app.run();
}

fn build_ui(device: Rc<HidDevice>, app: &Application) {
    let button = ToggleButton::builder()
        .label("Light")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_toggled(move |button| {
        device
            .write(&[
                0x11,
                0xff,
                0x04,
                0x1a,
                if button.is_active() { 1 } else { 0 },
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
                00,
            ])
            .unwrap();
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Litra Glow Control")
        .child(&button)
        .build();

    // Present window
    window.present();
}
