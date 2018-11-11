use gleam::gl::{self, Gl};
type GlPtr = std::rc::Rc<Gl>;
use crate::canvas;

#[repr(C)]
pub struct System {
    gl: GlPtr,
}

impl System {
    pub fn new(gl: GlPtr) -> Self {
        /*let program = gl.create_program();
        gl.attach_shader(program, v_shader);
        gl.attach_shader(program, f_shader);
        gl.link_program(program);
        gl.use_program(program);*/

        Self { gl }
    }

    pub fn draw(&self) {
        let gl = &self.gl;
        let (width, height) = canvas::get_canvas_size();
        gl.viewport(0, 0, width as i32, height as i32);
        gl.clear_color(1., 0., 0., 1.);
        gl.clear(gl::COLOR_BUFFER_BIT);
        //ezgl::InstantDraw::clear();
    }

    pub fn step(&mut self) {
        self.draw();
    }
}
