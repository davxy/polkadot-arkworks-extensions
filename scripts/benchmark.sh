#!/bin/bash
set -e

cargo build --release -p ark-node --features runtime-benchmarks,small-ring

run="cargo run --release -p ark-node --features runtime-benchmarks,small-ring -- benchmark pallet"

pallet=$1
extrinsic=$2

# How many repetitions of each benchmark should be run
REPEAT=${REPEAT:-3}
# How many samples we should take across the variable components
STEPS=${STEPS:-30}

results_dir="./results"
mkdir -p "$results_dir"


if [[ $pallet == "all" ]]; then
    pallet="*"
fi

if [[ $pallet == "" ]]; then
    echo "Usage ./benchmark.sh <pallet> <extrinsic>"
    echo ""
    echo "Available pallets:"
    $run --list
    exit 1
fi

if [[ $extrinsic == "all" ]]; then
    extrinsic="*"
fi

if [[ $extrinsic == "" ]]; then
    echo "Usage ./benchmark.sh <pallet> <extrinsic>"
    echo ""
    echo "Available extrinsics for '$pallet':"
    $run --pallet "$pallet" --list | grep "$pallet" | awk '{ print "- " $2 }'
    echo ""
    echo "Use 'all' to run all benchmarks"
    exit 1
fi

export RUSTFLAGS="-C target-cpu=native"

$run \
  --chain="dev" \
  --pallet="$pallet" \
  --extrinsic="$extrinsic" \
  --steps="$STEPS" \
  --repeat="$REPEAT" \
  --no-storage-info \
  --disable-proof-recording \
  --no-median-slopes \
  --json-file=$results_dir/results.json \
  --output=$results_dir/weights.rs

