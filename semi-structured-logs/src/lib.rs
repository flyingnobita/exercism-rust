// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    // match level {
    //     LogLevel::Info => info(message),
    //     LogLevel::Warning => warn(message),
    //     LogLevel::Error => error(message),
    //     _ => "Unknown LogLevel".to_owned(),
    // }
    let level = format!("{:?}", level).to_uppercase();
    format!("[{}]: {}", level, message)
}
pub fn info(message: &str) -> String {
    // let mut msg: String = "[INFO]: ".to_owned();
    // msg.push_str(message);
    // msg
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    // let mut msg: String = "[WARNING]: ".to_owned();
    // msg.push_str(message);
    // msg
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    // let mut msg: String = "[ERROR]: ".to_owned();
    // msg.push_str(message);
    // msg
    log(LogLevel::Error, message)
}
