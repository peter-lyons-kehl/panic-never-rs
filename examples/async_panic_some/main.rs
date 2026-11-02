#![no_std]
#![no_main]

#[cfg(all(debug_assertions, target_os = "linux"))]
compile_error!("Build in release profile only. Required by `panic_never`.");

#[cfg(all(debug_assertions, not(target_os = "linux")))]
compile_error!(
    "(1) Build in release profile only. Required by `panic_never`. (2) You are not on Linux, so \
     you may need to tweak ../../.cargo/config.toml. Please do report any fixes."
);

use core::future::Future;
use core::hint;
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

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let future = async {
        outer().await;
    };
    if core::hint::black_box(false) {
        new_dummy_runtime_and_poll(future);
    }
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

async fn inner() {
    if hint::black_box(true) {
        panic!()
    }
}
