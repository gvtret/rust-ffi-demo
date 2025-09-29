//! C ABI wrappers around core::Counter.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use crate::core::Counter;

/// Status / error codes returned by API.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RustFfiDemoStatus {
    RustffiOk = 0,
    RustffiNullArg = 1,
    RustffiInvalidArg = 2,
    RustffiInternalError = 3,
}

/// Opaque handle for C/C++.
#[repr(C)]
pub struct CounterHandle {
    _private: [u8; 0],
}

pub type CounterCallback = Option<unsafe extern "C" fn(value: i64)>;

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = RefCell::new(None);
}

fn set_last_error(msg: &str) {
    let cstr = CString::new(msg).ok();
    LAST_ERROR.with(|slot| *slot.borrow_mut() = cstr);
}

/// Safe wrapper: convert opaque pointer to &mut Counter
fn as_counter_mut(h: *mut CounterHandle) -> Result<&'static mut Counter, RustFfiDemoStatus> {
    if h.is_null() {
        set_last_error("null handle");
        Err(RustFfiDemoStatus::RustffiNullArg)
    } else {
        Ok(unsafe { &mut *(h as *mut Counter) })
    }
}

/// Safe wrapper: convert opaque pointer to &Counter
fn as_counter_ref(h: *const CounterHandle) -> Result<&'static Counter, RustFfiDemoStatus> {
    if h.is_null() {
        set_last_error("null handle");
        Err(RustFfiDemoStatus::RustffiNullArg)
    } else {
        Ok(unsafe { &*(h as *const Counter) })
    }
}

/* =========================
 *          FFI
 * ========================= */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_new(
    initial: i64,
    out_counter: *mut *mut CounterHandle,
) -> RustFfiDemoStatus {
    if out_counter.is_null() {
        set_last_error("out_counter is NULL");
        return RustFfiDemoStatus::RustffiNullArg;
    }
    let boxed = Box::new(Counter::new(initial));
    unsafe { *out_counter = Box::into_raw(boxed) as *mut CounterHandle };
    RustFfiDemoStatus::RustffiOk
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_free(handle: *mut CounterHandle) {
    if handle.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(handle as *mut Counter)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_increment(
    handle: *mut CounterHandle,
    delta: i64,
) -> RustFfiDemoStatus {
    let c = match as_counter_mut(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    c.increment(delta);
    RustFfiDemoStatus::RustffiOk
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_value(
    handle: *const CounterHandle,
    out_value: *mut i64,
) -> RustFfiDemoStatus {
    if out_value.is_null() {
        set_last_error("out_value is NULL");
        return RustFfiDemoStatus::RustffiNullArg;
    }
    let c = match as_counter_ref(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    unsafe { *out_value = c.value() };
    RustFfiDemoStatus::RustffiOk
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_reset(
    handle: *mut CounterHandle,
) -> RustFfiDemoStatus {
    let c = match as_counter_mut(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    c.reset();
    RustFfiDemoStatus::RustffiOk
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_set_label(
    handle: *mut CounterHandle,
    label_utf8: *const c_char,
) -> RustFfiDemoStatus {
    let c = match as_counter_mut(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };

    if label_utf8.is_null() {
        c.set_label(None);
        return RustFfiDemoStatus::RustffiOk;
    }

    let s = unsafe { CStr::from_ptr(label_utf8) };
    match s.to_str() {
        Ok(utf8) => {
            c.set_label(Some(utf8.to_owned()));
            RustFfiDemoStatus::RustffiOk
        }
        Err(_) => {
            set_last_error("label must be valid UTF-8 without NUL");
            RustFfiDemoStatus::RustffiInvalidArg
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_get_label(
    handle: *const CounterHandle,
    out_buf: *mut c_char,
    buf_len: usize,
    out_needed: *mut usize,
) -> RustFfiDemoStatus {
    if out_needed.is_null() {
        set_last_error("out_needed is NULL");
        return RustFfiDemoStatus::RustffiNullArg;
    }

    let c = match as_counter_ref(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let s = c.label().unwrap_or("");
    let needed = s.len() + 1;

    unsafe { *out_needed = needed };

    if out_buf.is_null() {
        return RustFfiDemoStatus::RustffiOk;
    }
    if buf_len < needed {
        set_last_error("buffer too small for label");
        return RustFfiDemoStatus::RustffiInvalidArg;
    }

    unsafe {
        ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, out_buf, s.len());
        *out_buf.add(s.len()) = 0;
    }
    RustFfiDemoStatus::RustffiOk
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_last_error_message() -> *const c_char {
    LAST_ERROR.with(|slot| match slot.borrow().as_ref() {
        Some(cs) => cs.as_ptr(),
        None => ptr::null(),
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_version() -> *const c_char {
    static VER: &str =
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), "\0");
    VER.as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_ffi_demo_counter_set_callback(
    handle: *mut CounterHandle,
    cb: CounterCallback,
) -> RustFfiDemoStatus {
    let c = match as_counter_mut(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };

    if let Some(func) = cb {
        // Wrap C function pointer in a Rust closure
        c.set_callback(Some(Box::new(move |val| {
            unsafe { func(val) };
        })));
    } else {
        c.set_callback(None);
    }

    RustFfiDemoStatus::RustffiOk
}
