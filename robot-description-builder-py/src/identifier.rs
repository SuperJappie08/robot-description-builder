use pyo3::{exceptions::PyException, intern, prelude::*};
use robot_description_builder::identifiers;

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add("GroupIDError", py.get_type::<GroupIDError>())?;

	Ok(())
}

//TODO: improve docs
pyo3::create_exception!("robot_description_builder", GroupIDError, PyException, "An error which can be returned when checking for a [`GroupID`]'s validity. This error is used as an error type for functions which check for [`GroupID`] validity such as [`GroupID::is_valid_group_id`]");

impl GroupIDError {
	pub fn from(err: identifiers::GroupIDError) -> PyErr {
		GroupIDError::new_err((format!("{}", err),))
	}
}

/// TODO: This should probably be a subclass

#[pyclass(
	name = "GroupIDChangable",
	module = "robot_description_builder",
	subclass
)]
pub struct PyGroupIDChangable;

#[pymethods]
impl PyGroupIDChangable {
	fn change_group_id(&mut self, new_group_id: String, py: Python<'_>) -> PyResult<()> {
		let qualname = py
			.get_type::<Self>()
			.getattr(intern!(py, "change_group_id"))?
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;
		Err(pyo3::exceptions::PyNotImplementedError::new_err((format!(
			"{} is not implemented. ({{'new_group_id': '{}'}})",
			qualname, new_group_id
		),)))
	}

	fn apply_group_id(&mut self, py: Python<'_>) -> PyResult<()> {
		let qualname = py
			.get_type::<Self>()
			.getattr(intern!(py, "apply_group_id"))?
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;
		Err(pyo3::exceptions::PyNotImplementedError::new_err((format!(
			"{} is not implemented.",
			qualname
		),)))
	}
}
