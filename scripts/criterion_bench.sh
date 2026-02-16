#!/usr/bin/env bash
set -euo pipefail

cargo bench --bench criterion > artifacts/before/criterion_before.txt
