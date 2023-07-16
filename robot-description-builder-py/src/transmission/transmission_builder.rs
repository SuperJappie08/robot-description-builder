use super::{
	transmission_wrappers::PyTransmissionActuator, PyTransmissionJointBuilder, PyTransmissionType,
};
use itertools::Itertools;
use pyo3::{intern, prelude::*};
use robot_description_builder::transmission::{
	TransmissionBuilder, TransmissionJointBuilder, WithActuator, WithJoints,
};

#[derive(Debug, Clone)]
#[pyclass(
	name = "TransmissionBuilder",
	module = "robot_description_builder.transmission"
)]
pub struct PyTransmissionBuilder(TransmissionBuilder<WithJoints, WithActuator>);

#[pymethods]
impl PyTransmissionBuilder {
	#[new]
	fn py_new(
		name: String,
		transmission_type: PyTransmissionType,
		#[pyo3(from_py_with = "crate::utils::one_or_list")] joints: Vec<PyTransmissionJointBuilder>,
		#[pyo3(from_py_with = "crate::utils::one_or_list")] actuators: Vec<PyTransmissionActuator>,
	) -> PyResult<Self> {
		let transmission_builder = TransmissionBuilder::new(name, transmission_type.into());

		let mut joints = joints.into_iter().map(Into::into);
		let transmission_builder = transmission_builder.add_joint(joints.next().unwrap());

		let transmission_builder = joints.fold(
			transmission_builder,
			|builder, joint: TransmissionJointBuilder| builder.add_joint(joint),
		);

		let mut actuators = actuators.into_iter().map(Into::into);
		let transmission_builder = transmission_builder.add_actuator(actuators.next().unwrap());

		let transmission_builder = actuators.fold(transmission_builder, |builder, actuator| {
			builder.add_actuator(actuator)
		});

		Ok(Self(transmission_builder))
	}

	// TODO: Setter
	#[getter]
	fn get_name(&self) -> String {
		self.0.name().clone()
	}

	// TODO: Setter
	#[getter]
	fn get_type(&self) -> PyResult<PyTransmissionType> {
		(*self.0.transmission_type()).try_into()
	}

	#[getter]
	fn get_joints(&self) -> PyResult<Vec<PyTransmissionJointBuilder>> {
		self.0
			.joints()
			.unwrap() // Unwrap Ok
			.iter()
			.cloned()
			.map(TryInto::try_into)
			.process_results(|iter| iter.collect())
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let data = format!(
			"name='{}', type={}",
			self.get_name(),
			self.get_type()?.repr()
		);

		Ok(format!("{class_name}({data})"))
	}
}
