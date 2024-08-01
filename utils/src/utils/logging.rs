use std::sync::Mutex;
use lazy_static::lazy_static;


enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}


pub struct Logger {
    level_: LogLevel,
    enabled_: bool,

}

impl Logger {
    fn new() -> Logger {
        Logger {
            level_: LogLevel::Info,
            enabled_: true,
        }
    }

    pub fn init(level: LogLevel, enabled: bool) {
    }
}
