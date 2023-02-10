use crate::*;

/// log level for the entire environment
pub static LOG_LEVEL: LogLevel = LogLevel::Debug;
/// suppress meaningless rewrite rules (e.g. * 1, + 0, pow 1)
pub static SUPPRESS: bool = true;
/// maximum limit of rewrite rule frequency
pub static FREQ_MAX: u8 = 1;
/* may put max_rw_len here as global ver */