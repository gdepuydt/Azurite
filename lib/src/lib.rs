use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{
        iter::once,
        io::Error,
        ptr::null_mut,
};
use winapi::{
    shared::windef::HWND,
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::WNDCLASSW,
    }
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

#[cfg(windows)]
pub fn create_window(title: &str) -> Result<Window, Error> {
    
    
    let window_name = win32_string("azurite_window");
    let window_title = win32_string(title);

    unsafe {
        unimplemented!();
        let hinstance = GetModuleHandleW(null_mut());

        let win_class = WNDCLASSW {

        };

    }
}

