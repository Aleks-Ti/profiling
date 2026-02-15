#!/usr/bin/env bash
set -euo pipefail

# Пример профилирования (Linux, perf). Настройте под свою систему.
cargo build --release
perf record -g --call-graph dwarf ./target/release/demo sleep 10
perf report --stdio > artifacts/perf_before.txt
