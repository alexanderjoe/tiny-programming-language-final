use std::sync::Mutex;
use lazy_static::lazy_static;

pub enum Level {
    Info,
    Debug,
    Warn,
    None,
}

pub struct Logger {
    pub level: Level,
}

impl Logger {
    fn log(level: Level, message: &str) {
        let logger = LOGGER.lock().unwrap();
        match level {
            Level::Info => {
                if let Level::Info | Level::Debug | Level::Warn = logger.level {
                    println!("[INFO] {}", message);
                }
            },
            Level::Debug => {
                if let Level::Debug | Level::Warn = logger.level {
                    println!("[DEBUG] {}", message);
                }
            },
            Level::Warn => {
                if let Level::Warn = logger.level {
                    println!("[WARN] {}", message);
                }
            },
            Level::None => {},
        }
    }

    pub fn info(message: &str) {
        Self::log(Level::Info, message);
    }

    pub fn debug(message: &str) {
        Self::log(Level::Debug, message);
    }

    pub fn warn(message: &str) {
        Self::log(Level::Warn, message);
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger {
        level: Level::Info,
    });
}