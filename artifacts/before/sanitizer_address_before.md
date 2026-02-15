# sanitizer=address

Команда:

```bash
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test
```

Вывод:

```log
...
153 |     Src: TransmuteFrom<Dst, DV, SV> + ?Sized,
    |          -------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<u128>` to implement `TransmuteFromPtr<NonZero<u128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-8076250168299743113.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<u128>: Read<AA, BecauseImmutable>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:191:33
    |
191 |                   $nonzero::new(n.read_unaligned().into_inner()).is_some()
    |                                   ^^^^^^^^^^^^^^ unsatisfied trait bound
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `Immutable` is not implemented for `wrappers::Unalign<u128>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `Immutable`:
              &T
              &mut T
              ()
              *const T
              *mut T
              NonZero<i128>
              NonZero<i16>
              NonZero<i32>
            and 96 others
note: required for `wrappers::Unalign<u128>` to implement `pointer::invariant::Read<AA, pointer::invariant::BecauseImmutable>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/invariant.rs:217:49
    |
217 | impl<A: Aliasing, T: ?Sized + crate::Immutable> Read<A, BecauseImmutable> for T {}
    |                               ----------------  ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
    |                               |
    |                               unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::read_unaligned`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:531:16
    |
528 |         pub fn read_unaligned<R>(self) -> T
    |                -------------- required by a bound in this associated function
...
531 |             T: Read<I::Aliasing, R>,
    |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::read_unaligned`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-7943583895019466250.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<i128>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<i128>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `FromBytes`:
              ()
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
              AtomicU32
            and 47 others
note: required for `wrappers::Unalign<i128>` to implement `TransmuteFrom<NonZero<i128>, Initialized, Valid>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:326:23
    |
326 | unsafe impl<Src, Dst> TransmuteFrom<Src, Initialized, Valid> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
329 |     Dst: FromBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<i128>` to implement `TransmuteFromPtr<NonZero<i128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |          -------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-10293215829824381509.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<i128>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `pointer::transmute::MutationCompatible<NonZero<i128>, AA, pointer::invariant::Initialized, pointer::invariant::Valid, _>` is not implemented for `wrappers::Unalign<i128>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
note: required for `wrappers::Unalign<i128>` to implement `TryTransmuteFromPtr<NonZero<i128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
154 |     Dst: MutationCompatible<Src, A, SV, DV, R> + SizeEq<Src> + ?Sized,
    |          ------------------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<i128>` to implement `TransmuteFromPtr<NonZero<i128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-10293215829824381509.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<i128>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `IntoBytes` is not implemented for `wrappers::Unalign<i128>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `IntoBytes`:
              ()
              AtomicBool
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
            and 62 others
note: required for `NonZero<i128>` to implement `TransmuteFrom<Unalign<i128>, Valid, Initialized>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:316:23
    |
316 | unsafe impl<Src, Dst> TransmuteFrom<Src, Valid, Initialized> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
317 | where
318 |     Src: IntoBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<i128>` to implement `TryTransmuteFromPtr<NonZero<i128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
153 |     Src: TransmuteFrom<Dst, DV, SV> + ?Sized,
    |          -------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<i128>` to implement `TransmuteFromPtr<NonZero<i128>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-10293215829824381509.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<i128>: Read<AA, BecauseImmutable>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:191:33
    |
191 |                   $nonzero::new(n.read_unaligned().into_inner()).is_some()
    |                                   ^^^^^^^^^^^^^^ unsatisfied trait bound
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `Immutable` is not implemented for `wrappers::Unalign<i128>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `Immutable`:
              &T
              &mut T
              ()
              *const T
              *mut T
              NonZero<i128>
              NonZero<i16>
              NonZero<i32>
            and 96 others
note: required for `wrappers::Unalign<i128>` to implement `pointer::invariant::Read<AA, pointer::invariant::BecauseImmutable>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/invariant.rs:217:49
    |
217 | impl<A: Aliasing, T: ?Sized + crate::Immutable> Read<A, BecauseImmutable> for T {}
    |                               ----------------  ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
    |                               |
    |                               unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::read_unaligned`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:531:16
    |
528 |         pub fn read_unaligned<R>(self) -> T
    |                -------------- required by a bound in this associated function
...
531 |             T: Read<I::Aliasing, R>,
    |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::read_unaligned`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-4961920695348611711.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<usize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<usize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `FromBytes`:
              ()
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
              AtomicU32
            and 47 others
