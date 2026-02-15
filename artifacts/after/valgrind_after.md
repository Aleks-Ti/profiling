# valgrind

Команда:

```bash
valgrind --leak-check=full cargo test --tests
```

Вывод:

```log
==212373== Memcheck, a memory error detector
==212373== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==212373== Using Valgrind-3.22.0 and LibVEX; rerun with -h for copyright info
==212373== Command: cargo test --tests
==212373== 
   Compiling broken-app v0.1.0 (/home/alti/dev/rust/profiling)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.14s
     Running unittests src/lib.rs (target/debug/deps/broken_app-1fcfe3268fbb11df)

running 9 tests
test algo::tests::test_slow_dedup_correctness ... ok
test algo::tests::test_slow_fib_correctness ... ok
test algo::tests::test_slow_fib_performance_regression ... ok
test tests::test_average_positive_regression ... ok
test tests::test_normalize_regression ... ok
test tests::test_leak_buffer_regression ... ok
test tests::test_use_after_free_no_ub_and_correct_result ... ok
test tests::test_sum_even_regression ... ok
test concurrency::tests::test_race_increment_regression ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/demo.rs (target/debug/deps/demo-48b0b01bb0836e32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration.rs (target/debug/deps/integration-f368c4363e590c2a)

running 6 tests
test averages_only_positive ... ok
test fib_small_numbers ... ok
test sums_even_numbers ... ok
test dedup_preserves_uniques ... ok
test counts_non_zero_bytes ... ok
test normalize_simple ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
