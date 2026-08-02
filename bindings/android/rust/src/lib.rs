//! Android JNI bindings for bsdiff-core.
//!
//! Exposes `com.atome.bsdiff.BsdiffCore` native methods.
//! The shared library is named `libbsdiff_core.so`.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};

use bsdiff_core::bsdiff::BsdiffRust;
use bsdiff_core::utils::get_patch_info;

// ---------------------------------------------------------------------------
// Error → JNI string
// ---------------------------------------------------------------------------

fn last_error_to_jstring(env: &mut JNIEnv) -> jstring {
    unsafe {
        let ptr = bsdiff_core::ffi::bsdiff_last_error();
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            let c_str = std::ffi::CStr::from_ptr(ptr);
            let rust_str = c_str.to_string_lossy().to_string();
            bsdiff_core::ffi::bsdiff_free_string(ptr);
            env.new_string(&rust_str)
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

fn get_string(env: &mut JNIEnv, obj: &JString) -> Result<String, jni::errors::Error> {
    env.get_string(obj).map(|s| s.into())
}

// ---------------------------------------------------------------------------
// JNI exports
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_com_atome_bsdiff_BsdiffCore_nativePatch(
    mut env: JNIEnv,
    _class: JClass,
    old_path: JString,
    new_path: JString,
    patch_path: JString,
) -> jint {
    let old = match get_string(&mut env, &old_path) {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let new = match get_string(&mut env, &new_path) {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let patch = match get_string(&mut env, &patch_path) {
        Ok(s) => s,
        Err(_) => return -1,
    };

    match BsdiffRust::patch(&old, &new, &patch) {
        Ok(()) => 0,
        Err(e) => {
            bsdiff_core::ffi::set_error(format!("bspatch: {e}"));
            -1
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_atome_bsdiff_BsdiffCore_nativeIsValidPatch(
    mut env: JNIEnv,
    _class: JClass,
    patch_path: JString,
) -> jint {
    let path = match get_string(&mut env, &patch_path) {
        Ok(s) => s,
        Err(_) => return -1,
    };
    match get_patch_info(&path) {
        Ok(info) => {
            if info.is_bsdiff40 { 1 } else { 0 }
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_atome_bsdiff_BsdiffCore_nativeLastError(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    last_error_to_jstring(&mut env)
}
