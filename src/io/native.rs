use crate::io::{Button, Input, InputEvent, Output};
use sdl2::{pixels::Color, render::WindowCanvas, EventPump, Sdl};

pub struct NativeRenderer {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl NativeRenderer {
    pub fn new(x: u32, y: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("nerust NES emulator", x, y)
            .position_centered()
            .build()
            .map_err(|e| format!("{}", e))?;

        let mut canvas = window.into_canvas().build().map_err(|e| format!("{}", e))?;

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            sdl_context,
            canvas,
            event_pump,
        })
    }
}

impl Input for NativeRenderer {
    fn get_next_input(&mut self) -> Option<InputEvent> {
        use sdl2::event::Event;
        let sdl_event = self.event_pump.poll_event()?;

        Some(match sdl_event {
            Event::AppTerminating { .. } | Event::Quit { .. } => InputEvent::Quit,
            // TODO: add other window events...
            Event::AppDidEnterBackground { .. } => InputEvent::LostFocus,
            Event::AppDidEnterForeground { .. } => InputEvent::GainedFocus,
            Event::JoyButtonDown {
                which, button_idx, ..
            } => {
                dbg!(which, button_idx);
                InputEvent::ButtonPress {
                    player: 0,
                    button: Button::A,
                }
            }
            Event::JoyButtonUp {
                which, button_idx, ..
            } => {
                dbg!(which, button_idx);
                InputEvent::ButtonRelease {
                    player: 0,
                    button: Button::A,
                }
            } // TODO: add event for controller connected and disconnected
            _ => return None,
        })
    }
}

impl Output for NativeRenderer {
    fn display_screen(&mut self, colors: &[u8]) {
        todo!()
    }
}
