use nes::{Button, Nes};
use sdl2::{self, event::Event, keyboard::Keycode, render::Canvas, video::Window, Sdl};

pub struct EmuWindow {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
}

impl EmuWindow {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = match video_subsystem
            .window("Rust NES emulator", 800, 600)
            .position_centered()
            .build()
        {
            Ok(v) => v,
            _ => return Err("Could not create window".to_owned()),
        };
        let canvas = match window.into_canvas().software().build() {
            Ok(v) => v,
            _ => return Err("Could not create canvas".to_owned()),
        };
        Ok(Self {
            sdl_context,
            canvas,
        })
    }

    pub fn draw(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn handle_events(&mut self, nes: &mut Nes) {
        for event in self.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => ::std::process::exit(0),

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::A => nes.set_button(Button::A, false, true),
                    Keycode::B => nes.set_button(Button::B, false, true),
                    Keycode::H => nes.set_button(Button::Left, false, true),
                    Keycode::J => nes.set_button(Button::Down, false, true),
                    Keycode::K => nes.set_button(Button::Up, false, true),
                    Keycode::L => nes.set_button(Button::Right, false, true),
                    Keycode::Num1 => nes.set_button(Button::Start, false, true),
                    Keycode::Num2 => nes.set_button(Button::Select, false, true),
                    Keycode::Escape => ::std::process::exit(0),
                    _ => (),
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::A => nes.set_button(Button::A, false, false),
                    Keycode::B => nes.set_button(Button::B, false, false),
                    Keycode::H => nes.set_button(Button::Left, false, false),
                    Keycode::J => nes.set_button(Button::Down, false, false),
                    Keycode::K => nes.set_button(Button::Up, false, false),
                    Keycode::L => nes.set_button(Button::Right, false, false),
                    Keycode::Num1 => nes.set_button(Button::Start, false, false),
                    Keycode::Num2 => nes.set_button(Button::Select, false, false),
                    _ => (),
                },
                _ => (),
            }
        }
    }
}
