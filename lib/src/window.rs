#![allow(bad_style)]

use std::ffi::OsStr;
use std::ffi::c_void;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::iter::once;
use std::io::Error;
use std::ptr::null_mut;

#[repr(C)] 
#[derive(Copy)]
#[cfg_attr(feature = "impl-debug", derive(Debug))]
struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_uchar; 8],
}

impl Clone for GUID {
    #[inline]
    fn clone(&self) -> GUID { *self }
}
        
#[cfg(feature = "impl-default")]
impl Default for GUID {
    #[inline]
    fn default() -> GUID { unsafe { _core::mem::zeroed() } }
}


// This is the huge macro called for implementing IUnknown interface, which I need... I will need a tool to deconstruct this monster
// C:\dev\winapi-rs\src\macros.rs

#[macro_export]
macro_rules! RIDL {
    (#[uuid($l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr)]
    class $class:ident;) => (
        pub enum $class {}
        impl $crate::Class for $class {
            #[inline]
            fn uuidof() -> $crate::shared::guiddef::GUID {
                $crate::shared::guiddef::GUID {
                    Data1: $l,
                    Data2: $w1,
                    Data3: $w2,
                    Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
                }
            }
        }
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl () $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@uuid $interface $($uuid),+}
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,)}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,) $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );
    (@deref $interface:ident $pinterface:ident) => (
        impl $crate::_core::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const $interface as *const $pinterface) }
            }
        }
    );
    (@method fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, $($p,)*)
        }
    );
    (@method #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            let mut ret = $crate::_core::mem::uninitialized();
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, &mut ret, $($p,)*);
            ret
        }
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        $(fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,)*
    ) => (
        RIDL!{@item #[repr(C)]
        pub struct $vtbl {
            $($fields)*
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,)*
        }}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,
        ) $($tail)*}
    );
    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                ret: *mut $rtr,
                $($p: $t,)*
            ) -> *mut $rtr,
        ) $($tail)*}
    );
    (@uuid $interface:ident
        $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => (
        impl $crate::Interface for $interface {
            #[inline]
            fn uuidof() -> $crate::shared::guiddef::GUID {
                $crate::shared::guiddef::GUID {
                    Data1: $l,
                    Data2: $w1,
                    Data3: $w2,
                    Data4: [$b1, $b2, $b3, $b4, $b5, $b6, $b7, $b8],
                }
            }
        }
    );
    (@item $thing:item) => ($thing);
}

// This is the macro call for implementing the IUnknown interface...
// C:\dev\winapi-rs\src\um\unknwnbase.rs
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn AddRef() -> ULONG,
    fn Release() -> ULONG,
}}
pub type LPUNKNOWN = *mut IUnknown;

type c_uchar = u8;
type c_ushort = u16;
type c_uint = u64;
type c_ulong = u64;
type wchar_t = u16;
type c_int = i32;
type c_long = i32;
type LONG_PTR = isize;
type UINT_PTR = usize;
type UINT = c_uint;
type ULONG = c_ulong;
type WORD = c_ushort;
type DWORD = c_ulong;
type ATOM = WORD;
type WCHAR = wchar_t;
type LPCWSTR = *const WCHAR;
type LPVOID = *mut c_void;
type LPMSG = *mut MSG;
type BOOL = c_int;
type LPARAM = LONG_PTR;
type WPARAM = UINT_PTR;
type LRESULT = LONG_PTR;
type LONG = c_long;
type HRESULT = c_long;
pub type IID = GUID;
type REFIID = *const IID;

const CS_OWNDC: UINT = 0x0020;
const CS_HREDRAW: UINT = 0x0002;
const CS_VREDRAW: UINT = 0x0001;
const WS_VISIBLE: DWORD = 0x10000000;
const CW_USEDEFAULT: c_int = 2147483648u32 as c_int;
const WS_OVERLAPPED: DWORD = 0x00000000;
const WS_CAPTION: DWORD = 0x00C00000;
const WS_SYSMENU: DWORD = 0x00080000;
const WS_THICKFRAME: DWORD = 0x00040000;
const WS_MINIMIZEBOX: DWORD = 0x00020000;
const WS_MAXIMIZEBOX: DWORD = 0x00010000;
const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub enum HWND__ {}
type HWND = *mut HWND__;

pub enum HMENU__ {}
type HMENU = *mut HMENU__;

pub enum HINSTANCE__ {}
type HINSTANCE = *mut HINSTANCE__;

pub enum HICON__ {}
type HICON = *mut HICON__;

pub enum HCURSOR__ {}
type HCURSOR = *mut HCURSOR__;

pub enum HBRUSH__ {}
type HBRUSH = *mut HBRUSH__;

type HMODULE = HINSTANCE;

type WNDPROC = Option<unsafe extern "system" fn(_: HWND, _: UINT, _: WPARAM, _: LPARAM) -> LRESULT>;

#[cfg(windows)]
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(windows)]
// This wrapper struct hides the fact that HWND is unsafe
pub struct Window {
    window_handle: HWND,
}

extern "system" {
    pub fn DefWindowProcW(
            hWnd: HWND,
            Msg: UINT,
            wParam: WPARAM,
            lParam: LPARAM,
        ) -> LRESULT;
    fn RegisterClassW(
        lpWndClass: *const WNDCLASSW,
    ) -> ATOM;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
}

// from shcore.dll
type GetDpiForSystem = unsafe extern "system" fn() -> UINT;
type GetDpiForMonitor = unsafe extern "system" fn(HMONITOR, MONITOR_DPI_TYPE, *mut UINT, *mut UINT);

//from user32.dll
type SetProcessDpiAwareness = unsafe extern "system" fn(PROCESS_DPI_AWARENESS) ->HRESULT;
type DCompositionCreateDevice2 = unsafe extern "system" fn(renderingDevice: *const IUnknown, iid: REFIID, dcompositionDevice: *mut *mut c_void,) -> HRESULT;
type CreateDXGIFactory2 = unsafe extern "system" fn(Flags: UINT, riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;

#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[cfg(windows)]
unsafe extern "system"
    fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        
        match msg {
            // TODO
            _ => DefWindowProcW(hwnd, msg, wparam, lparam)
        }
        
    }



/*
Start of the actual user code
*/

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

