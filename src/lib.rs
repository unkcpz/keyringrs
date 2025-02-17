use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

// Replace these imports with the actual path to your crate that defines them.
use keyring::{Entry, Error};

/// Convert crate's `Error` to a Python `PyErr`.
fn to_py_err(err: Error) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyclass(name = "Entry")]
#[derive(Debug)]
struct PyEntry {
    inner: Entry,
}

#[pymethods]
impl PyEntry {

    /// This mirrors `Entry::new(service, user)`.
    #[new]
    pub fn new(service: &str, user: &str) -> PyResult<Self> {
        let entry = Entry::new(service, user).map_err(to_py_err)?;
        Ok(PyEntry { inner: entry })
    }

    /// `Entry::set_password(&self, password)`
    pub fn set_password(&self, password: &str) -> PyResult<()> {
        self.inner.set_password(password).map_err(to_py_err)
    }

    /// `Entry::get_password(&self) -> String`
    pub fn get_password(&self) -> PyResult<String> {
        self.inner.get_password().map_err(to_py_err)
    }

    /// `Entry::delete_credential(&self) -> ()`
    pub fn delete_credential(&self) -> PyResult<()> {
        self.inner.delete_credential().map_err(to_py_err)
    }

    /// For a nice display of Entry object
    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// The Python module definition.
#[pymodule]
fn keyringrs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEntry>()?;
    Ok(())
}
