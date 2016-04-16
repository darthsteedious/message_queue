extern crate log;
extern crate time;

use std::error::Error;
use std::io::{Write};
use std::fs::{File, OpenOptions};
use log::{LogLevelFilter, LogMetadata, LogLevel, LogRecord, SetLoggerError};

pub struct FileLogger { file: File,  }

const TIME_FORMAT: &'static str = "%m-%d-%Y %H:%M:%S";

impl FileLogger {
    fn time_stamp() -> String {
        let tm = time::now();

        time::strftime(TIME_FORMAT, &tm).unwrap()
    }
    pub fn init(file_name: &str) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            let result = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(file_name);


            let file = result.expect("Opening log file failed!");

            match write!(&file, "---------- LOG INITIALIZING {} ----------\n", FileLogger::time_stamp())
                        .expect("Error configuring logger!") {
                _ => {
                    max_log_level.set(LogLevelFilter::Debug);

                    Box::new(FileLogger { file: file })
                }
            }
        })
    }
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let write_result = write!(&self.file,
                "{}\t{}\t --- {}\n", record.level(), FileLogger::time_stamp(), record.args());

            if let Err(e) = write_result {
                println!("Error writing to log file. -- {}", e.description());
            }
        }
    }
}
