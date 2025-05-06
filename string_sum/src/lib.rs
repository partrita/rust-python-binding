use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// You can create a module using #[pymodule]:
#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}


/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

/// This module is implemented in Rust.
#[pymodule]
fn double(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    Ok(())
}


/// Python submodules
///You can create a module hierarchy within a single extension module by using Bound<'_, PyModule>::add_submodule(). For example, you could define the modules parent_module and parent_module.child_module.

#[pymodule]
fn parent_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "child_module")?;
    child_module.add_function(wrap_pyfunction!(func, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}

#[pyfunction]
fn func() -> String {
    "func".to_string()
}