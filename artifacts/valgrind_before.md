# valgrind

```log
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
