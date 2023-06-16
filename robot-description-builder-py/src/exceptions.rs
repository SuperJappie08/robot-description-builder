use pyo3::{create_exception, exceptions::PyException, prelude::*};
use robot_description_builder::{errors, reexport::quick_xml};

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add("AddJointError", py.get_type::<AddJointError>())?;
	module.add("AddLinkError", py.get_type::<AddLinkError>())?;

	module.add("XMLError", py.get_type::<XMLError>())?;

	Ok(())
}

// TODO: DOC
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

// TODO: DOC
create_exception!(
	"robot_description_builder.exceptions",
	AddLinkError,
	PyException
);

impl AddLinkError {
	pub fn from(err: errors::AddLinkError) -> PyErr {
		AddLinkError::new_err((format!("{}", err),))
	}
}

// TODO: DOC
create_exception!(
	"robot_description_builder.exceptions",
	XMLError,
	PyException
);

impl XMLError {
	pub fn from(err: quick_xml::Error) -> PyErr {
		XMLError::new_err(format!("{}", err))
	}
}
