mod collect;

use polars::prelude::*;
use pyo3::prelude::*;

#[pyfunction]
fn get_csv(csv_path: &str) -> PyResult<()> {
    collect::collect_csv(PlRefPath::from(csv_path)).unwrap();
    Ok(())
}

#[pymodule]
fn py_rust_web_pipeline(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_csv, m)?)?;
    Ok(())
}
