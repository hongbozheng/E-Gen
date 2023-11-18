use crate::*;

/* global variables */
/// optimized extraction flag
pub static mut OPTIMIZED: bool = false;
/// number of equivalent expressions
pub static mut NUM_EQUIV_EXPRS: u8 = 10;
/// token limit
pub static mut TOKEN_LIMIT: u8 = 8;
/// time limit in sec
pub static mut TIME_LIMIT: u16 = 350;
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
/// suppress meaningless rewrite rules (e.g. * 1, pow 1, + 0)
pub static SUPPRESS: bool = true;