#![allow(bad_style)]

use std::ffi::OsStr;
use std::ffi::c_void;
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::iter::once;
use std::io::Error;
use std::ptr::null_mut;
use std::cell::Cell;
use std::sync::Once;
use core::ops::Deref;
use std::hint::unreachable_unchecked;

#[repr(C)] 
#[derive(Copy)]
#[cfg_attr(feature = "impl-debug", derive(Debug))]
pub struct GUID {
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

#[repr(C)] 
#[derive(Copy)]
#[cfg_attr(feature = "impl-debug", derive(Debug))]
pub struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}

impl Clone for SECURITY_ATTRIBUTES {
    #[inline]
    fn clone(&self) -> SECURITY_ATTRIBUTES { *self }
}
        
#[cfg(feature = "impl-default")]
impl Default for SECURITY_ATTRIBUTES {
    #[inline]
    fn default() -> SECURITY_ATTRIBUTES { unsafe { _core::mem::zeroed() } }
}


#[repr(C)]
pub struct IUnknownVtbl {
    pub QueryInterface: unsafe extern "system" fn(
        This: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(
        This: *mut IUnknown,
    ) -> HRESULT,
    pub Release: unsafe extern "system" fn(
        This: *mut IUnknown,
    ) -> HRESULT,
}

pub trait Interface {
    // Returns the IID of the Interface
    fn uuidof() -> GUID;
}

#[repr(C)]
pub struct IUnknown {
    pub lpVtbl: *const IUnknownVtbl,
}

// https://docs.microsoft.com/en-us/windows/win32/com/queryinterface--navigating-in-an-object
impl IUnknown {
            #[inline] pub unsafe fn QueryInterface(&self, riid: REFIID, ppvObject: *mut *mut c_void) -> HRESULT {
                ((*self.lpVtbl).QueryInterface)(self as *const _ as *mut _, riid, ppvObject)
            }
            #[inline] pub unsafe fn AddRef(&self) -> HRESULT {
                ((*self.lpVtbl).AddRef)(self as *const _ as *mut _)
            }
            #[inline] pub unsafe fn Release(&self) -> HRESULT {
                ((*self.lpVtbl).Release)(self as *const _ as *mut _)
            }
}

impl Interface for IUnknown {
    #[inline]
    fn uuidof() -> GUID {
        GUID {
            Data1: 0x00000000,
            Data2: 0x0000,
            Data3: 0x0000,
            Data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        }
    }
}

pub type LPUNKNOWN = *mut IUnknown;

type c_uchar = u8;
type c_ushort = u16;
type c_uint = u64;
type c_ulong = u32;
type wchar_t = u16;
type c_int = i32;
type c_long = i32;
type LONG_PTR = isize;
type UINT_PTR = usize;
type UINT = c_uint;
// type ULONG = c_ulong;
type WORD = c_ushort;
type DWORD = c_ulong;
type ATOM = WORD;
type WCHAR = wchar_t;
type LPCWSTR = *const WCHAR;
type LPCSTR = *const CHAR;
type CHAR = c_char;
type c_char = i8;
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
type FARPROC = *mut __some_function;
type HANDLE = *mut c_void;
type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
type COLORREF = DWORD;

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
const STD_OUTPUT_HANDLE: DWORD = 0xFFFFFFF5;
const STD_ERROR_HANDLE: DWORD = -12i32 as u32;
const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const FILE_TYPE_UNKNOWN: DWORD = 0x0000;
const ATTACH_PARENT_PROCESS: DWORD = 0xFFFFFFFF;
const GENERIC_READ: DWORD = 0x80000000;
const GENERIC_WRITE: DWORD = 0x40000000;
const FILE_SHARE_WRITE: DWORD = 0x00000002;
const OPEN_EXISTING: DWORD = 3;
const IDI_APPLICATION: LPCWSTR = 32512 as LPCWSTR;



