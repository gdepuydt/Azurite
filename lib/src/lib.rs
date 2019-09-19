
use std::{
	ffi::OsStr,
	os::windows::ffi::OsStrExt,
	ptr,
	mem,
};

use winapi::{
	um::{ 
		winuser,
		winnt::{LPCWSTR},
		libloaderapi,
	},
		shared::{
    	minwindef::{UINT},
		windef::{HWND}
    },
};


mod win32;

// Global constants
const FALSE: usize = 0;
const TRUE: usize = 1;
const MAX_KEYS: usize = 256;
// const MAX_TEXT: usize = 256;
const MAX_ERROR: usize = 1024;
const CTRL: usize = 0x11;
const ALT: usize = 0x12;
const SHIFT:usize = 0x10;
const MAX_AUDIO_BUFFER: usize = 2 * 1024;
/*
// type A_Bool = u8;
type SoundSample = i16; 

pub struct Int2 {
	x: usize,
	y: usize,
}

pub struct DigitalButton {
	down: bool,
	pressed: bool, // !down -> down
	released: bool, //down -> !down
}

impl DigitalButton {
	fn digital_button_update(&mut self, down: bool) {
		let was_down = self.down;
		self.down = down;
		self.released = was_down && !down;
		self.pressed = !was_down &&  down;
	}
}

pub struct AnalogButton {
	threshold: f32, //defaults to 0.5
	value: f32, //0.0 to 1.0
	down: bool, //value <= threshold
	pressed: bool,// !down -> down
	released: bool, //down -> !down
}

impl AnalogButton {
	fn analog_button_update(&mut self, value: f32) {
		self.value = value;
		let was_down = self.down;
		self.down = value >= self.threshold;
		self.pressed = !was_down && self.down;
		self.released = was_down && !self.down;  
	}
}

pub struct Stick {
	threshold: f32,
	x: f32,
	y: f32,
}

impl Stick {
	fn stick_update(&mut self, x: f32, y: f32) {
		if x.abs() <= self.threshold {
			x = 0.0_f32;
		}
		self.x = x;

		if y.abs() <= self.threshold {
			y = 0.0_f32;
		} 
		self.y = y;
	}
}

pub struct Gamepad {
	connected: bool,
	a_button: DigitalButton,
	b_button: DigitalButton,
	x_button: DigitalButton,
	y_button: DigitalButton,
	left_shoulder_button: DigitalButton,
	right_shoulder_button: DigitalButton,
	up_button: DigitalButton,
	down_button: DigitalButton,
	left_button: DigitalButton,
	right_button: DigitalButton,
	left_trigger: AnalogButton,
	right_trigger: AnalogButton,
	left_thumb_stick: Stick,
	right_thumb_stick: Stick,
	left_thumb_button: DigitalButton,
	right_thumb_button: DigitalButton,
	back_button: DigitalButton,
	start_button: DigitalButton,
}

pub struct Mouse {
	left_button: DigitalButton,
	right_button: DigitalButton,
	delta_position: Int2,
	position: Int2, //client window relative
	wheel: usize,
	delta_wheel: usize,
}

pub struct Window {
	title: String,
	pos: Int2,
	size: Int2,
	resized: bool,
}


// user can first define a simple window struct. this infomation can then be used to create the win32 window (TODO, see per vognsen code)
impl Window {
	fn new(title: String, pos: Int2, size: Int2, resized: bool ) -> Window {
		Window{
			title,
			pos,
			size,
			resized,
		}	
    }
}

struct Image<'a> {
	pixels: &'a u8,
    channels: u32,
    width: u32,
    height: u32,
}

impl<'a> Image<'a> {
	fn load_image(file_name: &str) -> Image {

	}
}


	

struct AudioFormat {
	samples_per_second: u32,
	channels: u32,
	bytes_per_sample: u32,
}

struct AudioBuffer<'a> {
	samples: &'a i16,
	samples_count: usize,
	format: AudioFormat,
}
impl<'a> AudioBuffer<'a> {
	fn load_audio(file_name: &str) -> AudioBuffer {

	}
}

// typedef void(*P_AudioCallback)(P_AudioBuffer *buffer);
type AudioCallback = fn(buffer: &AudioBuffer);

pub struct Audio {
	format: AudioFormat,
	callback: AudioCallback,
}

struct Time {
	delta_seconds: f32,
	delta_ticks: u64,
	delta_nanoseconds: u64,
	delta_microseconds: u64,
	delta_milliseconds: u64,
	delta_sound_samples: u64, 

	seconds: f64, 
	ticks: u64,
	nanoseconds: u64,
	microseconds: u64,
	milliseconds: u64,

	initial_ticks: u64,
	ticks_per_second: u64,
}

pub struct Azurite<'a> {
    initialized: bool,
    quit: bool,

	// TODO: error handling
    /*
	char *error;
    char error_buffer[MU_MAX_ERROR];
	*/
    
	window: Window<'a>,
    //keys: [DigitalButton; MAX_KEYS],
    gamepad: Gamepad,
    mouse: Mouse,
	//text: String,
	time: Time,
    audio: Audio,
    //#[cfg(target_os = "windows")]
	win32: &'a win32::Win32<'a>,
    // /* @platform{macos} */ struct Mu_Cocoa *cocoa;
}





