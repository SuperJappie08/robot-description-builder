use pyo3::{create_exception, exceptions::PyException, prelude::*};
use robot_description_builder::{errors, reexport::quick_xml};

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add("AttachChainError", py.get_type::<AttachChainError>())?;
	module.add("RebuildBranchError", py.get_type::<AttachChainError>())?;

	module.add("XMLError", py.get_type::<XMLError>())?;

	Ok(())
}

// TODO: DOC
create_exception!(
	"robot_description_builder.exceptions",
	AttachChainError,
	PyException
);

impl AttachChainError {
	pub fn from(err: errors::AttachChainError) -> PyErr {
		AttachChainError::new_err((format!("{}", err),))
	}
}

// TODO: DOC
create_exception!(
	"robot_description_builder.exceptions",
	RebuildBranchError,
	PyException
);

impl RebuildBranchError {
	pub fn from(err: errors::RebuildBranchError) -> PyErr {
		RebuildBranchError::new_err((format!("{}", err),))
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
