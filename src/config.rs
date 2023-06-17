use crate::*;

/* global variables */
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Debug;
/// percentage of max # of OS threads
pub static mut THREAD_PCT: f64 = 0.80;
/// maximum expression rewrite length
pub static mut MAX_RW_LEN: u8 = 25;
/// exhaustive extraction flag
pub static mut CSG: bool = true;
/// suppress meaningless rewrite rules (e.g. * 1, pow 1, + 0)
pub static SUPPRESS: bool = true;
/// maximum limit of rewrite rule frequency
pub static FREQ_MAX: u8 = 1;