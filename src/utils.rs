/// Python utilities
pub mod python {
    use std::path::PathBuf;
    use pyo3::prelude::*;
    use pyo3::exceptions::{FileNotFoundError, IOError, Exception};
    use pyo3::types::{PyDict, IntoPyDict};

    /// Print a Python object as it would be printed by the Python interpreter.
    pub fn print_py_value(py: Python, value: PyObject) -> PyResult<()> {
        // Put value into local scope
        let locals = [("value", value)].into_py_dict(py);

        // Print value with Python's `print` function
        py.run("print(value)", None, Some(locals))?;

        Ok(())
    }

    /// Get the path to the arsenal blender addon directory
    pub fn get_arsenal_plugin_path(py: Python) -> PyResult<String> {
        // Import the sys package
        let sys = py.import("sys")?.to_object(py);

        // Path to the arsenal-blender Python module's __init__.py file
        let module_path: String = sys
            .getattr(py, "modules")?
            .cast_as::<PyDict>(py)?
            .get_item("arsenal-blender")
            .ok_or_else(|| Exception::py_err("Could not load arsenal-blender module"))?
            .to_object(py)
            .getattr(py, "__spec__")?
            .getattr(py, "origin")?
            .extract(py)?;
        let module_path = PathBuf::from(module_path);
        
        // Path to the arsenal-blend addon directory
        let arsenal_plugin_path = module_path
            .parent()
            .ok_or_else(|| FileNotFoundError::py_err("Cannot find arsenal-blender addon dir"))?
            .to_str()
            .ok_or_else(|| IOError::py_err("arsenal-blender addon path not valid UTF-8"))?;

        Ok(arsenal_plugin_path.into())
    }

    /// Get the path to the arsenal_runtime executable
    pub fn get_arsenal_runtime_path(py: Python) -> PyResult<String> {
        let arsenal_runtime_path = PathBuf::from(get_arsenal_plugin_path(py)?)
            .join("bin")
            .join("arsenal-runtime");

        Ok(arsenal_runtime_path
            .to_str()
            .ok_or_else(|| IOError::py_err("Path to arsenal-runtime not valid UTF-8"))?
            .into())
    }
}

/// Blender utilities
pub mod blender {
    use std::path::PathBuf;
    use std::fs::DirBuilder;
    use pyo3::exceptions::{FileNotFoundError, IOError};
    use pyo3::prelude::*;

    /// Get the blend filepath. This will be None if the blend has not been saved
    /// yet.
    pub fn get_blend_file_path(py: Python) -> PyResult<Option<String>> {
        // Import bpy
        let bpy = py.import("bpy")?;

        // Get blend filepath
        let filepath = bpy
            .to_object(py)
            .getattr(py, "context")?
            .getattr(py, "blend_data")?
            .getattr(py, "filepath")?
            .extract(py)?;

        if filepath == "" {
            // Blend has not been saved
            Ok(None)
        } else {
            // Return blend file path
            Ok(Some(filepath))
        }
    }

    /// Get the build directory for the Arsenal game. Creates the directory if
    /// it doesn't exist.
    pub fn get_build_dir(py: Python) -> PyResult<String> {
        // Get blend file path
        let blend_file_path = get_blend_file_path(py)?.ok_or_else(|| {
            FileNotFoundError::py_err("Blend not saved. Save blend before exporting.")
        })?;
        let blend_file_path = PathBuf::from(blend_file_path);

        // Set the export dir
        let parent_dir = blend_file_path
            .parent()
            .ok_or_else(|| IOError::py_err("Could not get parent dir of blend file."))?;

        // Name of the build dir
        let build_dir_name = format!(
            "build_{}",
            blend_file_path
                .file_stem()
                .ok_or_else(|| IOError::py_err("Blend file name error"))?
                .to_str()
                .ok_or_else(|| IOError::py_err("Build dir path not valid UTF-8"))?
        );

        // Full export dir path
        let build_dir_path = parent_dir.to_path_buf().join(build_dir_name);

        // Create path if it doesn't exist
        DirBuilder::new()
            .recursive(true)
            .create(&build_dir_path)?;

        // Return path string
        Ok(build_dir_path
            .to_str()
            .ok_or_else(|| IOError::py_err("Build dir path not valid UTF-8"))?
            .into())
    }
}
