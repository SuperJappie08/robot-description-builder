use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use itertools::Itertools;
use pyo3::{
	exceptions::{PyIndexError, PyRuntimeError, PyTypeError},
	prelude::*,
	pyclass_init::PyObjectInit,
	sync::GILOnceCell,
	types::{PyCFunction, PyFunction, PySequence, PyType},
	PyClass, PyTypeCheck, PyTypeInfo,
};

pub trait PoisonErrorHandler<T>: Into<Result<T, PoisonError<T>>> {
	fn to_pyerr(self) -> Result<T, PyErr>;
}

impl<'a, T> PoisonErrorHandler<RwLockReadGuard<'a, T>>
	for Result<RwLockReadGuard<'a, T>, PoisonError<RwLockReadGuard<'a, T>>>
{
	fn to_pyerr(self) -> Result<RwLockReadGuard<'a, T>, PyErr> {
		self.map_err(|_| {
			PyRuntimeError::new_err(
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
			PyRuntimeError::new_err(
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
	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>>;
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
			initializer.into_new_object(py, py.get_type_bound::<T>().as_type_ptr())?,
		))
	}
}

pub fn non_empty<'py, T>(obj: &Bound<'py, PySequence>) -> PyResult<Vec<T>>
where
	T: FromPyObject<'py>,
{
	if !obj.is_empty()? {
		obj.extract()
	} else {
		Err(PyIndexError::new_err(format!(
			"Supplied list {} must be non empty",
			obj.repr()?.extract::<&str>()?
		)))
	}
}

pub fn one_or_list<'py, T>(obj: &Bound<'py, PyAny>) -> PyResult<Vec<T>>
where
	T: PyTypeInfo + FromPyObject<'py>,
{
	if PySequence::type_check(obj) {
		non_empty(unsafe { obj.downcast_unchecked() })
	} else if obj.is_instance_of::<T>() {
		Ok(vec![obj.extract()?])
	} else {
		let py = obj.py();
		let target_type = T::type_object_bound(py);
		Err(PyTypeError::new_err(format!(
			"Expected type {target_type} or list[{target_type}] got {} instead.",
			obj.get_type()
		)))
	}
}

/// Implement get_or_try_init_type_ref from PyO3: https://github.com/PyO3/pyo3/blob/1be2fad9bfa900dc2df412e32613641d9175d759/src/sync.rs#L203-L213
pub(crate) trait GILOnceCellTypeExtract {
	fn get_or_try_init_type_ref<'py>(
		&'py self,
		py: Python<'py>,
		module_name: &str,
		attr_name: &str,
	) -> PyResult<&Bound<'py, PyType>>;
}

impl GILOnceCellTypeExtract for GILOnceCell<Py<PyType>> {
	#[inline]
	fn get_or_try_init_type_ref<'py>(
		&'py self,
		py: Python<'py>,
		module_name: &str,
		attr_name: &str,
	) -> PyResult<&Bound<'py, PyType>> {
		self.get_or_try_init(py, || {
			py.import_bound(module_name)?.getattr(attr_name)?.extract()
		})
		.map(|ty| ty.bind(py))
	}
}

pub(crate) trait GILOnceCellFuncExtract<F>
where
	F: PyTypeInfo,
{
	fn get_or_try_init_func_ref<'py>(
		&'py self,
		py: Python<'py>,
		module_name: &str,
		attr_name: &str,
	) -> PyResult<&Bound<'py, F>>;
}

impl GILOnceCellFuncExtract<PyFunction> for GILOnceCell<Py<PyFunction>> {
	#[inline]
	fn get_or_try_init_func_ref<'py>(
		&'py self,
		py: Python<'py>,
		module_name: &str,
		attr_name: &str,
	) -> PyResult<&Bound<'py, PyFunction>> {
		self.get_or_try_init(py, || {
			py.import_bound(module_name)?.getattr(attr_name)?.extract()
		})
		.map(|ty| ty.bind(py))
	}
}

impl GILOnceCellFuncExtract<PyCFunction> for GILOnceCell<Py<PyCFunction>> {
	#[inline]
	fn get_or_try_init_func_ref<'py>(
		&'py self,
		py: Python<'py>,
		module_name: &str,
		attr_name: &str,
	) -> PyResult<&Bound<'py, PyCFunction>> {
		self.get_or_try_init(py, || {
			py.import_bound(module_name)?.getattr(attr_name)?.extract()
		})
		.map(|ty| ty.bind(py))
	}
}
