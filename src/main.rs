use std::rc::Rc;

use gtk::{prelude::*, ToggleButton, Builder, Application, ApplicationWindow};
use hidapi::HidDevice;

const APP_ID: &str = "com.github.ccouzens.litra-glow-gtk";

fn main() {
    let api = hidapi::HidApi::new().unwrap();
    let device = Rc::new(api.open(0x046d, 0xc900).unwrap());

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app: &Application| build_ui(device.clone(), app));
    app.run();
}

fn build_ui(device: Rc<HidDevice>, app: &Application) {
    let builder = Builder::from_string(include_str!("../com.github.ccouzens.litra_glow_gtk.ui"));
    let button: ToggleButton = builder.object("light-toggle").expect("Couldn't get toggle");
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");

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

    app.add_window(&window);

    window.present();
}
