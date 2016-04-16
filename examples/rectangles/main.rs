extern crate glium;
extern crate kaiga;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Rectangles example".to_string())
        .build_glium().unwrap();

    loop {

        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,   // the window has been closed by the user
                _ => ()
            }
        }

        let mut drawer = kaiga::Drawer::new(&display);

        drawer.clear(0.0,0.0,0.0);

        drawer.draw_rectangle( 50, 50,200,200,[1.0,0.0,0.0]);

        drawer.draw_rectangle(400, 50,400,200,[1.0,0.0,1.0]);

        drawer.draw_rectangle( 50,300,200,400,[0.0,1.0,0.0]);

        drawer.draw_rectangle(400,300,400,400,[0.0,0.0,1.0]);

        drawer.submit()
    }
}