note: required for `wrappers::Unalign<usize>` to implement `TransmuteFrom<NonZero<usize>, Initialized, Valid>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:326:23
    |
326 | unsafe impl<Src, Dst> TransmuteFrom<Src, Initialized, Valid> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
329 |     Dst: FromBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<usize>` to implement `TransmuteFromPtr<NonZero<usize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |          -------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-7550066965432190709.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<usize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `pointer::transmute::MutationCompatible<NonZero<usize>, AA, pointer::invariant::Initialized, pointer::invariant::Valid, _>` is not implemented for `wrappers::Unalign<usize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
note: required for `wrappers::Unalign<usize>` to implement `TryTransmuteFromPtr<NonZero<usize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
154 |     Dst: MutationCompatible<Src, A, SV, DV, R> + SizeEq<Src> + ?Sized,
    |          ------------------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<usize>` to implement `TransmuteFromPtr<NonZero<usize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-7550066965432190709.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<usize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `IntoBytes` is not implemented for `wrappers::Unalign<usize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `IntoBytes`:
              ()
              AtomicBool
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
            and 62 others
note: required for `NonZero<usize>` to implement `TransmuteFrom<Unalign<usize>, Valid, Initialized>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:316:23
    |
316 | unsafe impl<Src, Dst> TransmuteFrom<Src, Valid, Initialized> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
317 | where
318 |     Src: IntoBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<usize>` to implement `TryTransmuteFromPtr<NonZero<usize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
153 |     Src: TransmuteFrom<Dst, DV, SV> + ?Sized,
    |          -------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<usize>` to implement `TransmuteFromPtr<NonZero<usize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-7550066965432190709.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<usize>: Read<AA, BecauseImmutable>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:191:33
    |
191 |                   $nonzero::new(n.read_unaligned().into_inner()).is_some()
    |                                   ^^^^^^^^^^^^^^ unsatisfied trait bound
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `Immutable` is not implemented for `wrappers::Unalign<usize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `Immutable`:
              &T
              &mut T
              ()
              *const T
              *mut T
              NonZero<i128>
              NonZero<i16>
              NonZero<i32>
            and 96 others
note: required for `wrappers::Unalign<usize>` to implement `pointer::invariant::Read<AA, pointer::invariant::BecauseImmutable>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/invariant.rs:217:49
    |
217 | impl<A: Aliasing, T: ?Sized + crate::Immutable> Read<A, BecauseImmutable> for T {}
    |                               ----------------  ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
    |                               |
    |                               unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::read_unaligned`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:531:16
    |
528 |         pub fn read_unaligned<R>(self) -> T
    |                -------------- required by a bound in this associated function
...
531 |             T: Read<I::Aliasing, R>,
    |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::read_unaligned`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-3601162299171880994.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<isize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<isize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `FromBytes`:
              ()
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
              AtomicU32
            and 47 others
note: required for `wrappers::Unalign<isize>` to implement `TransmuteFrom<NonZero<isize>, Initialized, Valid>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:326:23
    |
326 | unsafe impl<Src, Dst> TransmuteFrom<Src, Initialized, Valid> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
329 |     Dst: FromBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<isize>` to implement `TransmuteFromPtr<NonZero<isize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |          -------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-1721111260285083119.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<isize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `pointer::transmute::MutationCompatible<NonZero<isize>, AA, pointer::invariant::Initialized, pointer::invariant::Valid, _>` is not implemented for `wrappers::Unalign<isize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
note: required for `wrappers::Unalign<isize>` to implement `TryTransmuteFromPtr<NonZero<isize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
154 |     Dst: MutationCompatible<Src, A, SV, DV, R> + SizeEq<Src> + ?Sized,
    |          ------------------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<isize>` to implement `TransmuteFromPtr<NonZero<isize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-1721111260285083119.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<isize>: TransmuteFromPtr<..., ..., ..., ..., ...>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:190:39
    |
190 |                   let n = n.transmute::<Unalign<$prim>, invariant::Valid, _>();
    |                             ---------   ^^^^^^^^^^^^^^ unsatisfied trait bound
    |                             |
    |                             required by a bound introduced by this call
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `IntoBytes` is not implemented for `wrappers::Unalign<isize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `IntoBytes`:
              ()
              AtomicBool
              AtomicI16
              AtomicI32
              AtomicI64
              AtomicI8
              AtomicIsize
              AtomicU16
            and 62 others
note: required for `NonZero<isize>` to implement `TransmuteFrom<Unalign<isize>, Valid, Initialized>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:316:23
    |
