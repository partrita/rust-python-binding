use pyo3::prelude::*;

#[pyfunction]
fn run(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    run(n - 1) + run(n - 2)
}

#[pymodule]
fn fibonacci(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_function(wrap_pyfunction!(run, py)?)?;
    Ok(())
}
