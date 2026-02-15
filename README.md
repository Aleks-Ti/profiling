# profiling

## miri

Команда:

```bash
cargo +nightly miri test
```

## valgrind

Команда:

```bash
valgrind --leak-check=full cargo test --tests
```

## sanitizers

### sanitizer address

Команда:

```bash
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test
```

Система может ругаться на всякие флаги для ABI, потому можно применить такую команду:

```bash
RUSTFLAGS="-Zsanitizer=address -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test
```

### sanitizer thread

```bash
RUSTFLAGS="-Zsanitizer=thread -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test
```

### cargo +nightly build --release -Zbuild-std --target x86_64-unknown-linux-gnu
