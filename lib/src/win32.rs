use std::ffi::c_void;

type HANDLE = *mut c_void;

struct _XINPUT_STATE;
type XINPUT_STATE = _XINPUT_STATE;

// typedef unsigned long(__stdcall *XINPUTGETSTATE)(unsigned long dwUserIndex, XINPUT_STATE* pState);
type XINPUTGETSTATE = extern "system" fn(dwUserIndex: u64, a_state: &XINPUT_STATE) -> u64; 
 
pub struct IAudioClient;
pub struct IAudioRenderClient;


pub struct Win32<'a> {
	window: HANDLE,
	device_context: HANDLE,

	main_fiber: *mut c_void,
	message_fiber: *mut c_void,

	xinput_get_state: XINPUTGETSTATE,

	audio_client: &'a IAudioClient,
	audio_render_client: &'a IAudioRenderClient,

	wgl_context: HANDLE,
}


// registering the window class
    // let class_name = register_window_class(&window_icon, &taskbar_icon);