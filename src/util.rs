use std::fmt::Display as FmtDisplay;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, LockResult, Mutex};



pub trait PoisonClearer<Guard> {

    fn ignore_poison(self) -> Guard;

}

impl<Guard> PoisonClearer<Guard> for LockResult<Guard> {
    fn ignore_poison(self) -> Guard {

        self.unwrap_or_else(|e| e.into_inner())
    }
}



pub trait ResultLoggerExcept<T> {

    fn expect_logger(self, logger: &GlobalLogger, error_name: &str) -> T;

}

impl<T, E: FmtDisplay> ResultLoggerExcept<T> for Result<T, E> {
    fn expect_logger(self, logger: &GlobalLogger, error_name: &str) -> T {

        match self {
            Ok(t) => t,
            Err(error) => {
                logger.lock().ignore_poison().err(&format!("{error_name} > {error}."));

                exit(1)
            },
        }
    }
}



pub type GlobalLogger = Arc<Mutex<Logger>>;

pub struct Logger {

    colored_terminal: bool,
    file_writer: File,
    has_already_file_writing_failed: bool,

}

impl Logger {

    pub fn new() -> Self {
        let file_writer = match File::create("wojenka logs.txt") {
            Ok(file) => file,
            Err(error) => {
                Self::stderr_log(LogType::Error, &format!("Failed to create log file. The error: {error}."), false);

                exit(1)
            },
        };

        Self {
            colored_terminal: true,
            file_writer,
            has_already_file_writing_failed: false,
        }
    }

    pub fn to_global(self) -> GlobalLogger {

        Arc::new( Mutex::new( self))
    }



    pub fn info(&mut self, message: &str) {
        self.log_message(LogType::Information, message);
    }

    pub fn warn(&mut self, message: &str) {
        self.log_message(LogType::Warning, message);
    }

    pub fn err(&mut self, message: &str) {
        self.log_message(LogType::Error, message);
    }



    fn log_message(&mut self, log_type: LogType, message: &str) {

        Self::stderr_log(log_type, message, self.colored_terminal);
        match writeln!(self.file_writer, "{}{message}", log_type.to_str(false)) {
            Ok(_) => self.has_already_file_writing_failed = false,
            Err(error) => if !self.has_already_file_writing_failed {
                eprintln!("{}Logging to file failed - {error}.", LogType::Error.to_str(false));
                self.has_already_file_writing_failed = true;
            },
        }

    }

    fn stderr_log(log_type: LogType, message: &str, colored_terminal: bool) {

        eprintln!("{}{message}", log_type.to_str(colored_terminal));

    }

}



#[derive(Clone, Copy)]
enum LogType {

    Information,
    Warning,
    Error,

}

impl LogType {

    fn to_str(self, colored: bool) -> &'static str {

        if colored {

            match self {
                LogType::Information => "\x1b[0;1;34mInformation\x1b[0;1m:\x1b[0m ",
                LogType::Warning => "\x1b[0;1;33mWarning\x1b[0;1m:\x1b[0m ",
                LogType::Error => "\x1b[0;1;31mError\x1b[0;1m:\x1b[0m ",
            }
        } else {

            match self {
                LogType::Information => "Information: ",
                LogType::Warning => "Warning: ",
                LogType::Error => "Error: ",
            }
        }
    }

}
