from typing import Optional

import config
from datetime import datetime
from enum import Enum


class LogLevel(Enum):
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


def timestamp() -> str:
    time = datetime.now().strftime(format="%Y-%m-%d %H:%M:%S")
    return time


def log_msg(level: Optional[str], *args, **kwargs) -> None:
    ts = timestamp()
    if level:
        print(f"[{ts}] [{level}]: ", end="")
    else:
        print(f"[{ts}] ", end="")
    ts = timestamp()
    print(f"[{ts}] ", end="")
    print(*args, **kwargs)
    return


def log_trace(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.TRACE:
        ts = timestamp()
        print(f"[{ts}] [TRACE]: ", end="")
        print(*args, **kwargs)
    return


def log_debug(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.DEBUG:
        ts = timestamp()
        print(f"[{ts}] [DEBUG]: ", end="")
        print(*args, **kwargs)
    return


def log_info(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.INFO:
        ts = timestamp()
        print(f"[{ts}] [INFO]: ", end="")
        print(*args, **kwargs)
    return


def log_warn(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.WARN:
        ts = timestamp()
        print(f"[{ts}] [WARN]: ", end="")
        print(*args, **kwargs)
    return


def log_error(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.ERROR:
        ts = timestamp()
        print(f"[{ts}] [ERROR]: ", end="")
        print(*args, **kwargs)
    return


def log_fatal(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.FATAL:
        ts = timestamp()
        print(f"[{ts}] [FATAL]: ", end="")
        print(*args, **kwargs)
    return


def log_trace_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.TRACE:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return


def log_debug_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.DEBUG:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return


def log_info_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.INFO:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return


def log_warn_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.WARN:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return


def log_error_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.ERROR:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return


def log_fatal_raw(*args, **kwargs) -> None:
    if config.LOG_LEVEL >= LogLevel.FATAL:
        ts = timestamp()
        print(f"[{ts}] ", end="")
        print(*args, **kwargs)
    return
