use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_float, c_double};

pub type ALboolean = c_char;

pub type ALchar    = c_char;

pub type ALbyte    = c_char;
pub type ALubyte   = c_uchar;

pub type ALshort  = c_short;
pub type ALushort = c_ushort;

pub type ALint  = c_int;
pub type ALuint = c_uint;

pub type ALsizei = c_int;
pub type ALenum  = c_int;

pub type ALfloat  = c_float;
pub type ALdouble = c_double;

pub type ALvoid = c_void;

pub const AL_NONE: ALint = 0;

pub const AL_FALSE: ALboolean = 0;
pub const AL_TRUE:  ALboolean = 1;

pub const AL_SOURCE_RELATIVE: c_int = 0x202;

pub const AL_CONE_INNER_ANGLE: c_int = 0x1001;
pub const AL_CONE_OUTER_ANGLE: c_int = 0x1002;

pub const AL_PITCH:     c_int= 0x1003;
pub const AL_POSITION:  c_int = 0x1004;
pub const AL_DIRECTION: c_int = 0x1005;
pub const AL_VELOCITY:  c_int = 0x1006;
pub const AL_LOOPING:   c_int = 0x1007;
pub const AL_BUFFER:    c_int = 0x1009;

pub const AL_GAIN:     c_int = 0x100A;
pub const AL_MIN_GAIN: c_int = 0x100D;
pub const AL_MAX_GAIN: c_int = 0x100E;

pub const AL_ORIENTATION: c_int = 0x100F;

pub const AL_SOURCE_STATE: c_int = 0x1010;

pub const AL_INITIAL: c_int = 0x1011;
pub const AL_PLAYING: c_int = 0x1012;
pub const AL_PAUSED:  c_int = 0x1013;
pub const AL_STOPPED: c_int = 0x1014;

pub const AL_BUFFERS_QUEUED: c_int = 0x1015;
pub const AL_BUFFERS_PROCESSED: c_int = 0x1016;

pub const AL_REFERENCE_DISTANCE: c_int = 0x1020;
pub const AL_ROLLOF_FACTOR:      c_int = 0x1021;
pub const AL_CONE_OUTER_GAIN:    c_int = 0x1022;
pub const AL_MAX_DISTANCE:       c_int = 0x1023;

pub const AL_SEC_OFFSET:    c_int = 0x1024;
pub const AL_SAMPLE_OFFSET: c_int = 0x1025;
pub const AL_BYTE_OFFSET:   c_int = 0x1026;

pub const AL_SOURCE_TYPE: c_int = 0x1027;

pub const AL_STATIC:       c_int = 0x1028;
pub const AL_STREAMING:    c_int = 0x1029;
pub const AL_UNDETERMINED: c_int = 0x1030;

pub const AL_FORMAT_MONO8:    c_int = 0x1100;
pub const AL_FORMAT_MONO16:   c_int = 0x1101;
pub const AL_FORMAT_STEREO8:  c_int = 0x1102;
pub const AL_FORMAT_STEREO16: c_int = 0x1103;

pub const AL_FREQUENCY: c_int = 0x2001;
pub const AL_BITS:      c_int = 0x2002;
pub const AL_CHANNELS:  c_int = 0x2003;
pub const AL_SIZE:      c_int = 0x2004;

pub const AL_UNUSED:    c_int = 0x2010;
pub const AL_PENDING:   c_int = 0x2011;
pub const AL_PROCESSED: c_int = 0x2012;

pub const AL_NO_ERROR:          c_int = 0;
pub const AL_INVALID_NAME:      c_int = 0xA001;
pub const AL_INVALID_ENUM:      c_int = 0xA002;
pub const AL_INVALID_VALUE:     c_int = 0xA003;
pub const AL_INVALID_OPERATION: c_int = 0xA004;
pub const AL_OUT_OF_MEMORY:     c_int = 0xA005;

pub const AL_VENDOR:     c_int = 0xB001;
pub const AL_VERSION:    c_int = 0xB002;
pub const AL_RENDERER:   c_int = 0xB003;
pub const AL_EXTENSIONS: c_int = 0xB004;

pub const AL_DOPPLER_FACTOR:   c_int = 0xC000;
pub const AL_DOPPLER_VELOCITY: c_int = 0xC001;
pub const AL_SPEED_OF_SOUND:   c_int = 0xC003;

pub const AL_DISTANCE_MODEL:            c_int = 0xD000;
pub const AL_INVERSE_DISTANCE:          c_int = 0xD001;
pub const AL_INVERSE_DISTANCE_CLAMPED:  c_int = 0xD002;
pub const AL_LINEAR_DISTANCE:           c_int = 0xD003;
pub const AL_LINEAR_DISTANCE_CLAMPED:   c_int = 0xD004;
pub const AL_EXPONENT_DISTANCE:         c_int = 0xD005;
pub const AL_EXPONENT_DISTANCE_CLAMPED: c_int = 0xD006;

