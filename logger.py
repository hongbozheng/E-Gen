import enum
import config

class LogLevel(enum.Enum):
    all = 6
    trace = 5
    debug = 4
    info = 3
    warn = 2
    error = 1
    fatal = 0
    off = -1

    def __ge__(self, other):
        if self.__class__ is other.__class__:
            return self.value >= other.value
        return NotImplemented

    def __gt__(self, other):
        if self.__class__ is other.__class__:
            return self.value > other.value
        return NotImplemented

    def __le__(self, other):
        if self.__class__ is other.__class__:
            return self.value <= other.value
        return NotImplemented

    def __lt__(self, other):
        if self.__class__ is other.__class__:
            return self.value < other.value
        return NotImplemented

def log_trace(*args, **kwargs):
    if config.log_level >= LogLevel.trace:
        print("[TRACE]: ", end = "")
        print(*args, **kwargs)
    return

def log_debug(*args, **kwargs):
    if config.log_level >= LogLevel.debug:
        print("[DEBUG]: ", end = "")
        print(*args, **kwargs)
    return

def log_info(*args, **kwargs):
    if config.log_level >= LogLevel.info:
        print("[INFO]: ", end = "")
        print(*args, **kwargs)
    return

def log_warn(*args, **kwargs):
    if config.log_level >= LogLevel.warn:
        print("[WARN]: ", end = "")
        print(*args, **kwargs)
    return

def log_error(*args, **kwargs):
    if config.log_level >= LogLevel.error:
        print("[ERROR]: ", end = "")
        print(*args, **kwargs)
    return

def log_fatal(*args, **kwargs):
    if config.log_level >= LogLevel.fatal:
        print("[FATAL]: ", end = "")
        print(*args, **kwargs)
    return

def log_trace_raw(*args, **kwargs):
    if config.log_level >= LogLevel.trace:
        print(*args, **kwargs)
    return

def log_debug_raw(*args, **kwargs):
    if config.log_level >= LogLevel.debug:
        print(*args, **kwargs)
    return

def log_info_raw(*args, **kwargs):
    if config.log_level >= LogLevel.info:
        print(*args, **kwargs)
    return

def log_warn_raw(*args, **kwargs):
    if config.log_level >= LogLevel.warn:
        print(*args, **kwargs)
    return

def log_error_raw(*args, **kwargs):
    if config.log_level >= LogLevel.error:
        print(*args, **kwargs)
    return

def log_fatal_raw(*args, **kwargs):
    if config.log_level >= LogLevel.fatal:
        print(*args, **kwargs)
    return