#!/bin/bash
# Runs a match between two engines using cutechess-cli to measure ELO difference.

ENGINE1=$1
ENGINE2=$2

if [ -z "$ENGINE1" ] || [ -z "$ENGINE2" ]; then
    echo "Usage: ./run_match.sh <path_to_new_engine> <path_to_old_engine>"
    echo "Example: ./run_match.sh ../target/release/chess-engine ./chess-engine-v051"
    exit 1
fi

if ! command -v cutechess-cli &> /dev/null; then
    echo "cutechess-cli could not be found. Please install it to run matches."
    exit 1
fi

echo "Starting 100-game match between $ENGINE1 and $ENGINE2..."
echo "Time Control: 10 seconds + 0.1s increment"

cutechess-cli -engine cmd="$ENGINE1" name="NewVersion" \
              -engine cmd="$ENGINE2" name="OldVersion" \
              -each tc=10+0.1 \
              -rounds 50 -games 2 -repeat \
              -concurrency 4 \
              -ratinginterval 10