#[link(name = "openal")]
extern {
	pub fn alDopplerFactor(value: ALfloat);
	pub fn alDopplerVelocity(value: ALfloat);
	pub fn alSpeedOfSound(value: ALfloat);
	pub fn alDistanceModel(distanceModel: ALenum);

	pub fn alEnable(capability: ALenum);
	pub fn alDisable(capability: ALenum);
	pub fn alIsEnabled(capability: ALenum) -> ALboolean;

	pub fn alGetString(param: ALenum) -> *const ALchar;
	pub fn alGetBooleanv(param: ALenum, values: *mut ALboolean);
	pub fn alGetIntegerv(param: ALenum, values: *mut ALint);
	pub fn alGetFloatv(param: ALenum, values: *mut ALfloat);
	pub fn alGetDoublev(param: ALenum, values: *mut ALdouble);
	pub fn alGetBoolean(param: ALenum) -> ALboolean;
	pub fn alGetInteger(param: ALenum) -> ALint;
	pub fn alGetFloat(param: ALenum) -> ALfloat;
	pub fn alGetDouble(param: ALenum) -> ALdouble;

	pub fn alGetError() -> ALenum;

	pub fn alIsExtensionPresent(extname: *const ALchar) -> ALboolean;
	pub fn alGetProcAddress(fname: *const ALchar) -> *mut c_void;
	pub fn alGetEnumValue(ename: *const ALchar) -> ALenum;

	pub fn alListenerf(param: ALenum, value: ALfloat);
	pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
	pub fn alListenerfv(param: ALenum, values: *const ALfloat);

	pub fn alListeneri(param: ALenum, value: ALint);
	pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
	pub fn alListeneriv(param: ALenum, values: *const ALint);

	pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
	pub fn alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
	pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);

	pub fn alGetListeneri(param: ALenum, value: *mut ALint);
	pub fn alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
	pub fn alGetListeneriv(param: ALenum, values: *mut ALint);

	pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
	pub fn alDeleteSources(n: ALsizei, source: *const ALuint);
	pub fn alIsSource(source: ALuint) -> ALboolean;

	pub fn alSourcef(source: ALuint, param: ALenum, value: ALfloat);
	pub fn alSource3f(source: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
	pub fn alSourcefv(source: ALuint, param: ALenum, values: *const ALfloat);

	pub fn alSourcei(source: ALuint, param: ALenum, value: ALint);
	pub fn alSource3i(source: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
	pub fn alSourceiv(source: ALuint, param: ALenum, values: *const ALint);

	pub fn alGetSourcef(source: ALuint, param: ALenum, value: *mut ALfloat);
	pub fn alGetSource3f(source: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
	pub fn alGetSourcefv(source: ALuint, param: ALenum, values: *mut ALfloat);

	pub fn alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint);
	pub fn alGetSource3i(source: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
	pub fn alGetSourceiv(source: ALuint, param: ALenum, values: *mut ALint);

	pub fn alSourcePlayv(n: ALsizei, sources: *const ALuint);
	pub fn alSourceStopv(n: ALsizei, sources: *const ALuint);
	pub fn alSourceRewindv(n: ALsizei, sources: *const ALuint);
	pub fn alSourcePausev(n: ALsizei, sources: *const ALuint);

	pub fn alSourcePlay(source: ALuint);
	pub fn alSourceStop(source: ALuint);
	pub fn alSourceRewind(source: ALuint);
	pub fn alSourcePause(source: ALuint);

	pub fn alSourceQueueBuffers(source: ALuint, nb: ALsizei, buffers: *const ALuint);
	pub fn alSourceUnqueueBuffers(source: ALuint, nb: ALsizei, buffers: *mut ALuint);

	pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
	pub fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
	pub fn alIsBuffer(buffer: ALuint) -> ALboolean;

	pub fn alBufferData(buffer: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei);

	pub fn alBufferf(buffer: ALuint, param: ALenum, value: ALfloat);
	pub fn alBuffer3f(buffer: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
	pub fn alBufferfv(buffer: ALuint, param: ALenum, values: *const ALfloat);

	pub fn alBufferi(buffer: ALuint, param: ALenum, value: ALint);
	pub fn alBuffer3i(buffer: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
	pub fn alBufferiv(buffer: ALuint, param: ALenum, values: *const ALint);

	pub fn alGetBufferf(buffer: ALuint, param: ALenum, value: *mut ALfloat);
	pub fn alGetBuffer3f(buffer: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
	pub fn alGetBufferfv(buffer: ALuint, param: ALenum, values: *mut ALfloat);

	pub fn alGetBufferi(buffer: ALuint, param: ALenum, value: *mut ALint);
	pub fn alGetBuffer3i(buffer: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
	pub fn alGetBufferiv(buffer: ALuint, param: ALenum, values: *mut ALint);
}
