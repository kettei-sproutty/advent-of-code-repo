#! /bin/bash

if [ "$1" == "--write" ]; then
  cargo bench --workspace --all-features | grep -A 100 'benchmarks' > benchmarks_output/cargo_bench_output.txt
  exit 0
fi

cargo bench --workspace --all-features | grep -A 100 'benchmarks'
