use pyo3::{intern, prelude::*, types::PyDict};
use robot_description_builder::JointBuilder;

use crate::{link::PyLinkBuilder, transform::PyTransform};

use super::PyJointType;

#[derive(Debug, Default)]
#[pyclass(
	name = "JointBuilderBase",
	module = "robot_description_builder.joint",
	subclass
)]
pub struct PyJointBuilderBase {
	builder: JointBuilder,
	transform: Option<Py<PyTransform>>,
}

/// These functions probally should become pub(super)
impl PyJointBuilderBase {
	pub(crate) fn new(py: Python<'_>, builder: JointBuilder) -> PyResult<Self> {
		Ok(Self {
			transform: match builder
				.transform()
				.copied()
				.map(Into::into)
				.map(|transform: PyTransform| Py::new(py, transform))
			{
				Some(Ok(obj)) => Ok(Some(obj)),
				None => Ok(None),
				Some(Err(err)) => Err(err),
			}?,
			builder,
		})
	}

	pub(crate) fn update(&mut self, builder: JointBuilder, py: Python<'_>) -> PyResult<()> {
		self.builder = builder;
		match (&mut self.transform, self.builder.transform().copied()) {
			(None, None) => Ok(()),
			(transform, None) => {
				*transform = None;
				Ok(())
			}
			(Some(obj), Some(transform)) => {
				*obj.borrow_mut(py) = transform.into();
				Ok(())
			}
			(field @ None, Some(transform)) => {
				*field = Some(Py::new(py, Into::<PyTransform>::into(transform))?);
				Ok(())
			}
		}
	}
}

#[pymethods]
impl PyJointBuilderBase {
	#[getter]
	fn get_name(&self) -> String {
		self.builder.name().clone()
	}

	#[getter]
	fn get_type(&self) -> PyJointType {
		(*self.builder.joint_type()).into()
	}

	#[getter]
	fn get_transform(&self) -> Option<Py<PyTransform>> {
		// TODO: How to now if updated
		// Might be able to use pre-existing technique
		self.transform.clone()
	}

	// TODO: I want this on this object however do not know how to pass back to inheriter
	// I can probably overwrit it when necessary
	// #[setter]
	// fn set_transform(&mut self)

	#[getter]
	fn get_child(&self) -> Option<PyLinkBuilder> {
		self.builder.child().cloned().map(Into::into)
	}

	#[getter]
	fn get_axis(&self) -> Option<(f32, f32, f32)> {
		self.builder.axis()
	}

	/// TODO: BETTER TYPE (falling, rising)
	#[getter]
	fn get_calibration(&self) -> Option<(Option<f32>, Option<f32>)> {
		let data = self.builder.calibration();

		match data.contains_some() {
			true => Some((data.falling, data.rising)),
			false => None,
		}
	}

	/// TODO: BETTER TYPE (friction, damping)
	#[getter]
	fn get_dynamics(&self) -> Option<(Option<f32>, Option<f32>)> {
		let data = self.builder.dynamics();

		match data.contains_some() {
			true => Some((data.friction, data.damping)),
			false => None,
		}
	}

	/// TODO: Better types
	#[getter]
	fn get_limit(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
		match self.builder.limit() {
			Some(limit) => {
				let dict = PyDict::new(py);
				dict.set_item(intern!(py, "velocity"), limit.velocity)?;
				dict.set_item(intern!(py, "effort"), limit.effort)?;

				if limit.lower.is_some() || limit.upper.is_some() {
					dict.set_item(
						intern!(py, "lower"),
						limit.lower.unwrap_or(f32::NEG_INFINITY),
					)?;
					dict.set_item(intern!(py, "upper"), limit.upper.unwrap_or(f32::INFINITY))?;
				}

				Ok(Some(unsafe {
					Py::from_owned_ptr_or_err(
						py,
						pyo3::ffi::PyDictProxy_New(dict.as_mapping().into_ptr()),
					)?
				}))
			}
			None => Ok(None),
		}
	}

	/// TODO: Propper types
	#[getter]
	fn get_mimic(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
		match self.builder.mimic() {
			Some(mimic) => {
				let dict = PyDict::new(py);
				dict.set_item(intern!(py, "name"), mimic.joint_name.clone())?;
				dict.set_item(intern!(py, "multiplier"), mimic.multiplier)?;
				dict.set_item(intern!(py, "offset"), mimic.offset)?;

				Ok(Some(unsafe {
					Py::from_owned_ptr_or_err(
						py,
						pyo3::ffi::PyDictProxy_New(dict.as_mapping().into_ptr()),
					)?
				}))
			}
			None => Ok(None),
		}
	}

	/// TODO: Propper types
	#[getter]
	fn get_safety_controller(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
		match self.builder.safety_controller() {
			Some(safety_controller) => {
				let dict = PyDict::new(py);
				dict.set_item(intern!(py, "k_velocity"), safety_controller.k_velocity)?;
				dict.set_item(intern!(py, "k_position"), safety_controller.k_position)?;

				// Not very cool
				dict.set_item(
					intern!(py, "soft_lower_limit"),
					safety_controller.soft_lower_limit,
				)?;
				dict.set_item(
					intern!(py, "soft_upper_limit"),
					safety_controller.soft_upper_limit,
				)?;

				Ok(Some(unsafe {
					Py::from_owned_ptr_or_err(
						py,
						pyo3::ffi::PyDictProxy_New(dict.as_mapping().into_ptr()),
					)?
				}))
			}
			None => Ok(None),
		}
	}
}
