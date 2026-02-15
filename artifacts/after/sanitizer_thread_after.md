# sanitizer=thread

Команда:

```bash
RUSTFLAGS="-Zsanitizer=thread -C unsafe-allow-abi-mismatch=sanitizer" cargo +nightly test
```

Вывод:

Ругается только на библу zerocopy с бесконечными unsafe

```log
Some errors have detailed explanations: E0277, E0432, E0463.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `zerocopy` (lib) due to 136 previous errors
```
