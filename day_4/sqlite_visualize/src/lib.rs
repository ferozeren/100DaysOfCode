use pyo3::prelude::*;
use rusqlite::Connection;

#[pyclass(unsendable)]
struct DBConnection {
    conn: Connection,
}

#[pymethods]
impl DBConnection {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        let conn = Connection::open(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(DBConnection { conn })
    }

    fn create_table(&self, table_name: String) -> PyResult<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, data TEXT)",
            table_name
        );
        self.conn
            .execute(&sql, [])
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }

    fn show_tables(&self) -> PyResult<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        let table_names = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(table_names)
    }

    fn delete_table(&self, table_name: String) -> PyResult<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn
            .execute(&sql, [])
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }
}

#[pymodule]
fn sqlite_visualize(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DBConnection>()?;
    Ok(())
}
