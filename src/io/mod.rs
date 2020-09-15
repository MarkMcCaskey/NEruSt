#[cfg(feature = "native-renderer")]
pub mod native;

#[derive(Debug, Clone)]
pub enum InputEvent {
    Paused,
    Quit,
    LostFocus,
    GainedFocus,
    ButtonPress { player: u8, button: Button },
    ButtonRelease { player: u8, button: Button },
    // TODO: add event for controller connected and disconnected
}

#[derive(Debug, Copy, Clone)]
pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

pub trait Input {
    /// Get the next piece of input
    fn get_next_input(&mut self) -> Option<InputEvent>;
}

pub trait Output {
    /// Draws to the screen. Colors is RGB (interface is WIP)
    fn display_screen(&mut self, colors: &[u8]);
}
