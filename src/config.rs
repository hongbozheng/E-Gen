use crate::*;

/// percentage of max # of threads
pub static MAX_NUM_THREADS_PCT: f32 = 0.50;
/// max # of threads can be used (not max # of threads of OS)
pub static mut MAX_NUM_THREADS: u32 = 0;
/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Info;
/// max rewrite str len
pub static mut MAX_RW_LEN: u8 = 20;
/// suppress meaningless rewrite rules (e.g. * 1, + 0, pow 1)
pub static SUPPRESS: bool = true;
/// maximum limit of rewrite rule frequency
pub static FREQ_MAX: u8 = 1;
/* may put max_rw_len here as global ver */