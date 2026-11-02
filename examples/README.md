# Examples

- `block_panic_*` is for ordinary code (not `async`)
- `async_panic_*` is for `async`
- `*_panic_none` guarantees no `panic!` on any execution branch
- `*_panic_some` has `panic!` at least on one execution branch
- [`block_panic_none`](block_panic_none/main.rs)
- [`block_panic_some`](block_panic_some/main.rs)
- [`async_panic_none`](async_panic_none/main.rs)
- [`async_panic_some`](async_panic_some/main.rs)

## ../.cargo/config.toml

<../.cargo/config.toml> applies to all examples.

## Runnable on Linux only. Buildable possibly on other platforms.

If your platform is other than Linux, then
- find out what to put in <../.cargo/config.toml>, and
- share it (by opening a GitHub issue, or even better, by a Pull Request),
  please.

## Dummy panic handler for debug profile

Each example defines `#[panic_handler]` in debug mode. That is _not_ being used,
because in debug mode each example triggers `compile_error!`. However, we do
include this in debug mode, otherwise (in debug mode) we would also get a
confusing error:

`#[panic_handler]` function required, but not found

## exit - on Linux only. On other platforms run indefinitely.

Each non-panicking (`***_none`) example invokes a syscall to exit gracefully -
but on Linux only. On other platforms it runs _indefinitely_.

- As per https://github.com/rust-lang/libc/discussions/5683, we don't use
  `libc::exit(..)`.
- Neither we use `exit-no-std` (which itself on systems other than DOS/Windows
  uses `libc::exit(..)`).
- Neither we use `rustix` (which supposedly does use a syscall rather than
  `libc` on Linux, but it uses `libc` on FreeBSD/NetBSD, even though those two
  BSD unixes (unlike OpenBSD) _do_ allow direct invocation of syscalls).
- It _should_ be possible to use `libc::exit(..)` (directly, or through
  `exit-no-std` or `rustix`) in `no_std` and `no_main` and with `panic-never`,
  but with much more boiler plate. See
  https://github.com/rust-lang/libc/discussions/5683.

## async: new_dummy_runtime_and_poll

<async_panic_none/main.rs> and <async_panic_some/main.rs> have function
`new_dummy_runtime_and_poll` (same in both). That is _never_ run (see `_start`
function in those examples), but it _is_ built and linked (ensured by
[core::hint::black_box](https://doc.rust-lang.org/core/hint/fn.black_box.html)
in `_start`).

Because it's never run, we can use a dummy/uninitialized
[core::task::Context](https://doc.rust-lang.org/core/task/struct.Context.html).
But, we _cannot_ use `loop {}` to yield it. Otherwise the compiler optimizes
away the rest of the code (as unreachable) - and we don't want that, because
then we don't get the linking failure that we _do_ want to get (if there is a
panic used and that panic is reachable when run with a normal async runtime)!

`let mut context = loop {};`

That's why `new_dummy_runtime_and_poll` has `unsafe` code and it calls
[core::mem::MaybeUninit](https://doc.rust-lang.org/core/mem/union.MaybeUninit.html).
