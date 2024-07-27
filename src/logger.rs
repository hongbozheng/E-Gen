use crate::*;
use chrono::Local;
use std::cmp::Ordering;

/// log level for the environment
pub enum LogLevel {
    /// All -> print everything
    All = 6,
    /// Trace -> verbose (extraction details)
    Trace = 5,
    /// Debug -> information of all var
    Debug = 4,
    /// Info -> information of progress
    Info = 3,
    /// Warn -> warning(s)
    Warn = 2,
    /// Error -> error(s)
    Error = 1,
    /// Fatal -> fatal error(s)
    Fatal = 0,
    /// Off -> turn off printing
    Off = -1
}

/* implement PartialEq Trait for LogLevel */
impl PartialEq for LogLevel {
    /// ## function to check equal
    /// ## Argument
    /// * `self` - LogLevel set by user
    /// * `other` - other LogLevel
    /// ## Return
    /// * `bool` - 2 loglevels are equal or not
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogLevel::All, LogLevel::All) => true,
            (LogLevel::Trace, LogLevel::Trace) => true,
            (LogLevel::Debug, LogLevel::Debug) => true,
            (LogLevel::Info, LogLevel::Info) => true,
            (LogLevel::Warn, LogLevel::Warn) => true,
            (LogLevel::Error, LogLevel::Error) => true,
            (LogLevel::Fatal, LogLevel::Fatal) => true,
            (LogLevel::Off, LogLevel::Off) => true,
            (_, _) => false,
        }
    }
}

/* implement PartialOrd Trait for LogLevel */
impl PartialOrd for LogLevel {
    /// ## funtion to compare log level set by user with other log level
    /// ## Argument
    /// * `self` - LogLevel set by user
    /// * `other` - other LogLevel
    /// ## Return
    /// * `Option<Ordering>` enum Option
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LogLevel::All, _) => Some(Ordering::Greater),
            (LogLevel::Trace, LogLevel::All) => Some(Ordering::Less),
            (LogLevel::Trace, _) => Some(Ordering::Greater),
            (LogLevel::Debug, LogLevel::All) => Some(Ordering::Less),
            (LogLevel::Debug, LogLevel::Trace) => Some(Ordering::Less),
            (LogLevel::Debug, _) => Some(Ordering::Greater),
            (LogLevel::Info, LogLevel::All) => Some(Ordering::Less),
            (LogLevel::Info, LogLevel::Trace) => Some(Ordering::Less),
            (LogLevel::Info, LogLevel::Debug) => Some(Ordering::Less),
            (LogLevel::Info, _) => Some(Ordering::Greater),
            (LogLevel::Warn, LogLevel::All) => Some(Ordering::Less),
            (LogLevel::Warn, LogLevel::Trace) => Some(Ordering::Less),
            (LogLevel::Warn, LogLevel::Debug) => Some(Ordering::Less),
            (LogLevel::Warn, LogLevel::Info) => Some(Ordering::Less),
            (LogLevel::Warn, _) => Some(Ordering::Greater),
            (LogLevel::Error, LogLevel::Error) => Some(Ordering::Equal),
            (LogLevel::Error, LogLevel::Fatal) => Some(Ordering::Greater),
            (LogLevel::Error, LogLevel::Off) => Some(Ordering::Greater),
            (LogLevel::Error, _) => Some(Ordering::Less),
            (LogLevel::Fatal, LogLevel::Fatal) => Some(Ordering::Equal),
            (LogLevel::Fatal, LogLevel::Off) => Some(Ordering::Greater),
            (LogLevel::Fatal, _) => Some(Ordering::Less),
            (LogLevel::Off, LogLevel::Off) => Some(Ordering::Equal),
            (LogLevel::Off, _) => Some(Ordering::Less),
        }
    }
}

/// ## function to get current timestamp
/// ## Argument
/// * `None`
/// ## Return
/// * `time` - timestamp in String
pub fn timestamp() -> String {
    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    return time
}

/// ## function to print log level - trace
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_trace(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Trace { print!("[{}] [TRACE]: {}", ts, str); }
}

/// ## function to print log level - debug
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_debug(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Debug { print!("[{}] [DEBUG]: {}", ts, str); }
}

/// ## function to print log level - info
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_info(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Info { print!("[{}] [INFO]: {}", ts, str); }
}

/// ## function to print log level - warn
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_warn(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Warn { print!("[{}] [WARN]: {}", ts, str); }
}

/// ## function to print log level - error
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_error(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Error { print!("[{}] [ERROR]: {}", ts, str); }
}

/// ## function to print log level - fatal
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_fatal(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Fatal { print!("[{}] [FATAL]: {}", ts, str); }
}

/// ## function to print log level - trace
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_trace_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Trace { print!("[{}] {}", ts, str); }
}

/// ## function to print log level - debug
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_debug_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Debug { print!("[{}] {}", ts, str); }
}

/// ## function to print log level - info
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_info_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Info { print!("[{}] {}", ts, str); }
}

/// ## function to print log level - warn
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_warn_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Warn { print!("[{}] {}", ts, str); }
}

/// ## function to print log level - error
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_error_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Error { print!("[{}] {}", ts, str); }
}

/// ## function to print log level - fatal
/// ## Argument
/// * `str` - msg to print
/// ## Return
/// * `None`
pub fn log_fatal_raw(str: &str) {
    let ts = timestamp();
    if LOG_LEVEL >= LogLevel::Fatal { print!("[{}] {}", ts, str); }
}
