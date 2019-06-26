pub mod py {
    use crate::utils::blender::get_build_dir;
    use crate::gltf;
    use pyo3::prelude::*;

    /// Blender operators
    #[pymodule]
    pub fn operators(_py: Python, module: &PyModule) -> PyResult<()> {
        /// Run the Arsenal game
        #[pyfn(module, "arsenal_run")]
        fn arsenal_run(py: Python, _context: PyObject) -> PyResult<()> {
            #[cfg(feature = "enable_profiling")]
            println!("Operator: arsenal_run");

            // Export the blend
            gltf::export(py, &get_build_dir(py)?)?;

            // Run the exported scene in Amethyst
            // TODO

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
