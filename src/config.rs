use crate::*;

/* global variables */
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
/// percentage of max # of OS threads
pub static mut THD_PCT: f64 = 0.80;
/// maximum expression rewrite length
pub static mut MAX_NUM_TOKEN: u8 = 35;
/// exhaustive extraction flag
pub static mut EXHAUSTIVE: bool = false;
/// suppress meaningless rewrite rules (e.g. * 1, pow 1, + 0)
pub static SUPPRESS: bool = true;