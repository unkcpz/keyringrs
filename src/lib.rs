use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use keyring::{Entry, Error};

#[cfg(target_os = "linux")]
use keyring::keyutils;

/// Convert crate's `Error` to a Python `PyErr`.
fn to_py_err(err: Error) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CredentialType {
    Default = 0,

    #[cfg(target_os = "linux")]
    KeyUtils = 1,
}

#[pyclass(name = "Entry")]
#[derive(Debug)]
pub(crate) struct PyEntry {
    inner: Entry,
}

#[pymethods]
impl PyEntry {
    #[new]
    #[pyo3(signature=(service, user, target = None, credential_type = CredentialType::Default))]
    pub fn new(
        _py: Python,
        service: &str,
        user: &str,
        target: Option<&str>,
        credential_type: CredentialType,
    ) -> PyResult<Self> {
        match credential_type {
            CredentialType::Default => {
                let entry = if let Some(target) = target {
                    Entry::new_with_target(target, service, user).map_err(to_py_err)?
                } else {
                    Entry::new(service, user).map_err(to_py_err)?
                };
                Ok(PyEntry {
                    inner: entry,
                })
            }
            #[cfg(target_os = "linux")]
            CredentialType::KeyUtils => {
                let builder = keyutils::default_credential_builder();
                let credential = builder.build(target, service, user).map_err(to_py_err)?;
                let entry = Entry::new_with_credential(credential);
                Ok(PyEntry {
                    inner: entry,
                })
            }
        }
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

    /// For a nice display of Entry
    fn __str__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// The Python module definition.
#[pymodule]
fn keyringrs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEntry>()?;
    m.add_class::<CredentialType>()?;
    Ok(())
}
