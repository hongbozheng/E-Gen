use crate::*;
use std::cmp::Ordering;

/* enumerate log level */
pub enum LogLevel {
    All = 6,
    Trace = 5,
    Debug = 4,
    Info = 3,
    Warn = 2,
    Error = 1,
    Fatal = 0,
    Off = -1
}

/* implement PartialEq Trait for LogLevel */
impl PartialEq for LogLevel {
    /// ## function to check equal
    /// ## Argument
    /// * `self` - LogLevel set by user
    /// * `other` - other LogLevel
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        *self == *other
    }

    /// ## function to check unequal
    /// ## Argument
    /// * `self` - LogLevel set by user
    /// * `other` - other LogLevel
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        *self != *other
    }
}

/* implement PartialOrd Trait for LogLevel */
impl PartialOrd for LogLevel {
    /// ## funtion to compare log level set by user with other log level
    /// ## Argument
    /// * `self` - LogLevel set by user
    /// * `other` - other LogLevel
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

/// ## function to print log level - trace
/// ## Argument
/// * `str` - msg to print
pub fn log_trace(str: &str) {
    if LOG_LEVEL >= LogLevel::Trace { println!("[TRACE]: {}", str); }
}

/// ## function to print log level - debug
/// ## Argument
/// * `str` - msg to print
pub fn log_debug(str: &str) {
    if LOG_LEVEL >= LogLevel::Debug { println!("[DEBUG]: {}", str); }
}

/// ## function to print log level - info
/// ## Argument
/// * `str` - msg to print
pub fn log_info(str: &str) {
    if LOG_LEVEL >= LogLevel::Info { println!("[INFO]: {}", str); }
}

/// ## function to print log level - warn
/// ## Argument
/// * `str` - msg to print
pub fn log_warn(str: &str) {
    if LOG_LEVEL >= LogLevel::Warn { println!("[WARN]: {}", str); }
}

/// ## function to print log level - error
/// ## Argument
/// * `str` - msg to print
pub fn log_error(str: &str) {
    if LOG_LEVEL >= LogLevel::Error { println!("[ERROR]: {}", str); }
}

/// ## function to print log level - fatal
/// ## Argument
/// * `str` - msg to print
pub fn log_fatal(str: &str) {
    if LOG_LEVEL >= LogLevel::Fatal { println!("[FATAL]: {}", str); }
}