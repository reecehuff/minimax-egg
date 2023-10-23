#!/bin/bash

# Inputs
deepest_tree=16

# Build the cargo package
cargo build --release

# Loop through and apply our code
for i in $(seq 2 $deepest_tree)
do 
    echo "==============================================================="
    if [[ $i == 2 ]]
    then 
        echo ./target/release/minimax-egg $i false false true target/performance.csv
        echo "tree_depth = " $i
        ./target/release/minimax-egg $i false false true target/performance.csv
    else
        echo ./target/release/minimax-egg $i false false false target/performance.csv
        echo "tree_depth = " $i
        ./target/release/minimax-egg $i false false false target/performance.csv
    fi
done

# Plot the performance using Python
./src/plot.py