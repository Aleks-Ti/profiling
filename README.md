# profiling

## [before] cargo +nightly miri test

```bash
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:60:15
   |
60 |     let val = *raw;
   |               ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
  --> src/lib.rs:57:1
   |
57 | pub unsafe fn use_after_free() -> i32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

warning[E0133]: call to unsafe function `std::boxed::Box::<T>::from_raw` is unsafe and requires unsafe block
  --> src/lib.rs:61:10
   |
61 |     drop(Box::from_raw(raw));
   |          ^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: consult the function's documentation for information on how to avoid undefined behavior

warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
  --> src/lib.rs:62:11
   |
62 |     val + *raw
   |           ^^^^ dereference of raw pointer
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 6 tests
test averages_only_positive ... FAILED
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... error: Undefined Behavior: `assume` called with `false`
  --> src/lib.rs:11:22
   |
11 |             let v = *values.get_unchecked(idx);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: this is on thread `sums_even_numbe`
   = note: stack backtrace:
           0: broken_app::sum_even
               at src/lib.rs:11:22: 11:47
           1: sums_even_numbers
               at tests/integration.rs:7:16: 7:31
           2: sums_even_numbers::{closure#0}
               at tests/integration.rs:4:23: 4:23

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/alti/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo-miri runner /home/alti/dev/rust/profiling/target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8` (exit status: 1)
note: test exited abnormally; to see the full output pass --no-capture to the harness.
```

## [after] cargo +nightly miri test

## [before] valgrind --leak-check=full cargo test --tests

```bash
warning: `broken-app` (lib test) generated 3 warnings (3 duplicates)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.53s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1fcfe3268fbb11df)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-48b0b01bb0836e32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-f368c4363e590c2a)

running 6 tests
test averages_only_positive ... 
thread 'sums_even_numbers' (574106) panicked at src/lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
thread caused non-unwinding panic. aborting.
error: test failed, to rerun pass `--test integration`

Caused by:
  process didn't exit successfully: `/home/alti/dev/rust/profiling/target/debug/deps/integration-f368c4363e590c2a` (signal: 6, SIGABRT: process abort signal)
```

## [after] valgrind --leak-check=full cargo test --tests

## [before] RUSTFLAGS="-Zsanitizer=address" cargo +nightly test

### RUSTFLAGS="-Zsanitizer=address -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test

```bash
Some errors have detailed explanations: E0277, E0432, E0463.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `zerocopy` (lib) due to 136 previous errors
```

## [after] RUSTFLAGS="-Zsanitizer=address" cargo +nightly test

## [before] RUSTFLAGS="-Zsanitizer=thread -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test

### cargo +nightly build --release -Zbuild-std --target x86_64-unknown-linux-gnu
