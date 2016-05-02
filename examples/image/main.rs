extern crate glium;
extern crate kaiga;
extern crate image;

use std::io::Cursor;
use image::GenericImage;

fn main() {
    use glium::DisplayBuild;
    use glium::backend::Facade;
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Image example".to_string())
        .build_glium().unwrap();

    let device = kaiga::Device::new(display.get_context());


    let image = image::load(Cursor::new(&include_bytes!("./door.png")[..]),
                            image::PNG).unwrap();
    let texture = kaiga::Texture::new(&device,image.raw_pixels(),image.dimensions());

    loop {

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }

        let mut drawer = device.begin();

        drawer.clear(0.0,0.0,0.0);

        drawer.draw_image(&texture, 100, 100, 10);

        drawer.submit()
    }
}
