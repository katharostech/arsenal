pub mod py {
    use std::path::PathBuf;
    use std::process::Command;
    use crate::utils::python::get_arsenal_plugin_path;
    use crate::utils::blender::get_build_dir;
    use crate::exporter;
    use pyo3::prelude::*;
    use pyo3::exceptions::IOError;

    /// Blender operators
    #[pymodule]
    pub fn operators(_py: Python, module: &PyModule) -> PyResult<()> {
        /// Run the Arsenal game
        #[pyfn(module, "arsenal_run")]
        fn arsenal_run(py: Python, _context: PyObject) -> PyResult<()> {
            #[cfg(feature = "enable_profiling")]
            println!("Operator: arsenal_run");

            // Get build dir
            let build_dir = get_build_dir(py)?;

            // Export the blend
            exporter::export(py, &build_dir)?;

            // Get arsenal runtime binary path
            let arsenal_runtime_path =
                PathBuf::from(get_arsenal_plugin_path(py)?)
                .join("bin")
                .join("arsenal-runtime");
            let arsenal_runtime_path = arsenal_runtime_path
                .to_str()
                .ok_or_else(|| IOError::py_err("Path to arsenal-runtime not valid UTF-8"))?;

            // Run the exported scene with the arsenal runtime
            Command::new(arsenal_runtime_path) 
                .current_dir(&build_dir)
                .arg(PathBuf::from(&build_dir)
                    .join("scene.ron")
                    .to_str()
                    .ok_or_else(|| IOError::py_err("Build dir path not valid UTF-8"))?)
                .spawn()?;

            // Dump flamegraph
            #[cfg(feature = "enable_profiling")]
            {
                flame::dump_stdout();
                flame::dump_html(std::fs::File::create("flamegraph_arsenal_run.html").unwrap())
                    .unwrap();
            }

            Ok(())
        }

        Ok(())
    }
}
