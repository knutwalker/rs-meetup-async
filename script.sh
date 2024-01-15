#!/usr/bin/env bash

trap 'read' DEBUG
set -eux

cargo --quiet run --release --bin sync 2000
cargo --quiet run --release --bin parallel 16000
env RAYON_RS_NUM_CPUS=32 cargo --quiet run --release --bin parallel -- 64000
env RAYON_RS_NUM_CPUS=64 cargo --quiet run --release --bin parallel -- 64000
cargo --quiet run --release --bin concurrent -- 2000
cargo --quiet run --release --bin concurrent_working -- 2000
cargo --quiet run --release --bin concurrent_working -- 20000
cargo --quiet run --release --bin concurrent_limit -- 20000
env RAYON_RS_NUM_CPUS=64    cargo --quiet run --release --bin concurrent_limit -- 20000
env RAYON_RS_NUM_CPUS=64    cargo --quiet run --release --bin parallel -- 20000
env RAYON_RS_NUM_CPUS=128   cargo --quiet run --release --bin parallel -- 20000
# env RAYON_RS_NUM_CPUS=1024  cargo --quiet run --release --bin parallel -- 20000
# env RAYON_RS_NUM_CPUS=10240 cargo --quiet run --release --bin parallel -- 20000
env RAYON_RS_NUM_CPUS=1024  cargo --quiet run --release --bin concurrent_limit -- 20000
# env RAYON_RS_NUM_CPUS=10240 cargo --quiet run --release --bin concurrent_limit -- 20000
# env RAYON_RS_NUM_CPUS=128   cargo --quiet run --release --bin concurrent_limit -- 20000
env RAYON_RS_NUM_CPUS=128   cargo --quiet run --release --bin concurrent_stream -- 20000
env RAYON_RS_NUM_CPUS=128   cargo --quiet run --release --bin concurrent_parallel -- 200000
env RAYON_RS_NUM_CPUS=1024  cargo --quiet run --release --bin concurrent_parallel -- 200000
