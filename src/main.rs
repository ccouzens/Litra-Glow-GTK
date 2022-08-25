use std::rc::Rc;

use gtk::{glib::clone, prelude::*, Application, ApplicationWindow, Builder, Scale, ToggleButton};
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
    let brigthness_scale: Scale = builder
        .object("brightness-scale")
        .expect("Couldn't get brigthness scale");
    brigthness_scale.set_range(20.0, 250.0);
    brigthness_scale.set_increments(1.0, 230.0 / 5.0);

    let colour_scale: Scale = builder
        .object("colour-scale")
        .expect("Couldn't get colour scale");
    colour_scale.set_range(-6500.0, -2700.0);
    colour_scale.set_increments(100.0, 800.0);

    button.connect_toggled(clone!(@strong device => move |button| {
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
    }));

    brigthness_scale.connect_value_changed(clone!(@strong device => move |scale| {
        device
            .write(&[
                0x11,
                0xff,
                0x04,
                0x4d,
                00,
                scale.value() as u8,
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
    }));

    colour_scale.connect_value_changed(move |scale| {
        let value = -scale.value() as u16;
        device
            .write(&[
                0x11,
                0xff,
                0x04,
                0x9d,
                (value >> 8) as u8,
                (value & 0x00FF) as u8,
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
