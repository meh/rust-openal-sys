use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_float, c_double};

pub type ALCdevice  = c_void;
pub type ALCcontext = c_void;

pub type ALCboolean = c_char;

pub type ALCchar    = c_char;

pub type ALCbyte    = c_char;
pub type ALCubyte   = c_uchar;

pub type ALCshort  = c_short;
pub type ALCushort = c_ushort;

pub type ALCint  = c_int;
pub type ALCuint = c_uint;

pub type ALCsizei = c_int;
pub type ALCenum  = c_int;

pub type ALCfloat  = c_float;
pub type ALCdouble = c_double;

pub type ALCvoid = c_void;

pub const ALC_FALSE: ALCboolean = 0;
pub const ALC_TRUE:  ALCboolean = 1;

pub const ALC_FREQUENCY: c_int = 0x1007;
pub const ALC_REFRESH:   c_int = 0x1008;
pub const ALC_SYNC:      c_int = 0x1009;

pub const ALC_MONO_SOURCES:   c_int = 0x1010;
pub const ALC_STEREO_SOURCES: c_int = 0x1011;

pub const ALC_NO_ERROR:        c_int = 0;
pub const ALC_INVALID_DEVICE:  c_int = 0xA001;
pub const ALC_INVALID_CONTEXT: c_int = 0xA002;
pub const ALC_INVALID_ENUM:    c_int = 0xA003;
pub const ALC_INVALID_VALUE:   c_int = 0xA004;
pub const ALC_OUT_OF_MEMORY:   c_int = 0xA005;

pub const ALC_MAJOR_VERSION: c_int = 0x1000;
pub const ALC_MINOR_VERSION: c_int = 0x1001;

pub const ALC_ATTRIBUTES_SIZE: c_int = 0x1002;
pub const ALC_ALL_ATTRIBUTES:  c_int = 0x1003;

pub const ALC_DEFAULT_DEVICE_SPECIFIER:         c_int = 0x1004;
pub const ALC_DEVICE_SPECIFIER:                 c_int = 0x1005;
pub const ALC_EXTENSIONS:                       c_int = 0x1006;
pub const ALC_EXT_CAPTURE:                      c_int = 1;
pub const ALC_CAPTURE_DEVICE_SPECIFIER:         c_int = 0x310;
pub const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: c_int = 0x311;
pub const ALC_CAPTURE_SAMPLES:                  c_int = 0x312;
pub const ALC_ENUMERATE_ALL_EXT:                c_int = 1;
pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER:    c_int = 0x1012;
pub const ALC_ALL_DEVICES_SPECIFIER:            c_int = 0x1013;

extern {
	pub fn alcCreateContext(device: *const ALCdevice, attrlsit: *const ALCint) -> *mut ALCcontext;
	pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
	pub fn alcProcessContext(context: *mut ALCcontext);
	pub fn alcSuspendContext(context: *mut ALCcontext);
	pub fn alcDestroyContext(context: *mut ALCcontext);
	pub fn alcGetCurrentContext() -> *const ALCcontext;
	pub fn alcGetContextsDevice(context: *const ALCcontext) -> *mut ALCdevice;

	pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
	pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;

	pub fn alcGetError(device: *const ALCdevice) -> ALCenum;

	pub fn alcIsExtensionPresent(device: *const ALCdevice, extname: *const ALCchar) -> ALCboolean;
	pub fn alcGetProcAddress(device: *const ALCdevice, funcname: *const ALCchar) -> *mut c_void;
	pub fn alcGetEnumValue(device: *const ALCdevice, enumname: *const ALCchar) -> ALCenum;

	pub fn alcGetString(device: *const ALCdevice, param: ALCenum) -> *const ALCchar;
	pub fn alcGetIntegerv(device: *const ALCdevice, param: ALCenum, size: ALCsizei, values: *mut ALCint);

	pub fn alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice;
	pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
	pub fn alcCaptureStart(device: *mut ALCdevice);
	pub fn alcCaptureStop(device: *mut ALCdevice);
	pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
}