316 | unsafe impl<Src, Dst> TransmuteFrom<Src, Valid, Initialized> for Dst
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
317 | where
318 |     Src: IntoBytes + ?Sized,
    |          --------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<isize>` to implement `TryTransmuteFromPtr<NonZero<isize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:148:5
    |
148 |     TryTransmuteFromPtr<Src, A, SV, DV, (BecauseMutationCompatible, R)> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
...
153 |     Src: TransmuteFrom<Dst, DV, SV> + ?Sized,
    |          -------------------------- unsatisfied trait bound introduced here
note: required for `wrappers::Unalign<isize>` to implement `TransmuteFromPtr<NonZero<isize>, ..., ..., ..., ...>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/transmute.rs:268:5
    |
268 |     TransmuteFromPtr<Src, A, SV, DV, R> for Dst
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^^^
269 | where
270 |     Dst: TransmuteFrom<Src, SV, DV> + TryTransmuteFromPtr<Src, A, SV, DV, R>,
    |                                       -------------------------------------- unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::transmute`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:382:16
    |
379 |         pub(crate) fn transmute<U, V, R>(self) -> Ptr<'a, U, (I::Aliasing, Unaligned, V)>
    |                       --------- required by a bound in this associated function
...
382 |             U: TransmuteFromPtr<T, I::Aliasing, I::Validity, V, R> + SizeEq<T> + ?Sized,
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::transmute`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-1721111260285083119.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `Unalign<isize>: Read<AA, BecauseImmutable>` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/impls.rs:191:33
    |
191 |                   $nonzero::new(n.read_unaligned().into_inner()).is_some()
    |                                   ^^^^^^^^^^^^^^ unsatisfied trait bound
...
244 | /     unsafe_impl_try_from_bytes_for_nonzero!(
245 | |         NonZeroU8[u8],
246 | |         NonZeroI8[i8],
247 | |         NonZeroU16[u16],
...   |
256 | |         NonZeroIsize[isize]
257 | |     );
    | |_____- in this macro invocation
    |
help: the trait `Immutable` is not implemented for `wrappers::Unalign<isize>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = help: the following other types implement trait `Immutable`:
              &T
              &mut T
              ()
              *const T
              *mut T
              NonZero<i128>
              NonZero<i16>
              NonZero<i32>
            and 96 others
note: required for `wrappers::Unalign<isize>` to implement `pointer::invariant::Read<AA, pointer::invariant::BecauseImmutable>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/invariant.rs:217:49
    |
217 | impl<A: Aliasing, T: ?Sized + crate::Immutable> Read<A, BecauseImmutable> for T {}
    |                               ----------------  ^^^^^^^^^^^^^^^^^^^^^^^^^     ^
    |                               |
    |                               unsatisfied trait bound introduced here
note: required by a bound in `_conversions::<impl pointer::ptr::def::Ptr<'a, T, I>>::read_unaligned`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:531:16
    |
528 |         pub fn read_unaligned<R>(self) -> T
    |                -------------- required by a bound in this associated function
...
531 |             T: Read<I::Aliasing, R>,
    |                ^^^^^^^^^^^^^^^^^^^^ required by this bound in `_conversions::<impl Ptr<'a, T, I>>::read_unaligned`
    = note: the full name for the type has been written to '/home/alti/dev/rust/profiling/target/debug/deps/zerocopy-b888d8fbd1356ac6.long-type-4632255547539466475.txt'
    = note: consider using `--verbose` to print the full type name to the console
    = note: this error originates in the macro `unsafe_impl_try_from_bytes_for_nonzero` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `wrappers::Unalign<T>: Unaligned` is not satisfied
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:515:17
    |
515 |             ptr.bikeshed_recall_aligned()
    |                 ^^^^^^^^^^^^^^^^^^^^^^^ unsatisfied trait bound
    |
help: the trait `Unaligned` is not implemented for `wrappers::Unalign<T>`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
    |
115 | pub struct Unalign<T>(T);
    | ^^^^^^^^^^^^^^^^^^^^^
    = note: Consider adding `#[derive(Unaligned)]` to `wrappers::Unalign<T>`
    = help: the following other types implement trait `Unaligned`:
              ()
              AtomicBool
              AtomicI8
              AtomicU8
              Cell<T>
              NonZero<i8>
              NonZero<u8>
              Option<NonZero<i8>>
            and 12 others
