use crate::*;

/* global variables */
/// number of processes
pub static mut N_PROC: u8 = 1;
/// percentage of max # of OS threads
pub static mut THD_PCT: f32 = 0.0;
/// optimized extraction flag
pub static mut OPTIMIZED: bool = false;
/// number of equivalent expressions
pub static mut N_EQUIV_EXPRS: u8 = 10;
/// token limit
pub static mut TOKEN_LIMIT: u8 = 8;
/// max token limit
pub static mut MAX_TOKEN_LIMIT: u8 = 12;
/// time limit in sec
pub static mut TIME_LIMIT: u16 = 300;
/// start time
pub static mut START_TIME: Option<Instant> = None;
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
/// suppress meaningless rewrite rules (e.g. * 1, pow 1, + 0)
pub static SUPPRESS: bool = true;
