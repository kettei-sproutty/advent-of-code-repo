# Advent of Code

## Repository structure

To run a specific binary, run:

```sh
cargo run -p year-{year} --bin day-{day}
```

Example:

```sh
cargo run -p year-2024 --bin day-01
```

## Create new day

```sh
cargo run -p cli
```

this will inquire `year` and `day`

Example:

> Year:  2024
> Day:  3

and generate the scaffold.

## Update benchmark

```sh
cargo bench --workspace --all-features | grep -A 100 'benchmarks' > benchmarks_output/cargo_bench_output.txt
```
