// SPDX-License-Identifier: Apache-2.0

/// _Unwind_Resume is only needed in the `debug` profile
///
/// even though this project has `panic=abort`
/// it seems like the debug libc.rlib has some references
/// with unwinding
/// See also: https://github.com/rust-lang/rust/issues/47493
#[cfg(not(test))]
#[cfg(debug_assertions)]
#[no_mangle]
extern "C" fn _Unwind_Resume() {
    unimplemented!();
}

/// rust_eh_personality is only needed in the `debug` profile
///
/// even though this project has `panic=abort`
/// it seems like the debug libc.rlib has some references
/// with unwinding
/// See also: https://github.com/rust-lang/rust/issues/47493
#[cfg(not(test))]
#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {
    unimplemented!();
}

#[cfg(not(test))]
#[panic_handler]
#[allow(clippy::empty_loop)]
#[allow(missing_docs)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use crate::asm::hlt_loop;
    hlt_loop()
}

#[cfg(not(target_feature = "crt-static"))]
#[allow(missing_docs)]
fn __check_for_static_linking() {
    compile_error!("shim is not statically linked");
}
