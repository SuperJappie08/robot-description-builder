mod cluster_objects;
mod joint;
mod link;
mod material;
mod transform;
mod transmission;
mod utils;

#[macro_export]
macro_rules! impl_into_py_callback {
	($type:ty) => {
		impl pyo3::callback::IntoPyCallbackOutput<*mut pyo3::ffi::PyObject> for $type
		where
			$type: Sized + $crate::utils::TryIntoRefPyAny + $crate::utils::TryIntoPy<PyObject>,
		{
			#[inline]
			fn convert(self, py: Python<'_>) -> PyResult<*mut pyo3::ffi::PyObject> {
				Ok($crate::utils::TryIntoPy::<PyObject>::try_into_py(self, py)?.into_ptr())
			}
		}
	};
}

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn rdf_builder_py(py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	// INTERRESTING IDEA, DICT Constructors...

	// PyO3 + Maturin can only generate a python module, not a convienent package
	// As a result it is easier to export everything flat
	link::init_module(py, m)?;

	transform::init_module(py, m)?;

	material::init_module(py, m)?;

	joint::init_module(py, m)?;

	transmission::init_module(py, m)?;

	cluster_objects::init_module(py, m)?;

	Ok(())
}
