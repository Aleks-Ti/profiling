# profiling

## [before] cargo +nightly miri test

## [after] valgrind --leak-check=full cargo test --tests

## [before] RUSTFLAGS="-Zsanitizer=address" cargo +nightly test

### RUSTFLAGS="-Zsanitizer=address -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test

## [after] RUSTFLAGS="-Zsanitizer=address" cargo +nightly test

## [before] RUSTFLAGS="-Zsanitizer=thread -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test

### cargo +nightly build --release -Zbuild-std --target x86_64-unknown-linux-gnu
