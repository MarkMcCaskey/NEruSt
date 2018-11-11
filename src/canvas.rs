use emscripten::*;

pub fn get_canvas_size() -> (u32, u32) {
    unsafe {
        let mut width = std::mem::uninitialized();
        let mut height = std::mem::uninitialized();
        emscripten_get_element_css_size(std::ptr::null(), &mut width, &mut height);
        (width as u32, height as u32)
    }
}
