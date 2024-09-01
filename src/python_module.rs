use pyo3::{Bound, pyfunction, pymodule, PyResult, wrap_pyfunction};
use pyo3::prelude::{PyModule, PyModuleMethods};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(crate::core::add(a, b))
}

/// A Python module implemented in Rust.
#[pymodule]
fn united_multilanguage_project(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}