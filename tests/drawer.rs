extern crate kaiga;
extern crate glium;

use glium::{glutin, DisplayBuild};

use std::env;
use glium::backend::Facade;

#[test]
fn build_device() {
    let display = build_display();
    let device = kaiga::Device::new(display.get_context());
    drop(device);
}

#[test]
fn build_drawer() {
    let display = build_display();
    let device = kaiga::Device::new(display.get_context());

    let drawer = kaiga::Drawer::new(&device);

    drawer.submit();
}


#[test]
fn draw_rectangle() {
    let display = build_display();
    let device = kaiga::Device::new(display.get_context());

    let mut drawer = kaiga::Drawer::new(&device);

    drawer.draw_rectangle(12,53,566,234,[0.8,1.0,0.23]);

    drawer.submit();
}


fn build_display() -> glium::Display {
    if env::var("KAIGA_HEADLESS_TESTS").is_ok() {
        glutin::HeadlessRendererBuilder::new(1024, 768).with_gl_debug_flag(true)
                                                       .build_glium().unwrap()
    } else {
        glutin::WindowBuilder::new().with_gl_debug_flag(true).with_visibility(false)
                                    .build_glium().unwrap()
    }
}
