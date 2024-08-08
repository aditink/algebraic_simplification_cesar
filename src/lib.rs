use pyo3::prelude::*;

// Exposes the cesar module to the crate root
pub mod cesar;

#[pyfunction]
fn light_simplify(expr: String, assumptions: String) -> PyResult<()> {
    crate::cesar::simplify::light_simplify(expr, assumptions);
    Ok(())
}

#[pyfunction]
fn aggressive_simplify(expr: String, assumptions: String) -> PyResult<()> {
    crate::cesar::simplify::aggressive_simplify(expr, assumptions);
    Ok(())
}

#[pymodule]
fn simplify(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(light_simplify, m)?)?;
    m.add_function(wrap_pyfunction!(aggressive_simplify, m)?)?;
    Ok(())
}