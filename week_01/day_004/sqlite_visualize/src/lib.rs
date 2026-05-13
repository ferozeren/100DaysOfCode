use pyo3::prelude::*;
use rusqlite::{params, Connection};
use std::sync::Mutex;

#[pyclass]
struct DBConnection {
    conn: Mutex<Connection>,
}

#[pymethods]
impl DBConnection {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        let conn = Connection::open(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(DBConnection {
            conn: Mutex::new(conn),
        })
    }

    fn create_table(&self, table_name: String) -> PyResult<()> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (id INTEGER PRIMARY KEY, data TEXT)",
            table_name
        );
        conn.execute(&sql, [])
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }

    fn add_row(&self, table_name: String, data: String) -> PyResult<()> {
        let conn = self.conn.lock().unwrap();
        let sql = format!("INSERT INTO {} (data) VALUES (?1)", table_name);
        conn.execute(&sql, params![data])
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }

    fn get_rows(&self, table_name: String) -> PyResult<Vec<(i32, String)>> {
        let conn = self.conn.lock().unwrap();

        // This assumes every table has 'id' and 'data' columns
        let sql = format!("SELECT id, data FROM {}", table_name);
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok(rows)
    }
    fn show_tables(&self) -> PyResult<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
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
        let conn = self.conn.lock().unwrap();
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        conn.execute(&sql, [])
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(())
    }
}

#[pymodule]
fn sqlite_visualize(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DBConnection>()?;
    Ok(())
}
