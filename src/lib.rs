#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::Surface;

pub struct Drawer<'s> {
    frame: glium::Frame,
    rect_data: RectData,
    projection: [[f32; 4]; 4],
    params: glium::DrawParameters<'s>,
}

impl<'s> Drawer<'s> {
    pub fn new(display: &glium::Display) -> Drawer<'s> {
        let frame = display.draw();
        let mut params: glium::DrawParameters = Default::default();
        let (w, h) = frame.get_dimensions();
        params.viewport = Some(glium::Rect {
            left: 0,
            bottom: 0,
            width: w,
            height: h,
        });
        let projection = cgmath::ortho(0.0, w as f32, h as f32, 0.0, 0.0, 100.0).into();

        let rect_data = init_rectangle(display);
        Drawer {
            frame: frame,
            params: params,
            rect_data: rect_data,
            projection: projection,
        }
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: [f32; 3]) {
        draw_rectangle(self, x, y, width, height, color)
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32) {
        self.frame.clear_color(r, g, b, 1.0);
    }

    pub fn submit(self) {
        self.frame.finish().unwrap()
    }
}

// Rectangle Helpers

#[derive(Copy, Clone)]
struct RectVertex {
    position: [f32; 2],
}

implement_vertex!(RectVertex, position);

struct RectData {
    pub vertex_buffer: glium::VertexBuffer<RectVertex>,
    pub indices: glium::index::NoIndices,
    pub program: glium::Program,
}

fn init_rectangle(display: &glium::Display) -> RectData {
    let shape = [RectVertex { position: [0.0, 0.0] },
                 RectVertex { position: [1.0, 0.0] },
                 RectVertex { position: [0.0, 1.0] },

                 RectVertex { position: [0.0, 1.0] },
                 RectVertex { position: [1.0, 0.0] },
                 RectVertex { position: [1.0, 1.0] }];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(display,
                                              include_str!("shader/rectangle.v.glsl"),
                                              include_str!("shader/rectangle.f.glsl"),
                                              None)
                      .unwrap();

    RectData {
        vertex_buffer: vertex_buffer,
        indices: indices,
        program: program,
    }
}

fn draw_rectangle(drawer: &mut Drawer, x: i32, y: i32, width: i32, height: i32, color: [f32; 3]) {
    let trans = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x as f32, y as f32, 0.0));
    let scale = cgmath::Matrix4::from_nonuniform_scale(width as f32, height as f32, 1.0);

    let model: [[f32; 4]; 4] = (trans * scale).into();

    drawer.frame
          .draw(&drawer.rect_data.vertex_buffer,
                &drawer.rect_data.indices,
                &drawer.rect_data.program,
                &uniform! {
        color: color,
        projection: drawer.projection,
        model: model },
                &drawer.params)
          .unwrap();
}
