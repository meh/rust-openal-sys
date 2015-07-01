#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case, non_upper_case_globals)]

extern crate libc;

mod al;
pub use al::*;

mod alc;
pub use alc::*;

#[cfg_attr(all(feature = "static", target_os = "linux"), link(name = "openal", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "linux"), link(name = "openal"))]

#[cfg_attr(all(feature = "static", target_os = "macos"), link(name = "OpenAL", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "macos"), link(name = "OpenAL", kind = "framework"))]

#[cfg_attr(all(feature = "static", target_os = "windows"), link(name = "OpenAL32", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "windows"), link(name = "OpenAL32"))]
extern { }
