use pyo3::prelude::*;

#[pymodule]
fn test_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}