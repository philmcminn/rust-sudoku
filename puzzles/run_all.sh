#!/usr/bin/env bash

cd ..
echo "Building..."
cargo build --release
sudoku=./target/release/sudoku

function run_all_in_directory {
    echo "Running all in $1..."
    for f in $1/*.txt
    do
        if [ "$f" != "$1/empty.txt" ]
        then
            $sudoku $f
        fi
    done
}

run_all_in_directory "./puzzles/4x4"
run_all_in_directory "./puzzles/9x9"
run_all_in_directory "./puzzles/16x16"
run_all_in_directory "./puzzles/25x25"
