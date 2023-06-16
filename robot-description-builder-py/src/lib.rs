mod cluster_objects;
mod exceptions;
mod identifier;
mod joint;
mod link;
mod material;
mod to_rdf;
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

// use identifier::PyGroupIDChangable;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn rdf_builder_py(py: Python, m: &PyModule) -> PyResult<()> {
	// INTERRESTING IDEA, DICT Constructors...

	// PyO3 + Maturin can only generate a python module, not a convienent package
	// As a result it is easier to export everything flat
	link::init_module(py, m)?;

	transform::init_module(py, m)?;

	material::init_module(py, m)?;

	joint::init_module(py, m)?;

	transmission::init_module(py, m)?;

	cluster_objects::init_module(py, m)?;

	identifier::init_module(py, m)?;

	exceptions::init_module(py, m)?;

	to_rdf::init_module(py, m)?;

	Ok(())
}