pub type PROCESS_DPI_AWARENESS = u32;
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = 0;
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1;
pub const PROCESS_PER_MONITOR_DPI_AWARE:PROCESS_DPI_AWARENESS = 2;


pub type MONITOR_DPI_TYPE = u32;
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = 0;
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = 1;
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = 2;
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MDT_EFFECTIVE_DPI;

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

pub enum HMONITOR__ {}
type HMONITOR = *mut HMONITOR__;

pub enum __some_function {}

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
}

impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(once(0)).collect()
    }
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
    pub fn CreateFileA(
        lpFileName: LPCSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> HANDLE;
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    pub fn LoadLibraryW(lpFileName: LPCWSTR) -> HMODULE;
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    pub fn GetFileType(hFile: HANDLE) -> DWORD;
    pub fn AttachConsole(dwProcessId: DWORD) -> BOOL;
    pub fn SetStdHandle(nStdHandle: DWORD, hHandle: HANDLE) -> BOOL;
    pub fn LoadIconW(hInstance: HINSTANCE, lpIconName: LPCWSTR) -> HICON;
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
}

// from shcore.dll
type GetDpiForSystem = unsafe extern "system" fn() -> UINT;
type GetDpiForMonitor = unsafe extern "system" fn(HMONITOR, MONITOR_DPI_TYPE, *mut UINT, *mut UINT);

//from user32.dll
type SetProcessDpiAwareness = unsafe extern "system" fn(PROCESS_DPI_AWARENESS) ->HRESULT;
type DCompositionCreateDevice2 = unsafe extern "system" fn(renderingDevice: *const IUnknown, iid: REFIID, dcompositionDevice: *mut *mut c_void,) -> HRESULT;
type CreateDXGIFactory2 = unsafe extern "system" fn(Flags: UINT, riid: REFIID, ppFactory: *mut *mut c_void) -> HRESULT;

#[allow(non_snake_case)]
pub struct OptionalFunctions {
    pub GetDpiForSystem: Option<GetDpiForSystem>,
    pub GetDpiForMonitor: Option<GetDpiForMonitor>,
    pub SetProcessDpiAwareness: Option<SetProcessDpiAwareness>,
    pub DCompositionCreateDevice2: Option<DCompositionCreateDevice2>,
    pub CreateDXGIFactory2: Option<CreateDXGIFactory2>,
}



fn load_optional_functions() -> Option<OptionalFunctions> {
    
    macro_rules! load_function {
        ($lib: expr, $function: ident, $min_windows_version: expr) => {{
            let function_name = stringify!($function);
            let lib_name = stringify!($lib);
            let cstr = CString::new(function_name).unwrap();
            let function_ptr = unsafe { GetProcAddress($lib, cstr.as_ptr())}; // https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html
            if function_ptr.is_null() {                
                println!("Could not load `{}`. Windows {} or later is needed", 
                function_name, $min_windows_version);
            }
            else {
                // https://doc.rust-lang.org/std/mem/fn.transmute.html
                let function = unsafe { mem::transmute::<_, $function>(function_ptr)};
                $function = Some(function);
                println!("Loaded function {} from library {}", function_name, lib_name);
            }
        }}
    }

    fn load_library(name: &str) -> HMODULE {
        let encoded_name = name.to_wide();

        let library = unsafe {GetModuleHandleW(encoded_name.as_ptr())};
        if !library.is_null() {
            return library;
        }

        let library = unsafe {LoadLibraryW(encoded_name.as_ptr())};
        return library;
    }

    let shcore = load_library("shcore.dll");
    let user32 = load_library("user32.dll");
    let dcomp = load_library("dcomp.dll");
    let dxgi = load_library("dxgi.dll");

    let mut GetDpiForSystem = None;
    let mut GetDpiForMonitor = None;
    let mut SetProcessDpiAwareness = None;
    let mut DCompositionCreateDevice2 = None;
    let mut CreateDXGIFactory2 = None;

    if shcore.is_null() {
        println!("No shcore.dll");
    } else {
        load_function!(shcore, SetProcessDpiAwareness, "8.1");
        load_function!(shcore, GetDpiForMonitor, "8.1");
    }

    if user32.is_null() {
        println!("No user32.dll");
    } else {
        load_function!(user32, GetDpiForSystem, "10");
    }

    if !dcomp.is_null() {
        load_function!(dcomp, DCompositionCreateDevice2, "8.1");
    }

    if !dxgi.is_null() {
        load_function!(dxgi, CreateDXGIFactory2, "8.1");
    }

    Some(OptionalFunctions {
        GetDpiForSystem,
        GetDpiForMonitor,
        SetProcessDpiAwareness,
        DCompositionCreateDevice2,
        CreateDXGIFactory2,
    })
}




