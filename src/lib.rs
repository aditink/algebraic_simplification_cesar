use pyo3::prelude::*;

// Exposes the cesar module to the crate root
pub mod cesar;
pub mod language;
pub mod z3utils;
pub mod config;
pub mod simplify;

#[pyfunction]
fn light_simplify(expr: String, assumptions: String) -> PyResult<()> {
    crate::simplify::light_simplify(expr, assumptions);
    Ok(())
}

#[pyfunction]
fn aggressive_simplify(expr: String, assumptions: String) -> PyResult<()> {
    crate::simplify::aggressive_simplify(expr, assumptions);
    Ok(())
}

#[pymodule]
fn pysimplify(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(light_simplify, m)?)?;
    m.add_function(wrap_pyfunction!(aggressive_simplify, m)?)?;
    Ok(())
}
