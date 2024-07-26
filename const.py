#!/usr/bin/env python3


import argparse
import config
import logger
import random
import re


def randint(a: int, b: int) -> str:
    return str(random.randint(a=a, b=b))


def replace_c(seed: int, input_filepath: str, output_filepath: str) -> None:
    random.seed(a=seed)

    input_file = open(file=input_filepath, mode='r')
    output_file = open(file=output_filepath, mode='w')

    for line in input_file:
        digit = randint(a=0, b=9)
        expr = re.sub(pattern=r"\bc\b", repl=str(digit), string=line)
        digit = randint(a=0, b=9)
        expr = re.sub(pattern=r"\b\d{2,}", repl=str(digit), string=expr)
        digit = randint(a=0, b=9)
        expr = re.sub(pattern=r"-\d{2,}", repl=str(digit), string=expr)

        output_file.write(expr)

    input_file.close()
    output_file.close()

    return


def main():
    parser = argparse.ArgumentParser(prog="const", description="Replace 'c' with constant values from 0-9")
    parser.add_argument("--input_filepath", "-i", required=True, help="intput_filepath")
    parser.add_argument("--output_filepath", "-o", required=True, help="output_filepath")

    arg = parser.parse_args()
    input_filepath = arg.input_filepath
    output_filepath = arg.output_filepath

    logger.log_info(f"Replacing character 'c' in '{input_filepath}' file with random constant values from 0-9...")
    replace_c(seed=config.SEED, input_filepath=input_filepath, output_filepath=output_filepath)
    logger.log_info(f"Finish replacing, results are saved in '{output_filepath}' file.")

    return


if __name__ == '__main__':
    main()
    