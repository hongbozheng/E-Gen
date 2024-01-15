#!/bin/bash

# Get the number of sbatch files
num_tasks=$(ls -1 task_*.sbatch | wc -l)

for ((i = 0; i < num_tasks; i++)); do
    sbatch task_"$i".sbatch
done
