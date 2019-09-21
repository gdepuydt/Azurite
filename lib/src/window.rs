use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::{
        iter::once,
        io::Error,
        ptr::null_mut,
};
use winapi::{
    shared::windef::HWND,
	shared::minwindef::{
		UINT,
		LPARAM,
		WPARAM,
		LRESULT,
	},

    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            DefWindowProcW,
            RegisterClassW,
            CreateWindowExW,
            GetMessageW,
            TranslateMessage,
            DispatchMessageW,
            WNDCLASSW,
            CS_OWNDC,
            CS_HREDRAW,
            CS_VREDRAW,
            WS_OVERLAPPEDWINDOW,
            WS_VISIBLE,
            CW_USEDEFAULT,
            MSG,
		}
    }
};

#[cfg(windows)]
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(windows)]
// This wrapper struct hides the fact that HWND is unsafe
pub struct Window {
    window_handle: HWND,
}

#[cfg(windows)]
unsafe extern "system"
    fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        
        match msg {
            // TODO
            _ => DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        
    }

#[cfg(windows)]
pub fn create_window(title: &str) -> Result<Window, Error> {
    
    let window_name = win32_string("azurite_window");
    let window_title = win32_string(title);

    unsafe {
        let hinstance = GetModuleHandleW(null_mut());

        let win_class = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc), //Some(DefWindowProcW), 
            hInstance:  hinstance,
            lpszClassName: window_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW(&win_class);

        let window_handle = CreateWindowExW(
            0,
            window_name.as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut());
        
        if window_handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Window {window_handle})
        }
    }
}

pub fn handle_message(window: &mut Window) -> bool {
    unsafe {
        let mut message: MSG = mem::MaybeUninit::uninit().assume_init();
        
        if GetMessageW(&mut message as *mut MSG, window.window_handle,0,0) > 0 {
            TranslateMessage(&message as *const MSG);
            DispatchMessageW(&message as *const MSG);
            true
        } else {
            false
        }
    }
}

