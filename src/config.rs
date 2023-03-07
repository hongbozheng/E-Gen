use crate::*;

/* global variables */
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Debug;
/// percentage of max # of OS threads
pub static mut THREAD_PCT: f32 = 1.00;
/// max # of threads can be used (not max # of OS threads)
pub static mut MAX_NUM_THREADS: u32 = 0;
/// max rewrite str len
pub static mut MAX_RW_LEN: u8 = 25;
/// context-sensitive grammar flag (csg flag)
pub static mut CSG: bool = true;
/// suppress meaningless rewrite rules (e.g. * 1, + 0, pow 1)
pub static SUPPRESS: bool = true;
/// maximum limit of rewrite rule frequency
pub static FREQ_MAX: u8 = 1;
/* may put max_rw_len here as global ver */