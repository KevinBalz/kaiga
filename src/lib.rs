#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{Surface,Version, Api};
use std::rc::Rc;

pub struct Device {
    context: Rc<glium::backend::Context>,
    rect_data: RectData,
    image_data: ImageData,
}

impl Device {
    pub fn new(context: &Rc<glium::backend::Context>) -> Device {
        let rect_data = init_rectangle(context);
        let image_data = init_image(context);
        Device {
            context: context.clone(),
            rect_data: rect_data,
            image_data: image_data,
        }
    }

    pub fn begin(&self) -> Drawer {
        Drawer::new(self)
    }
}

pub struct Drawer<'s> {
    frame: glium::Frame,
    device: &'s Device,
    projection: [[f32; 4]; 4],
    params: glium::DrawParameters<'s>,
}

impl<'s> Drawer<'s> {
    pub fn new(device: &'s Device) -> Drawer<'s> {
        let (w, h) = device.context.get_framebuffer_dimensions();
        let frame = glium::Frame::new(device.context.clone(), (w, h));
        let mut params: glium::DrawParameters = Default::default();

        params.viewport = Some(glium::Rect {
            left: 0,
            bottom: 0,
            width: w,
            height: h,
        });
        params.smooth = None;
        let projection = cgmath::ortho(0.0, w as f32, h as f32, 0.0, 0.0, 100.0).into();

        Drawer {
            frame: frame,
            params: params,
            device: device,
            projection: projection,
        }
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: [f32; 3]) {
        draw_rectangle(self, x, y, width, height, color)
    }

    pub fn draw_image(&mut self, image: &Texture, x: i32, y: i32, scale: i32) {
        draw_image(self, image, x, y, scale)
    }


    pub fn clear(&mut self, r: f32, g: f32, b: f32) {
        self.frame.clear_color(r, g, b, 1.0);
    }

    pub fn submit(self) {
        self.frame.finish().unwrap()
    }
}

macro_rules! include_shader {
    ($name:expr,$version:expr,$ty:expr) => (
        include_str!(concat!("shader/",$name,"_",$version,".",$ty,".glsl"))
    )
}

macro_rules! get_shader_source {
    ($context:expr,$name:expr) => (
        if $context.is_glsl_version_supported(&Version(Api::Gl,1,2)) {
            (include_shader!($name,120,"v"),include_shader!($name,120,"f"))
        }
        else if $context.is_glsl_version_supported(&Version(Api::Gl,1,5)) {
            (include_shader!($name,150,"v"),include_shader!($name,150,"f"))
        }
        else {
            panic!("unsupported GLSL Version {:?}", $context.get_supported_glsl_version());
        }
    )
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

fn init_rectangle(context: &Rc<glium::backend::Context>) -> RectData {
    let shape = [RectVertex { position: [0.0, 0.0] },
                 RectVertex { position: [1.0, 0.0] },
                 RectVertex { position: [0.0, 1.0] },

                 RectVertex { position: [0.0, 1.0] },
                 RectVertex { position: [1.0, 0.0] },
                 RectVertex { position: [1.0, 1.0] }];

    let vertex_buffer = glium::VertexBuffer::new(context, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let (vertex_source,frag_source) = get_shader_source!(context,"rectangle");
    let program = glium::Program::from_source(context,
                                              vertex_source,
                                              frag_source,
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
          .draw(&drawer.device.rect_data.vertex_buffer,
                &drawer.device.rect_data.indices,
                &drawer.device.rect_data.program,
                &uniform! {
                    color: color,
                    projection: drawer.projection,
                    model: model },
                &drawer.params)
          .unwrap();
}

pub struct Texture {
    tex: glium::texture::CompressedSrgbTexture2d,
    dimensions: (u32, u32),
}

impl Texture {
    pub fn new(device: &Device, pixels: Vec<u8>, dimensions: (u32, u32)) -> Self {
        let raw = glium::texture::RawImage2d::from_raw_rgba(pixels, dimensions);
        let tex = glium::texture::CompressedSrgbTexture2d::new(&device.context, raw).unwrap();
        Texture {
            tex: tex,
            dimensions: dimensions,
        }
    }
}

// Draw Image Helpers

#[derive(Copy, Clone)]
struct ImageVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

implement_vertex!(ImageVertex, position, texcoord);

struct ImageData {
    pub vertex_buffer: glium::VertexBuffer<ImageVertex>,
    pub indices: glium::index::NoIndices,
    pub program: glium::Program,
}

fn init_image(context: &Rc<glium::backend::Context>) -> ImageData {
    let shape = [ImageVertex {
                     position: [0.0, 0.0],
                     texcoord: [0.0, 0.0],
                 },
                 ImageVertex {
                     position: [1.0, 0.0],
                     texcoord: [1.0, 0.0],
                 },
                 ImageVertex {
                     position: [0.0, 1.0],
                     texcoord: [0.0, 1.0],
                 },

                 ImageVertex {
                     position: [0.0, 1.0],
                     texcoord: [0.0, 1.0],
                 },
                 ImageVertex {
                     position: [1.0, 0.0],
                     texcoord: [1.0, 0.0],
                 },
                 ImageVertex {
                     position: [1.0, 1.0],
                     texcoord: [1.0, 1.0],
                 }];

    let vertex_buffer = glium::VertexBuffer::new(context, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let (vertex_source,frag_source) = get_shader_source!(context,"image");
    let program = glium::Program::from_source(context,
                                              vertex_source,
                                              frag_source,
                                              None)
                      .unwrap();

    ImageData {
        vertex_buffer: vertex_buffer,
        indices: indices,
        program: program,
    }
}

pub fn draw_image(drawer: &mut Drawer, image: &Texture, x: i32, y: i32, scale: i32) {
    let (width, height) = image.dimensions;
    let scale = scale as u32;
    let trans = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x as f32, y as f32, 0.0));
    let scale = cgmath::Matrix4::from_nonuniform_scale((width * scale) as f32,
                                                       (height * scale) as f32,
                                                       1.0);

    let model: [[f32; 4]; 4] = (trans * scale).into();

    let mag_filter = glium::uniforms::MagnifySamplerFilter::Nearest;

    drawer.frame
          .draw(&drawer.device.image_data.vertex_buffer,
                &drawer.device.image_data.indices,
                &drawer.device.image_data.program,
                &uniform! {
                    projection: drawer.projection,
                    model: model,
                    image: image.tex.sampled().magnify_filter(mag_filter)},
                &drawer.params)
          .unwrap();

}
