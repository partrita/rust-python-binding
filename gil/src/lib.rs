use std::thread;
use std::time::Duration;

use pyo3::prelude::*;
use pyo3::types::PyList;

fn _double_list(nums: Vec<i32>) -> Vec<i32> {
    println!("Rust: Release GIL");
    println!("Rust: Wait 1 sec");
    thread::sleep(Duration::from_secs(1));
    let result = nums.into_iter().map(|n| n*2).collect();
    println!("Rust: Resume thread");
    result
}

#[pyfunction]
fn double_list(
    py: Python<'_>,
    list: Bound<'_, PyList>,
    result: Bound<'_, PyList>,
    idx: usize,
) -> PyResult<()> {
    println!("Rust: Enter double_list...");
    py.allow_threads(|| {
        println!("Rust: Release GIL...");
        thread::sleep(Duration::from_secs(1));
    });

    let doubled: Vec<i32> = list.extract::<Vec<i32>>()?.iter().map(|x| x * 2).collect();

    Python::with_gil(|py| {
        println!("Rust: Acquire GIL...");
        thread::sleep(Duration::from_secs(1));
        let py_list = PyList::new(py, &doubled)?;
        println!("Rust: Exit...");
        result.set_item(idx, py_list)
    })
}

#[pymodule]
fn gil(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    m.add_function(wrap_pyfunction!(double_list, py)?)?;
    Ok(())
}
