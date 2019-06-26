/// Python utilities
pub mod python {
    use pyo3::prelude::*;
    use pyo3::types::IntoPyDict;

    /// Print a Python object as it would be printed by the Python interpreter.
    pub fn print_py_value(py: Python, value: PyObject) -> PyResult<()> {
        // Put value into local scope
        let locals = [("value", value)].into_py_dict(py);

        // Print value with Python's `print` function
        py.run("print(value)", None, Some(locals))?;

        Ok(())
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