note: required by a bound in `_transitions::<impl pointer::ptr::def::Ptr<'a, T, I>>::bikeshed_recall_aligned`
   --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/pointer/ptr.rs:695:16
    |
691 |         pub(crate) fn bikeshed_recall_aligned(
    |                       ----------------------- required by a bound in this associated function
...
695 |             T: crate::Unaligned,
    |                ^^^^^^^^^^^^^^^^ required by this bound in `_transitions::<impl Ptr<'a, T, I>>::bikeshed_recall_aligned`

error[E0277]: the trait bound `wrappers::Unalign<Self>: FromBytes` is not satisfied
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/lib.rs:4536:35
     |
4536 |             Ok(r) => Ok(Ref::read(&r).into_inner()),
     |                         --------- ^^ unsatisfied trait bound
     |                         |
     |                         required by a bound introduced by this call
     |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<Self>`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
     |
 115 | pub struct Unalign<T>(T);
     | ^^^^^^^^^^^^^^^^^^^^^
     = note: Consider adding `#[derive(FromBytes)]` to `wrappers::Unalign<Self>`
     = help: the following other types implement trait `FromBytes`:
               ()
               AtomicI16
               AtomicI32
               AtomicI64
               AtomicI8
               AtomicIsize
               AtomicU16
               AtomicU32
             and 47 others
note: required by a bound in `r#ref::<impl r#ref::def::Ref<B, T>>::read`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/ref.rs:705:8
     |
 705 |     T: FromBytes,
     |        ^^^^^^^^^ required by this bound in `r#ref::<impl Ref<B, T>>::read`
...
 714 |     pub fn read(r: &Self) -> T {
     |            ---- required by a bound in this associated function

error[E0277]: the trait bound `wrappers::Unalign<Self>: FromBytes` is not satisfied
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/lib.rs:4587:46
     |
4587 |             Ok((r, suffix)) => Ok((Ref::read(&r).into_inner(), suffix)),
     |                                    --------- ^^ unsatisfied trait bound
     |                                    |
     |                                    required by a bound introduced by this call
     |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<Self>`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
     |
 115 | pub struct Unalign<T>(T);
     | ^^^^^^^^^^^^^^^^^^^^^
     = note: Consider adding `#[derive(FromBytes)]` to `wrappers::Unalign<Self>`
     = help: the following other types implement trait `FromBytes`:
               ()
               AtomicI16
               AtomicI32
               AtomicI64
               AtomicI8
               AtomicIsize
               AtomicU16
               AtomicU32
             and 47 others
note: required by a bound in `r#ref::<impl r#ref::def::Ref<B, T>>::read`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/ref.rs:705:8
     |
 705 |     T: FromBytes,
     |        ^^^^^^^^^ required by this bound in `r#ref::<impl Ref<B, T>>::read`
...
 714 |     pub fn read(r: &Self) -> T {
     |            ---- required by a bound in this associated function

error[E0277]: the trait bound `wrappers::Unalign<Self>: FromBytes` is not satisfied
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/lib.rs:4632:54
     |
4632 |             Ok((prefix, r)) => Ok((prefix, Ref::read(&r).into_inner())),
     |                                            --------- ^^ unsatisfied trait bound
     |                                            |
     |                                            required by a bound introduced by this call
     |
help: the trait `FromBytes` is not implemented for `wrappers::Unalign<Self>`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/wrappers.rs:115:1
     |
 115 | pub struct Unalign<T>(T);
     | ^^^^^^^^^^^^^^^^^^^^^
     = note: Consider adding `#[derive(FromBytes)]` to `wrappers::Unalign<Self>`
     = help: the following other types implement trait `FromBytes`:
               ()
               AtomicI16
               AtomicI32
               AtomicI64
               AtomicI8
               AtomicIsize
               AtomicU16
               AtomicU32
             and 47 others
note: required by a bound in `r#ref::<impl r#ref::def::Ref<B, T>>::read`
    --> /home/alti/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.31/src/ref.rs:705:8
     |
 705 |     T: FromBytes,
     |        ^^^^^^^^^ required by this bound in `r#ref::<impl Ref<B, T>>::read`
...
 714 |     pub fn read(r: &Self) -> T {
     |            ---- required by a bound in this associated function

Some errors have detailed explanations: E0277, E0432, E0463.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `zerocopy` (lib) due to 136 previous errors
```
