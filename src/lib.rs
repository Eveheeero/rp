use pyo3::prelude::*;

#[pymodule]
fn rp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}

#[pyfunction]
fn hello() -> PyResult<()> {
    println!("Hello World!");
    Ok(())
}
