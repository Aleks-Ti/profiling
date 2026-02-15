# miri

Команда:

```bash
cargo +nightly miri test
```

Вывод:

```log
cargo +nightly miri test
   Compiling broken-app v0.1.0 (/home/alti/dev/rust/profiling)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/broken_app-8710f211e5fc635a)

running 9 tests
test algo::tests::test_slow_dedup_correctness ... ok
test algo::tests::test_slow_fib_correctness ... ok
test algo::tests::test_slow_fib_performance_regression ... ok
test concurrency::tests::test_race_increment_regression ... ok
test tests::test_average_positive_regression ... ok
test tests::test_leak_buffer_regression ... ok
test tests::test_normalize_regression ... ok
test tests::test_sum_even_regression ... ok
test tests::test_use_after_free_no_ub_and_correct_result ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 21.38s

     Running unittests src/bin/demo.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/demo-877f2d04eadb8ffe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/integration.rs (target/miri/x86_64-unknown-linux-gnu/debug/deps/integration-2e7be9085bf6b1f8)

running 6 tests
test averages_only_positive ... ok
test counts_non_zero_bytes ... ok
test dedup_preserves_uniques ... ok
test fib_small_numbers ... ok
test normalize_simple ... ok
test sums_even_numbers ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.74s

   Doc-tests broken_app

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
