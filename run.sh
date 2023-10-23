#!/bin/bash

# Inputs
deepest_tree=20

# Build the cargo package
cargo build --release

# Loop through and apply our code
for i in $(seq 2 $deepest_tree)
do 
    echo "==============================================================="
    if [[ $i == 2 ]]
    then 
        echo ./target/release/minimax-egg -d $i --init_csv
        echo "tree_depth = " $i
        ./target/release/minimax-egg -d $i --init_csv
    else
        echo ./target/release/minimax-egg -d $i 
        echo "tree_depth = " $i
        ./target/release/minimax-egg -d $i 
    fi
done

# Plot the performance using Python
./src/plot.py