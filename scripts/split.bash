#!/bin/bash

# Check for the correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "[USAGE]: $0 <input_file> <exprs_per_file>"
    exit 1
fi

input_file="$1"
num_exprs_per_file="$2"

# Loop to split the file
counter=0

# Counter for the output files
file_counter=0

while IFS= read -r line; do
    # Output each line to the appropriate file
    echo "$line" >> "${input_file%.*}_$file_counter.txt"
    ((counter++))

    # Check if the counter reaches the desired num_exprs_per_file
    if [ "$counter" -eq "$num_exprs_per_file" ]; then
        # Reset the counter and increment the file counter
        counter=0
        ((file_counter++))
    fi
done < "$input_file"
