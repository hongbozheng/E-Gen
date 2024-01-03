#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "[USAGE]: $0 <line_num> <output_file>"
    exit 1
fi

n_lines="$1"
output_file="$2"

# Check if the output file already exists and prompt before overwriting
if [ -e "$output_file" ]; then
    read -p "File '$output_file' already exists. Do you want to overwrite it? (y/n): " answer
    if [ "$answer" != "y" ]; then
        echo "[INFO]: Operation aborted."
        exit 1
    fi
fi

# Loop through each file and delete the first $line_num lines
for file in *.test; do
    if [ -f "$file" ]; then
	echo "[INFO]: Processing file: $file"
        tail -n +$n_lines "$file" >> "$output_file"
        echo "[INFO]: Deleted the first $((n_lines-1)) lines from $file and appended to $output_file"
    else
        echo "[ERROR]: $file is not a regular file or does not exist."
    fi
done
