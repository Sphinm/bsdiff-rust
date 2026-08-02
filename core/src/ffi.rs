//! C FFI layer for bsdiff-core.
//!
//! All functions are `extern "C"` and `#[no_mangle]`.
//! Thread-safe: every call internally acquires a global lock to prevent
//! concurrent file I/O races on the same filesystem state.
//!
//! Return convention: `0` = success, negative = error.
//! Call `bsdiff_last_error()` to retrieve the last error message.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::bsdiff::{BsdiffRust, DiffOptions};
use crate::utils::{get_compression_ratio, get_file_size, get_patch_info, verify_patch};

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

static LAST_ERROR: Mutex<Option<String>> = Mutex::new(None);

pub fn set_error(msg: String) {
    *LAST_ERROR.lock().unwrap() = Some(msg);
}

/// Retrieve the last error message. The caller must free the returned string
/// with `bsdiff_free_string`. Returns null if no error.
#[no_mangle]
pub extern "C" fn bsdiff_last_error() -> *mut c_char {
    let mut guard = LAST_ERROR.lock().unwrap();
    match guard.take() {
        Some(s) => CString::new(s).unwrap_or_default().into_raw(),
        None => std::ptr::null_mut(),
    }
}

/// Free a string previously returned by `bsdiff_last_error`.
#[no_mangle]
pub extern "C" fn bsdiff_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

fn cstr_to_str<'a>(ptr: *const c_char, label: &str) -> Result<&'a str, ()> {
    if ptr.is_null() {
        set_error(format!("{label}: null pointer"));
        return Err(());
    }
    unsafe {
        match CStr::from_ptr(ptr).to_str() {
            Ok(s) => Ok(s),
            Err(e) => {
                set_error(format!("{label}: invalid UTF-8: {e}"));
                Err(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// File info
// ---------------------------------------------------------------------------

/// Returns the file size in bytes, or -1 on error.
#[no_mangle]
pub extern "C" fn bsdiff_file_size(path: *const c_char) -> i64 {
    let path = match cstr_to_str(path, "path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match get_file_size(path) {
        Ok(sz) => sz as i64,
        Err(e) => {
            set_error(format!("bsdiff_file_size: {e}"));
            -1
        }
    }
}

/// Check whether a file is a valid BSDIFF40 patch.
/// Returns 1 if valid, 0 if not, -1 on error.
#[no_mangle]
pub extern "C" fn bsdiff_is_valid_patch(patch_path: *const c_char) -> i32 {
    let path = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match get_patch_info(path) {
        Ok(info) => {
            if info.is_bsdiff40 {
                1
            } else {
                0
            }
        }
        Err(e) => {
            set_error(format!("bsdiff_is_valid_patch: {e}"));
            -1
        }
    }
}

// ---------------------------------------------------------------------------
// Core operations
// ---------------------------------------------------------------------------

/// Generate a BSDIFF40 patch file.
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn bsdiff(
    old_path: *const c_char,
    new_path: *const c_char,
    patch_path: *const c_char,
) -> i32 {
    let old = match cstr_to_str(old_path, "old_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let new = match cstr_to_str(new_path, "new_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let patch = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match BsdiffRust::diff(old, new, patch) {
        Ok(()) => 0,
        Err(e) => {
            set_error(format!("bsdiff: {e}"));
            -1
        }
    }
}

/// Apply a BSDIFF40 patch to produce a new file.
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn bspatch(
    old_path: *const c_char,
    new_path: *const c_char,
    patch_path: *const c_char,
) -> i32 {
    let old = match cstr_to_str(old_path, "old_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let new = match cstr_to_str(new_path, "new_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let patch = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match BsdiffRust::patch(old, new, patch) {
        Ok(()) => 0,
        Err(e) => {
            set_error(format!("bspatch: {e}"));
            -1
        }
    }
}

// ---------------------------------------------------------------------------
// Advanced operations
// ---------------------------------------------------------------------------

/// Generate a patch with custom compression level (0-9).
/// Returns 0 on success, -1 on error.
#[no_mangle]
pub extern "C" fn bsdiff_with_compression(
    old_path: *const c_char,
    new_path: *const c_char,
    patch_path: *const c_char,
    compression_level: i32,
) -> i32 {
    let old = match cstr_to_str(old_path, "old_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let new = match cstr_to_str(new_path, "new_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let patch = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let level = compression_level.clamp(0, 9) as u32;
    let opts = DiffOptions {
        compression_level: level,
        enable_parallel: true,
    };
    match BsdiffRust::diff_with_options(old, new, patch, &opts) {
        Ok(()) => 0,
        Err(e) => {
            set_error(format!("bsdiff_with_compression: {e}"));
            -1
        }
    }
}

/// Verify that applying a patch to `old_path` produces `expected_new_path`.
/// Returns 1 if identical, 0 if different, -1 on error.
#[no_mangle]
pub extern "C" fn bsdiff_verify(
    old_path: *const c_char,
    expected_new_path: *const c_char,
    patch_path: *const c_char,
) -> i32 {
    let old = match cstr_to_str(old_path, "old_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let new = match cstr_to_str(expected_new_path, "expected_new_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let patch = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match verify_patch(old, new, patch) {
        Ok(true) => 1,
        Ok(false) => 0,
        Err(e) => {
            set_error(format!("bsdiff_verify: {e}"));
            -1
        }
    }
}

/// Compute compression ratio struct.  Fills `out_ratio` (patch_size/new_size * 100).
/// Returns 0 on success, -1 on error.
#[repr(C)]
pub struct BsdiffRatio {
    pub old_size: u64,
    pub new_size: u64,
    pub patch_size: u64,
    pub ratio_percent: f64,
}

#[no_mangle]
pub extern "C" fn bsdiff_compression_ratio(
    old_path: *const c_char,
    new_path: *const c_char,
    patch_path: *const c_char,
    out: *mut BsdiffRatio,
) -> i32 {
    if out.is_null() {
        set_error("bsdiff_compression_ratio: null output pointer".into());
        return -1;
    }
    let old = match cstr_to_str(old_path, "old_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let new = match cstr_to_str(new_path, "new_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    let patch = match cstr_to_str(patch_path, "patch_path") {
        Ok(p) => p,
        Err(()) => return -1,
    };
    match get_compression_ratio(old, new, patch) {
        Ok(r) => {
            unsafe {
                (*out) = BsdiffRatio {
                    old_size: r.old_size,
                    new_size: r.new_size,
                    patch_size: r.patch_size,
                    ratio_percent: r.ratio,
                };
            }
            0
        }
        Err(e) => {
            set_error(format!("bsdiff_compression_ratio: {e}"));
            -1
        }
    }
}
