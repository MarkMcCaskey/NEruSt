#[cfg(target_arch = "wasm32")]
mod log {
    use crate::wasm;
    use ::log::{Level, Log, Metadata, Record};

    pub struct Logger;

    impl Log for Logger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            match record.level() {
                Level::Error => wasm::error(&format!("{}", record.args())),
                Level::Warn => wasm::warn(&format!("{}", record.args())),
                Level::Info => wasm::info(&format!("{}", record.args())),
                Level::Debug => wasm::debug(&format!("{}", record.args())),
                Level::Trace => wasm::log(&format!("{}", record.args())),
            }
        }

        fn flush(&self) {}
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod log {
    use ::log::{Log, Metadata, Record};

    pub struct Logger;

    impl Log for Logger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            true
        }

        fn log(&self, record: &Record) {
            println!(
                "{}:{} -- {}",
                record.level(),
                record.target(),
                record.args()
            );
        }

        fn flush(&self) {}
    }
}

pub use self::log::*;

pub fn attach_logger(max_level: ::log::LevelFilter) {
    match ::log::set_boxed_logger(Box::new(Logger)) {
        Ok(()) => (),
        Err(_) => error!("Failed to attach logger! Was it already attached?"),
    }
    ::log::set_max_level(max_level);
}
