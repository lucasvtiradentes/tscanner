use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::constants::get_log_filename;

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

pub struct Logger {
    file_path: PathBuf,
}

impl Logger {
    fn new() -> Self {
        let file_path = std::env::temp_dir().join(get_log_filename());
        Self { file_path }
    }

    fn write(&self, prefix: &str, level: &str, message: &str) {
        use time::OffsetDateTime;

        let now = OffsetDateTime::now_utc();
        let utc_minus_3 = now.to_offset(time::UtcOffset::from_hms(-3, 0, 0).unwrap());

        let timestamp = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{:+03}:{:02}",
            utc_minus_3.year(),
            utc_minus_3.month() as u8,
            utc_minus_3.day(),
            utc_minus_3.hour(),
            utc_minus_3.minute(),
            utc_minus_3.second(),
            utc_minus_3.millisecond(),
            -3,
            0
        );

        let log_message = format!("[{}] [{}] [{}] {}\n", timestamp, prefix, level, message);

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = file.write_all(log_message.as_bytes());
        }
    }

    pub fn info(&self, prefix: &str, message: &str) {
        self.write(prefix, "INFO", message);
    }

    pub fn error(&self, prefix: &str, message: &str) {
        self.write(prefix, "ERROR", message);
    }

    pub fn warn(&self, prefix: &str, message: &str) {
        self.write(prefix, "WARN", message);
    }

    pub fn debug(&self, prefix: &str, message: &str) {
        self.write(prefix, "DEBUG", message);
    }
}

pub fn get_logger() -> &'static Mutex<Option<Logger>> {
    &LOGGER
}

pub fn init_logger() {
    let mut logger = LOGGER.lock().unwrap();
    if logger.is_none() {
        *logger = Some(Logger::new());
    }
}

pub fn log_info(prefix: &str, message: &str) {
    init_logger();
    if let Ok(logger) = LOGGER.lock() {
        if let Some(l) = logger.as_ref() {
            l.info(prefix, message);
        }
    }
}

pub fn log_error(prefix: &str, message: &str) {
    init_logger();
    if let Ok(logger) = LOGGER.lock() {
        if let Some(l) = logger.as_ref() {
            l.error(prefix, message);
        }
    }
}

pub fn log_warn(prefix: &str, message: &str) {
    init_logger();
    if let Ok(logger) = LOGGER.lock() {
        if let Some(l) = logger.as_ref() {
            l.warn(prefix, message);
        }
    }
}

pub fn log_debug(prefix: &str, message: &str) {
    init_logger();
    if let Ok(logger) = LOGGER.lock() {
        if let Some(l) = logger.as_ref() {
            l.debug(prefix, message);
        }
    }
}