impl<'a> Azurite<'a> {
	fn new() ->  Azurite<'static> {
		let win32_window = Self::window_initialize();
		
		let time = Time::new();
		let mouse = Mouse::new();
		let gamepad = Gamepad::new();
		let audio = Audio::new();
		let opengl = OpenGL::new();

		Azurite{
			win32: win32_window,
			time,
			mouse,
			gamepad,
			audio,
			initialized: true,
			quit: false
		}
	}
*/
	pub unsafe fn window_initialize() -> HWND {
		let class_name: Vec<_> = OsStr::new("Window Class")
        	.encode_wide()
        	.chain(Some(0).into_iter())
        	.collect();
		let title: Vec<_> = OsStr::new("Azurite")
			.encode_wide()
			.chain(Some(0).into_iter())
			.collect();

		let class = winuser::WNDCLASSEXW {
        	cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as UINT,
        	style: winuser::CS_HREDRAW | winuser::CS_VREDRAW | winuser::CS_OWNDC,
        	lpfnWndProc: Some(winuser::DefWindowProcW),
        	cbClsExtra: 0,
        	cbWndExtra: 0,
        	hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
        	hIcon: ptr::null_mut(),
        	hCursor: ptr::null_mut(), // must be null in order for cursor state to work properly
        	hbrBackground: ptr::null_mut(),
        	lpszMenuName: ptr::null(),
        	lpszClassName: class_name.as_ptr(),
        	hIconSm: ptr::null_mut(),
    	};

		winuser::RegisterClassExW(&class);
		
		let window_x = 800;//winuser::CW_USEDEFAULT;
		let window_y = 800;//winuser::CW_USEDEFAULT; 
		let window_width = 800;//winuser::CW_USEDEFAULT;
		let window_height = 800;//winuser::CW_USEDEFAULT;
		let ex_style = winuser::WS_EX_WINDOWEDGE 
						 | winuser::WS_EX_TOPMOST 
						 | winuser::WS_EX_NOREDIRECTIONBITMAP 
						 | winuser::WS_EX_LAYERED 
						 | winuser::WS_SIZEBOX 
						 | winuser::WS_MAXIMIZEBOX 
						 | winuser::WS_CHILD 
						 | winuser::WS_EX_APPWINDOW;

		let win32_window_handle = winuser::CreateWindowExW(
    	    ex_style,
    	    class_name.as_ptr(), //The window name that was registered
    	    title.as_ptr() as LPCWSTR,
    	    winuser::WS_OVERLAPPEDWINDOW,
    	    window_x,
			window_y,
    	    window_width,
    	    window_height,
			ptr::null_mut(), // pl_attribs.parent.unwrap_or(ptr::null_mut()),
			ptr::null_mut(), // ptr::null_mut(),
			ptr::null_mut(), // libloaderapi::GetModuleHandleW(ptr::null()),
			ptr::null_mut(), // ptr::null_mut(),   	    
    	);
		// TODO ?: winuser::SetWindowLongPtrW(win32_window_handle, winuser::GWLP_USERDATA, (LONG_PTR)p);
		winuser::ShowWindow(win32_window_handle, winuser::SW_SHOW);
		// TODO: p->win32.device_context = GetDC(p->win32.window);
		win32_window_handle
		
	}
/*
	fn pull(&self) -> bool {
		true
	}

	fn push(&mut self) {

	}

}

*/


// TODO continue image and audio