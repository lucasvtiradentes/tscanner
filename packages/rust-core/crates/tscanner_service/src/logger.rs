use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use tscanner_config::get_log_filename;

const TIMEZONE_OFFSET_HOURS: i8 = -3;

static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

pub struct Logger {
    file_path: PathBuf,
    context: String,
}

impl Logger {
    fn new(context: String) -> Self {
        let file_path = std::env::temp_dir().join(get_log_filename());
        Self { file_path, context }
    }

    fn write(&self, level: &str, message: &str) {
        use time::OffsetDateTime;

        let now = OffsetDateTime::now_utc();
        let local_time =
            now.to_offset(time::UtcOffset::from_hms(TIMEZONE_OFFSET_HOURS, 0, 0).unwrap());

        let timestamp = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{:+03}:{:02}",
            local_time.year(),
            local_time.month() as u8,
            local_time.day(),
            local_time.hour(),
            local_time.minute(),
            local_time.second(),
            local_time.millisecond(),
            TIMEZONE_OFFSET_HOURS,
            0
        );

        let log_message = format!(
            "[{}] [{}] [{}] {}\n",
            timestamp, self.context, level, message
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = file.write_all(log_message.as_bytes());
        }
    }

    pub fn info(&self, message: &str) {
        self.write("INFO ", message);
    }

    pub fn error(&self, message: &str) {
        self.write("ERROR", message);
    }

    pub fn warn(&self, message: &str) {
        self.write("WARN ", message);
    }

    pub fn debug(&self, message: &str) {
        self.write("DEBUG", message);
    }
}

pub fn get_logger() -> &'static Mutex<Option<Logger>> {
    &LOGGER
}

pub fn init_logger(context: &str) {
    let mut logger = LOGGER.lock().unwrap();
    if logger.is_none() {
        *logger = Some(Logger::new(context.to_string()));
    }
}

fn with_logger<F>(f: F)
where
    F: FnOnce(&Logger),
{
    if let Ok(logger) = LOGGER.lock() {
        if let Some(l) = logger.as_ref() {
            f(l);
        }
    }
}

pub fn log_info(message: &str) {
    with_logger(|l| l.info(message));
}

pub fn log_error(message: &str) {
    with_logger(|l| l.error(message));
}

pub fn log_warn(message: &str) {
    with_logger(|l| l.warn(message));
}

pub fn log_debug(message: &str) {
    with_logger(|l| l.debug(message));
}
