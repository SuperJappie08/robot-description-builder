mod transmission_builder;
mod transmission_joint;
mod transmission_variants;
pub(self) mod transmission_wrappers;

use std::sync::{Arc, RwLock, Weak};

use itertools::Itertools;
use pyo3::{intern, prelude::*};
use robot_description_builder::transmission::Transmission;

use crate::{
	joint::PyJoint,
	utils::{PyReadWriteable, TryIntoRefPyAny},
};

pub use transmission_builder::PyTransmissionBuilder;
pub use transmission_joint::PyTransmissionJointBuilder;
pub use transmission_variants::{PyTransmissionHardwareInterface, PyTransmissionType};

use transmission_wrappers::PyTransmissionActuator;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyTransmissionBuilder>()?;
	module.add_class::<PyTransmission>()?;
	module.add_class::<PyTransmissionType>()?;
	module.add_class::<PyTransmissionHardwareInterface>()?;

	Ok(())
}

#[derive(Debug)]
#[pyclass(
	name = "Transmission",
	module = "robot_description_builder.transmission",
	frozen
)]
pub struct PyTransmission {
	inner: Weak<RwLock<Transmission>>,
	tree: PyObject,
}

impl PyTransmission {
	fn try_internal(&self) -> PyResult<Arc<RwLock<Transmission>>> {
		match self.inner.upgrade() {
			Some(l) => Ok(l),
			None => Err(pyo3::exceptions::PyReferenceError::new_err(
				"Transmission already dropped",
			)),
		}
	}
}

#[pymethods]
impl PyTransmission {
	#[getter]
	fn get_name(&self) -> PyResult<String> {
		Ok(self.try_internal()?.py_read()?.name().clone())
	}

	#[getter]
	fn get_transmission_type(&self) -> PyResult<PyTransmissionType> {
		self.try_internal()?
			.py_read()?
			.transmission_type()
			.try_into()
	}

	#[getter]
	fn get_joints<'py>(&self, py: Python<'py>) -> PyResult<Vec<&'py PyAny>> {
		let py_joints = py
			.import(intern!(py, "robot_description_builder.transmission"))?
			.getattr(intern!(py, "TransmissionJoint"))?;

		self.try_internal()?
			.py_read()?
			.joints()
			.iter()
			.map(|trans_joint| {
				match trans_joint
					.hardware_interfaces()
					.iter()
					.copied()
					.map(TryInto::<PyTransmissionHardwareInterface>::try_into)
					.process_results(|iter| iter.collect_vec())
				{
					Ok(hw_interfaces) => py_joints.call_method1(
						intern!(py, "__new__"),
						(
							Into::<PyJoint>::into((trans_joint.joint(), self.tree.clone())),
							hw_interfaces,
						),
					),
					Err(e) => Err(e),
				}
			})
			.process_results(|iter| iter.collect())
	}

	#[getter]
	fn get_actuators<'py>(&self, py: Python<'py>) -> PyResult<Vec<&'py PyAny>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.actuators()
			.iter()
			.cloned()
			.map(|actuator| {
				Into::<PyTransmissionActuator>::into(actuator)
					.try_into_py_ref(py)
					.unwrap()
			})
			.collect())
	}

	fn rebuild(&self, py: Python<'_>) -> PyResult<PyTransmissionBuilder> {
		py.version();
		todo!()
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = format!(
			"'{}', {}, joints=[",
			self.try_internal()?.py_read()?.name(),
			self.get_transmission_type()?.repr()
		);

		data += self
			.get_joints(py)?
			.into_iter()
			.map(|joint| joint.repr())
			.process_results(|mut iter| iter.join(", "))?
			.as_str();

		data += "], actuators=[";
		data += self
			.get_actuators(py)?
			.into_iter()
			.map(|py_actuator| {
				py_actuator
					// .try_into_py_ref(py)?
					.repr()
					.and_then(|val| val.extract::<String>())
			})
			.process_results(|mut iter| iter.join(", "))?
			.as_str();
		data += "]";

		Ok(format!("{class_name}({data})"))
	}
}

impl From<(Weak<RwLock<Transmission>>, PyObject)> for PyTransmission {
	fn from(value: (Weak<RwLock<Transmission>>, PyObject)) -> Self {
		// TODO: Maybe add check for weakref
		Self {
			inner: value.0,
			tree: value.1,
		}
	}
}

impl From<(Arc<RwLock<Transmission>>, PyObject)> for PyTransmission {
	fn from(value: (Arc<RwLock<Transmission>>, PyObject)) -> Self {
		// TODO: Maybe add check for weakref
		Self {
			inner: Arc::downgrade(&value.0),
			tree: value.1,
		}
	}
}
