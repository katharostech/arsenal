//! Provides a language agnostic scripting interface for the Bevy game engine designed to be used in [Arsenal]
//!
//! [Arsenal]: https://github.com/katharostech/arsenal

#![warn(missing_docs)]

use std::{fs::OpenOptions, path::PathBuf};

use bevy::prelude::Plugin;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

pub mod bindings;

mod types;
pub use types::*;

/// The extension for dynamic library   
#[cfg(target_os = "linux")]
const SHARED_LIB_EXT: &str = "so";
#[cfg(target_os = "windows")]
const SHARED_LIB_EXT: &str = "dll";
#[cfg(target_os = "macos")]
const SHARED_LIB_EXT: &str = "dylib";

/// The Arsenal scripting plugin for Bevy
pub struct ScriptingPlugin {
    /// The path to the script dir containing all of the game's scripts
    script_path: String,
}

impl ScriptingPlugin {
    /// Initialize the scripting plugin by providing a path to the script dir
    pub fn new(script_path: &str) -> Self {
        ScriptingPlugin {
            script_path: script_path.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct AdapterInfo {
    module_path: String,
}

#[derive(WrapperApi)]
struct LanguageAdapterCApi {
    init_plugin: fn(),
}

impl Plugin for ScriptingPlugin {
    fn build(&self, _app: &mut bevy::prelude::AppBuilder) {
        // Get the path to the script dir
        let script_dir = PathBuf::from(&self.script_path);

        // Ensure the path points to a directory
        if !script_dir.is_dir() {
            panic!("Specified script path is not a directory");
        }

        // List the files in the dir
        for item in script_dir.read_dir().expect("Could not read_dir") {
            let item = item.expect("Could not read path in script dir");

            // If the path is a yaml file
            let filename = item.file_name();
            let filename = filename.to_string_lossy();
            if item.path().is_file() && (filename.ends_with("yml") || filename.ends_with("yaml")) {
                // Assume the file is an adapter definition and parse it
                let adapter_file = OpenOptions::new()
                    .read(true)
                    .open(item.path())
                    .expect("Could not open adapter module definition file");
                let adapter_info: AdapterInfo = serde_yaml::from_reader(adapter_file)
                    .expect("Could not parse adapter module definition file");

                // Get the path to the adapter module
                let module_path = item
                    .path()
                    .parent()
                    .expect("Path without parent!")
                    .join(format!("{}.{}", adapter_info.module_path, SHARED_LIB_EXT));

                if !module_path.exists() {
                    panic!("Module path does not exist: {:?}", module_path);
                }

                // Load the adapter module
                let module: Container<LanguageAdapterCApi> =
                    unsafe { Container::load(module_path) }
                        .expect("Could not load adapter module file");

                // Initialize the module
                module.init_plugin();
            }
        }
    }
}