//
//lazy static loading: technique borrowed from the lazy_static crate
//
pub struct OPTIONAL_FUNCTIONS {
    optional_functions: Cell<Option<OptionalFunctions>>,
}

static INIT: Once = Once::new();
pub static OPTIONAL_FUNCTIONS: OPTIONAL_FUNCTIONS = OPTIONAL_FUNCTIONS{optional_functions: Cell::new(None)} ;
unsafe impl Sync for OPTIONAL_FUNCTIONS<> {}

impl Deref for OPTIONAL_FUNCTIONS {
    type Target = OptionalFunctions;
    
    fn deref(&self) -> &OptionalFunctions {
        
        INIT.call_once(|| { OPTIONAL_FUNCTIONS.optional_functions.set(load_optional_functions())});
        unsafe {
            match *OPTIONAL_FUNCTIONS.optional_functions.as_ptr() {
                // "Created reference to optional system library functions."
                Some(ref x) =>  x,
                None => {
                    debug_assert!(false, "attempted to derefence an uninitialized value. This is a bug");
                    unreachable_unchecked()
                }
            }
        }
    }
}

//
// 
//

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
    
    let window_title = title.to_wide();

    unsafe {
        

        let window_handle = CreateWindowExW(
            0,
            window_name.to_wide().as_ptr(),
            window_title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            null_mut(),
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


pub const window_name: &str = "azurite_window";

pub fn init() {
    

    // Attach console
    attach_console();

    // Set DPI aware mode
    if let Some(func) = OPTIONAL_FUNCTIONS.SetProcessDpiAwareness {
        unsafe {
            func(PROCESS_SYSTEM_DPI_AWARE);
            println!("DPI aware mode set.");
        }
    }
    
    // Register window class
    
    
    
    unsafe{
        let hinstance = GetModuleHandleW(null_mut());        
        let icon = LoadIconW(0 as HINSTANCE, IDI_APPLICATION);
        let brush = CreateSolidBrush(0xffffff);
        
        let win_class = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc), //Some(DefWindowProcW), 
            hInstance:  hinstance,
            lpszClassName: window_name.to_wide().as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: icon,
            hCursor: null_mut(),
            hbrBackground: brush,
            lpszMenuName: null_mut(),
        };
         if RegisterClassW(&win_class) > 0 {
            println!("Registered window class.");
        }  else {
            panic!("Failed to register the window class.");
        }
        
    }
}

fn attach_console() {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
         if stdout != INVALID_HANDLE_VALUE && GetFileType(stdout) != FILE_TYPE_UNKNOWN {
            println!("Existing console already attached to main process."); 
            return;
         }
         if AttachConsole(ATTACH_PARENT_PROCESS) > 0 {
            let chnd = CreateFileA(CString::new("CONOUT$").unwrap().as_ptr(), GENERIC_READ| GENERIC_WRITE, FILE_SHARE_WRITE, null_mut(), OPEN_EXISTING, 0, null_mut(),);
            if chnd == INVALID_HANDLE_VALUE {
                return;
            }

            SetStdHandle(STD_OUTPUT_HANDLE, chnd);
            SetStdHandle(STD_ERROR_HANDLE, chnd);
            println!("Console attached to main process.");
         }
    }
} 

