use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use pyo3::{PyErr, PyResult};

pub trait PoisonErrorHandler<T>: Into<Result<T, PoisonError<T>>> {
	fn to_pyerr(self) -> Result<T, PyErr>;
}

impl<'a, T> PoisonErrorHandler<RwLockReadGuard<'a, T>>
	for Result<RwLockReadGuard<'a, T>, PoisonError<RwLockReadGuard<'a, T>>>
{
	fn to_pyerr(self) -> Result<RwLockReadGuard<'a, T>, PyErr> {
		self.map_err(|_| {
			// pyo3::exceptions::PyAttributeError::new_err("Lock Poisoned")
			pyo3::exceptions::PyRuntimeError::new_err(
				"Tried to read a Lock, which poissoned by a panic.", //, in this Rust version this is unrecoverable.",
			)
		})
	}
}

impl<'a, T> PoisonErrorHandler<RwLockWriteGuard<'a, T>>
	for Result<RwLockWriteGuard<'a, T>, PoisonError<RwLockWriteGuard<'a, T>>>
{
	fn to_pyerr(self) -> Result<RwLockWriteGuard<'a, T>, PyErr> {
		self.map_err(|_| {
			pyo3::exceptions::PyRuntimeError::new_err(
				"Tried to write to Lock, which poissoned by a panic.", //, in this Rust version this is unrecoverable.",
			)
		})
	}
}

pub trait PyReadWriteable<T> {
	fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>>;
	fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>>;
}

impl<T> PyReadWriteable<T> for RwLock<T> {
	fn py_read(&self) -> PyResult<RwLockReadGuard<'_, T>> {
		self.read().to_pyerr()
	}

	fn py_write(&self) -> PyResult<RwLockWriteGuard<'_, T>> {
		self.write().to_pyerr()
	}
}
