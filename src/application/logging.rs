use anyhow::Result;
use flexi_logger::{
    detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, LoggerHandle, Naming,
    WriteMode,
};

pub fn start_logger() -> LoggerHandle {
    try_start_logger().expect("Failed to start logger")
}

pub fn try_start_logger() -> Result<LoggerHandle> {
    /*
    Error
    Warn
    Info
    Debug
    Trace
    */
    let level = std::env::var("LOGLEVEL").unwrap_or(String::from("Info"));
    let logger_handle = Logger::try_with_str(level)?
        .log_to_file(FileSpec::default().directory("logs"))
        .write_mode(WriteMode::Direct)
        .duplicate_to_stdout(Duplicate::Info)
        .format_for_files(detailed_format)
        .rotate(
            Criterion::AgeOrSize(Age::Day, 1024 * 1024 * 25),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(10),
        )
        .print_message()
        .start()?;

    Ok(logger_handle)
}
