#[repr(C)]
#[derive(Debug)]
pub struct System {}

impl System {
    pub fn new() -> Self {
        ezgl::InstantDraw::bind_vao();
        ezgl::InstantDraw::enable_debug();

        Self {}
    }

    pub fn draw(&mut self) {
        unsafe {
            ::gl::ClearColor(1., 0., 1., 1.);
        }
        ezgl::InstantDraw::clear();
    }

    pub fn step(&mut self) {
        self.draw();
    }
}
