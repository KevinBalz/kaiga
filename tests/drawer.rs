extern crate kaiga;
extern crate glium;

use glium::{glutin, DisplayBuild};

use std::env;

#[test]
fn build_drawer() {
    let display = build_display();

    let drawer = kaiga::Drawer::new(&display);

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
