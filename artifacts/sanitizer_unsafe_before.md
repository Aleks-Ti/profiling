# RUSTFLAGS="-Zsanitizer=address -Cunsafe-allow-abi-mismatch=sanitizer" cargo +nightly test

```bash
# слишком огромный вывод, потом  ниже коротко
Some errors have detailed explanations: E0277, E0432, E0463.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `zerocopy` (lib) due to 136 previous errors
```
