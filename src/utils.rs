use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn print_py_value(py: Python, value: PyObject) -> PyResult<()> {
    let locals = [
        ("value", value)
    ].into_py_dict(py);

    py.run("print(value)", None, Some(locals))?;

    Ok(())
}
