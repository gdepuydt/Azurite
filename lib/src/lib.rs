use std::ffi::c_void;

// Global constants
const A_FALSE: usize = 0;
const A_TRUE: usize = 1;
const A_MAX_KEYS: usize = 256;
const A_MAX_TEXT: usize = 256;
const A_MAX_ERROR: usize = 1024;
const A_CTRL: usize = 0x11;
const A_ALT: usize = 0x12;
const A_SHIFT:usize = 0x10;
const A_MAX_AUDIO_BUFFER: usize = 2 * 1024;

type A_Bool = u8;
type A_SoundSample = i16; 

pub struct A_Int2 {
	x: usize,
	y: usize,
}

pub struct A_DigitalButton {
	down: A_Bool,
	pressed: A_Bool, // !down -> down
	released: A_Bool, //down -> !down
}

pub struct A_AnalogButton {
	threshold: f32, //defaults to 0.5
	value: f32, //0.0 to 1.0
	down: A_Bool, //value <= threshold
	pressed: A_Bool,// !down -> down
	released: A_Bool, //down -> !down
}

pub struct A_Stick {
	threshold: f32,
	x: f32,
	y: f32,
}

pub struct A_Gamepad {
	connected: A_Bool,
	a_button: A_DigitalButton,
	b_button: A_DigitalButton,
	x_button: A_DigitalButton,
	y_button: A_DigitalButton,
	left_shoulder_button: A_DigitalButton,
	right_shoulder_button: A_DigitalButton,
	up_button: A_DigitalButton,
	down_button: A_DigitalButton,
	left_button: A_DigitalButton,
	right_button: A_DigitalButton,
	left_trigger: A_AnalogButton,
	right_trigger: A_AnalogButton,
	left_thumb_stick: A_Stick,
	right_thumb_stick: A_Stick,
	left_thumb_button: A_DigitalButton,
	right_thumb_button: A_DigitalButton,
	back_button: A_DigitalButton,
	start_button: A_DigitalButton,
}

pub struct A_Mouse {
	left_button: A_DigitalButton,
	right_button: A_DigitalButton,
	delta_position: A_Int2,
	position: A_Int2, //client window relative
	wheel: usize,
	delta_wheel: usize,
}

pub struct A_Window<'a> {
	title: &'a str,
	pos: A_Int2,
	size: A_Int2,
	resized: A_Bool,
}

struct A_AudioFormat {
	samples_per_second: u32,
	channels: u32,
	bytes_per_sample: u32,
}

struct A_AudioBuffer<'a> {
	samples: &'a i16,
	samples_count: usize,
	format: A_AudioFormat,
}


// typedef void(*P_AudioCallback)(P_AudioBuffer *buffer);
type A_AudioCallback = fn(buffer: &A_AudioBuffer);

pub struct A_Audio {
	format: A_AudioFormat,
	callback: A_AudioCallback,
}

struct P_Time {
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

struct _XINPUT_STATE;
type XINPUT_STATE = _XINPUT_STATE;

type HANDLE = *mut c_void;

// typedef unsigned long(__stdcall *XINPUTGETSTATE)(unsigned long dwUserIndex, XINPUT_STATE* pState);
type XINPUTGETSTATE = extern "system" fn(dwUserIndex: u64, a_state: &XINPUT_STATE) -> u64; 
 

pub struct IAudioClient;
pub struct IAudioRenderClient;

pub struct A_Win32<'a> {
	window: HANDLE,
	device_context: HANDLE,

	main_fiber: *mut c_void,
	message_fiber: *mut c_void,

	xinput_get_state: XINPUTGETSTATE,

	audio_client: &'a IAudioClient,
	audio_render_client: &'a IAudioRenderClient,

	wgl_context: HANDLE,
}



