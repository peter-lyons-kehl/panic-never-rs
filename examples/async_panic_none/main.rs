#![no_std]
#![no_main]

#[cfg(all(debug_assertions, target_os = "linux"))]
compile_error!("Build in release profile only. Required by `panic_never`.");

#[cfg(all(debug_assertions, not(target_os = "linux")))]
compile_error!(
    "(1) Build in release profile only. Required by `panic_never`. (2) You are not on Linux, so \
     you may need to tweak ../../.cargo/config.toml. Please do report any fixes. (3) You are not \
     on Linux, so when you run the build (release) binary, it will run indefinitely. Use Ctrl+C or \
     similar to stop it."
);

#[cfg(target_os = "linux")]
use core::arch::asm;

use core::future::Future;
use core::mem::MaybeUninit;
use core::pin::pin;

#[cfg(not(debug_assertions))]
use panic_never as _;

#[cfg(debug_assertions)]
/// See <../README.md>
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "linux")]
/// See <../README.md>
fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let future = async {
        outer().await;
    };
    if core::hint::black_box(false) {
        new_dummy_runtime_and_poll(future);
    }

    #[cfg(target_os = "linux")]
    exit(0);

    #[cfg_attr(target_os = "linux", allow(unreachable_code))]
    loop {}
}

/// See <../README.md>
fn new_dummy_runtime_and_poll<F>(fut: F)
where
    F: Future<Output = ()>,
{
    let fut_pinned = pin!(fut);
    let mut context = unsafe {
        #[allow(invalid_value)]
        MaybeUninit::uninit().assume_init()
    };

    let _ = fut_pinned.poll(&mut context);
}
//-------

async fn outer() {
    inner().await
}

async fn awaited_panic_but_never_reached() {
    panic!()
}

async fn never_awaited_panic() {
    // the following _is_ .await-ed on; but the current function is _never_ .await-ed on.
    awaited_panic_but_never_reached().await;
    panic!()
}

async fn inner() {
    // intentionally no .await
    let _ = core::hint::black_box(never_awaited_panic());
}
