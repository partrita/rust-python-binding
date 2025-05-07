use pyo3::prelude::*;

#[pyfunction]
fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[pymodule]
fn fibonacci(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_function(wrap_pyfunction!(fib, py)?)?;
    Ok(())
}

