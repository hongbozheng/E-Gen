import config
import enum


class LogLevel(enum.Enum):
    ALL = 6
    TRACE = 5
    DEBUG = 4
    INFO = 3
    WARN = 2
    ERROR = 1
    FATAL = 0
    OFF = -1

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


def log_trace(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.TRACE:
        print("[TRACE]: ", end="")
        print(*args, **kwargs)
    return


def log_debug(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.DEBUG:
        print("[DEBUG]: ", end="")
        print(*args, **kwargs)
    return


def log_info(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.INFO:
        print("[INFO]: ", end="")
        print(*args, **kwargs)
    return


def log_warn(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.WARN:
        print("[WARN]: ", end="")
        print(*args, **kwargs)
    return


def log_error(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.ERROR:
        print("[ERROR]: ", end="")
        print(*args, **kwargs)
    return


def log_fatal(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.FATAL:
        print("[FATAL]: ", end="")
        print(*args, **kwargs)
    return


def log_trace_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.TRACE:
        print(*args, **kwargs)
    return


def log_debug_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.DEBUG:
        print(*args, **kwargs)
    return


def log_info_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.INFO:
        print(*args, **kwargs)
    return


def log_warn_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.WARN:
        print(*args, **kwargs)
    return


def log_error_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.ERROR:
        print(*args, **kwargs)
    return


def log_fatal_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.FATAL:
        print(*args, **kwargs)
    return