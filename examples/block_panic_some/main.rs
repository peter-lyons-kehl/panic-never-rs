#![no_std]
#![no_main]

#[cfg(all(debug_assertions, target_os = "linux"))]
compile_error!("Build in release profile only. Required by `panic_never`.");

#[cfg(all(debug_assertions, not(target_os = "linux")))]
compile_error!(
    "(1) Build in release profile only. Required by `panic_never`. (2) You are not on Linux, so \
     you may need to tweak .cargo/config.toml. Please do report any fixes."
);

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
    panic!();
}
