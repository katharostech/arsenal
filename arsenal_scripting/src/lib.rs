//! Provides a language agnostic scripting interface for the Bevy game engine designed to be used in [Arsenal]
//!
//! [Arsenal]: https://github.com/katharostech/arsenal

#![warn(missing_docs)]

#[macro_use]
extern crate dlopen_derive;
use arsenal_scripting_types::LanguageAdapterInitArgsC;
use dlopen::wrapper::WrapperApi;

mod bevy_plugin;
mod bindings;
mod ffi;
mod metadata;
mod type_registry;
mod tools;
mod utils;

pub use bevy_plugin::*;
pub use tools::*;

/// The extension for dynamic libraries on the target platform
#[cfg(target_os = "linux")]
pub(crate) const SHARED_LIB_EXT: &str = "so";
/// The extension for dynamic libraries on the target platform
#[cfg(target_os = "windows")]
pub(crate) const SHARED_LIB_EXT: &str = "dll";
/// The extension for dynamic libraries on the target platform
#[cfg(target_os = "macos")]
pub(crate) const SHARED_LIB_EXT: &str = "dylib";

/// The C API implemented by language adapters
#[derive(WrapperApi)]
pub(crate) struct LanguageAdapterCApi {
    init_adapter: fn(args: &LanguageAdapterInitArgsC),
    get_components: fn() -> safer_ffi::Vec<u8>,
}
