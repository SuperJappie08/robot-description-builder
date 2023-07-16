use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use itertools::Itertools;
use pyo3::{prelude::*, pyclass_init::PyObjectInit, types::PyList, PyClass, PyTypeInfo};

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

pub trait TryIntoPy<T>: Sized {
	fn try_into_py(self, py: Python<'_>) -> PyResult<T>;
}

pub trait TryIntoRefPyAny: Sized {
	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<&PyAny>;
}

impl<T> TryIntoPy<PyObject> for T
where
	T: TryIntoRefPyAny,
{
	fn try_into_py(self, py: Python<'_>) -> PyResult<PyObject> {
		self.try_into_py_ref(py).map(Into::into)
	}
}
// struct WrapPyObject<T>(T) where T: Sized + TryIntoRefPyAny;

// impl<T> From<T> for WrapPyObject<T> where T: Sized + TryIntoRefPyAny {
//     fn from(value: T) -> Self {
//         Self(value)
//     }
// }

// // https://github.com/PyO3/pyo3/issues/1813
// impl<T> IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for WrapPyObject<T> where T: Sized + TryIntoRefPyAny + TryIntoPy<*mut pyo3::ffi::PyObject> {
//     #[inline]
// 	fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
//         Ok(self.0.try_into_py(py)?)
//     }
// }

// https://github.com/PyO3/pyo3/blob/d71af734568263c986f8ed0c5a73ae62b6e9c0c1/src/callback.rs#LL50C9-L50C29
// impl<T> IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for T where T: TryIntoPy<*mut pyo3::ffi::PyObject>
// {
//     #[inline]
//     fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
//         Ok(self.try_into_py(py)?.into_ptr())
//     }
// }

// // Nearly
// impl<Value,Target> TryIntoPy<Target> for Value where Value: IntoPy<Target> {
//     fn try_into_py<'py>(self, py: Python<'py>) -> PyResult<Target> {
//         Ok(self.into_py(py))
//     }
// }

// impl<Value> IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for Value where Value: TryIntoPy<PyObject> {
//     fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
//         todo!()
//     }
// }

impl<T> TryIntoPy<PyObject> for Vec<T>
where
	T: TryIntoPy<PyObject>,
{
	fn try_into_py(self, py: Python<'_>) -> PyResult<PyObject> {
		let list: Vec<PyObject> = self
			.into_iter()
			.map(|e| e.try_into_py(py))
			.process_results(|iter| iter.collect())?;
		Ok(list.into_py(py))
	}

	#[cfg(feature = "experimental-inspect")]
	fn type_output() -> TypeInfo {
		TypeInfo::list_of(T::type_output())
	}
}

/// WARNING the PyClassInitializer must be complete
pub fn init_pyclass_initializer<T>(
	initializer: PyClassInitializer<T>,
	py: Python<'_>,
) -> PyResult<Py<T>>
where
	T: PyClass,
{
	unsafe {
		Ok(Py::from_owned_ptr(
			py,
			initializer.into_new_object(py, py.get_type::<T>().as_type_ptr())?,
		))
	}
}

pub fn non_empty<'source, T>(obj: &'source PyAny) -> PyResult<Vec<T>>
where
	T: FromPyObject<'source>,
{
	if obj.len()? > 0 {
		obj.extract()
	} else {
		Err(pyo3::exceptions::PyIndexError::new_err(format!(
			"Supplied list {} must be non empty",
			obj.repr()?.extract::<&str>()?
		)))
	}
}

pub fn one_or_list<'source, T>(obj: &'source PyAny) -> PyResult<Vec<T>>
where
	T: PyTypeInfo + FromPyObject<'source>,
{
	if obj.is_instance_of::<PyList>() {
		non_empty(obj)
	} else if obj.is_instance_of::<T>() {
		Ok(vec![obj.extract()?])
	} else {
		let py = obj.py();
		let target_type = T::type_object(py);
		Err(pyo3::exceptions::PyTypeError::new_err(format!(
			"Expected type {target_type} or list[{target_type}] got {} instead.",
			obj.get_type()
		)))
	}
}
