use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use winapi::{
    shared::windef::HWND,
};

#[cfg(windows)]
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(windows)]
// This wrapper struct hides the fact that HWND is unsafe
struct Window {
    handle: HWND,
}

