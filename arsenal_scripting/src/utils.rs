//! Utilities used internally to this crate
use dlopen::wrapper::Container;

use std::{collections::HashMap, path::Path};

use crate::{LanguageAdapterCApi, SHARED_LIB_EXT};

pub(crate) fn load_modules(game_dir: &Path) -> HashMap<String, Container<LanguageAdapterCApi>> {
    let mut adapters = HashMap::new();

    // Ensure the path points to a directory
    if !game_dir.is_dir() {
        panic!("Specified script path is not a directory");
    }

    // List the files in the dir
    for item in game_dir.read_dir().expect("Could not read_dir") {
        let item = item.expect("Could not read path in script dir");

        // If the path is a shared library
        let filename = item.file_name();
        let filename = filename.to_string_lossy();
        if item.path().is_file() && filename.ends_with(SHARED_LIB_EXT) {
            let adapter_path = item.path();

            // Load the adapter module
            let module: Container<LanguageAdapterCApi> = unsafe {
                Container::load(&adapter_path).expect("Could not load adapter module file")
            };

            adapters.insert(item.file_name().to_string_lossy().to_string(), module);
        }
    }

    adapters
}
