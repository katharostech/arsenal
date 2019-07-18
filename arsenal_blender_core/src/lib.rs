#![feature(proc_macro_hygiene)]

// Include profiling crates
#[cfg(feature = "enable_profiling")]
extern crate flame;
#[cfg(feature = "enable_profiling")]
#[macro_use]
extern crate flamer;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod exporter;
pub mod operators;
pub mod utils;

/// Arsenal Blender Core
/// 
/// This is the core library used to perform most work in the arsenal-blender
/// plugin.
#[pymodule]
fn core(_py: Python, module: &PyModule) -> PyResult<()> {
    // Put module wrappings in their own scopes so that the * import doesn't
    // shadow the parent modules.
    {
        use operators::py::*;
        module.add_wrapped(wrap_pymodule!(operators))?;
    }

    Ok(())
}
