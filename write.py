from typing import List


def write(
        filepath: str,
        mode: str,
        encoding: str,
        exprs: List[str],
        newline: bool,
) -> None:
    file = open(file=filepath, mode=mode, encoding=encoding)
    for expr in exprs:
        file.write(f"{expr}\n")
    if newline:
        file.write("\n")
    file.close()
    return
