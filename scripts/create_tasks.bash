#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "[USAGE]: $0 <num_tasks>"
    exit 1
fi

num_tasks=$1

for ((i = 0; i < num_tasks; i++)); do
    filename="task_${i}.sbatch"

    cat > $filename <<EOM
#!/bin/sh
#
#SBATCH --time=04:00:00                                                                                                          
#SBATCH --nodes=1
#SBATCH --ntasks-per-node=16
#SBATCH --job-name=eee
#SBATCH --partition=secondary
#SBATCH --output=eeg.o%j
#SBATCH --error=eeg.e%j
#SBATCH --mail-user=EMAIL_ADDR@illinois.edu
#SBATCH --mail-type=BEGIN,END
#
# End of embedded SBATCH options
#
 
# Run the hello world executable (a.out)
cd eeg/src/
cargo run --bin egg -- -f -n 10 -l 12 -m 12 -t 300 -i "../data/inv_${i}.txt" -o "../data/inv_${i}_gen.txt"
EOM
done
