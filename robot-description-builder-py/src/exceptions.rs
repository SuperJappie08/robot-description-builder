use pyo3::{create_exception, exceptions::PyException, prelude::*};
use robot_description_builder::errors;

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add("AddJointError", py.get_type::<AddJointError>())?;

	Ok(())
}

create_exception!(
	"robot_description_builder.exceptions",
	AddJointError,
	PyException
);

impl AddJointError {
	pub fn from(err: errors::AddJointError) -> PyErr {
		AddJointError::new_err((format!("{}", err),))
	}
}
