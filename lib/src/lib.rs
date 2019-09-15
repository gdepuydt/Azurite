
#[cfg(target_os = "windows")]
mod win32;

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
			self.x = 0.0_f32;
		} else {
			self.x = x;
		}

		if y.abs() <= self.threshold {
			self.y = 0.0_f32;
		} else {
			self.y = y;
		}
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

pub struct Window<'a> {
	title: &'a str,
	pos: Int2,
	size: Int2,
	resized: bool,
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


// typedef void(*P_AudioCallback)(P_AudioBuffer *buffer);
type AudioCallback = fn(buffer: &AudioBuffer);

pub struct A_Audio {
	format: AudioFormat,
	callback: AudioCallback,
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
