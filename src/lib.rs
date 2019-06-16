#![feature(proc_macro_hygiene)]

// Include profiling crates
#[cfg(feature = "enable_profiling")]
extern crate flame;
#[cfg(feature = "enable_profiling")]
#[macro_use]
extern crate flamer;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod gltf;
pub mod operators;
pub mod utils;

/// The Arsenal Python extension
///
/// This is a native Rust Python extension used by the arsenal-blender Blender
/// plugin to preform most processing intensive operations.
#[pymodule]
fn arsenal(_py: Python, module: &PyModule) -> PyResult<()> {
    // Put module wrappings in their own scopes so that the * import doesn't
    // shadow the parent modules.
    {
        use operators::py::*;
        module.add_wrapped(wrap_pymodule!(operators))?;
    }

    Ok(())
}
