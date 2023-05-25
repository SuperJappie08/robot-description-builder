use std::sync::{Arc, RwLock, Weak};

use itertools::{process_results, Itertools};
use pyo3::{intern, prelude::*};
use robot_description_builder::transmission::{
	Transmission, TransmissionBuilder, TransmissionHardwareInterface, TransmissionType,
	WithActuator, WithJoints,
};

use crate::{joint::PyJoint, utils::PyReadWriteable};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyTransmissionBuilder>()?;
	module.add_class::<PyTransmission>()?;
	module.add_class::<PyTransmissionType>()?;
	module.add_class::<PyTransmissionHardwareInterface>()?;

	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(
	name = "TransmissionBuilder",
	module = "robot_description_builder.transmission"
)]
pub struct PyTransmissionBuilder(TransmissionBuilder<WithJoints, WithActuator>);

#[pymethods]
impl PyTransmissionBuilder {
	#[new]
	fn py_new(name: String, transmission_type: PyTransmissionType, joint: String) -> Self {
		let transmission_builder = TransmissionBuilder::new(name, transmission_type.into());
		todo!();
		// let transmission_builder = transmission_builder.add_actuator(todo!()).add_joint(todo!());
		// Self(transmission_builder)
	}
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
	/// FIXME: Return type
	fn get_joints(&self) -> PyResult<Vec<(PyJoint, Vec<PyTransmissionHardwareInterface>)>> {
		process_results(
			self.try_internal()?.py_read()?.joints().iter().map(
				|trans_joint| match process_results(
					trans_joint
						.hardware_interfaces()
						.iter()
						.copied()
						.map(TryInto::try_into),
					|iter| iter.collect(),
				) {
					Ok(hw_interfaces) => Ok((
						(trans_joint.joint(), self.tree.clone()).into(),
						hw_interfaces,
					)),
					Err(e) => Err(e),
				},
			),
			|iter| iter.collect(),
		)
	}

	#[getter]
	/// FIXME: Return type
	fn get_actuators(&self) -> PyResult<Vec<(String, Option<f32>)>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.actuators()
			.iter()
			.map(|actuator| {
				(
					actuator.name().clone(),
					actuator.mechanical_reduction().copied(),
				)
			})
			.collect())
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = format!(
			"'{}', {}, joints=[",
			self.try_internal()?.py_read()?.name(),
			self.get_transmission_type()?.__pyo3__repr__()
		);

		data += self
			.get_joints()?
			.into_iter()
			.map(|(joint, hw_interfaces)| {
				format!(
					"(TODO: {}, [{}])",
					joint.__repr__(py).unwrap(),
					hw_interfaces
						.iter()
						.map(|hw_interface| hw_interface.__pyo3__repr__())
						.join(", ")
				)
			})
			.join(", ")
			.as_str();

		data += "], actuators=[";
		data += self
			.get_actuators()?
			.into_iter()
			.map(|(name, reduction)| {
				format!(
					"(TODO: {name}, {})",
					reduction
						.map(|reduction| reduction.to_string())
						.unwrap_or("None".to_string())
				)
			})
			.join(", ")
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[pyclass(
	name = "TransmissionType",
	module = "robot_description_builder.transmission"
)]
pub enum PyTransmissionType {
	SimpleTransmission,
	DifferentialTransmission,
	FourBarLinkageTransmission,
}

impl TryFrom<TransmissionType> for PyTransmissionType {
	type Error = pyo3::PyErr;

	fn try_from(value: TransmissionType) -> PyResult<Self> {
		match value {
			TransmissionType::SimpleTransmission => Ok(Self::SimpleTransmission),
			TransmissionType::DifferentialTransmission => Ok(Self::DifferentialTransmission),
			TransmissionType::FourBarLinkageTransmission => Ok(Self::FourBarLinkageTransmission),
			other => Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
				"TryFrom<TransmissionType> is not yet implemented for variant '{other:?}'.",
			))),
		}
	}
}

impl From<PyTransmissionType> for TransmissionType {
	fn from(value: PyTransmissionType) -> Self {
		match value {
			PyTransmissionType::SimpleTransmission => Self::SimpleTransmission,
			PyTransmissionType::DifferentialTransmission => Self::DifferentialTransmission,
			PyTransmissionType::FourBarLinkageTransmission => Self::FourBarLinkageTransmission,
		}
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[pyclass(
	name = "TransmissionHardwareInterface",
	module = "robot_description_builder.transmission"
)]
pub enum PyTransmissionHardwareInterface {
	JointCommandInterface,
	EffortJointInterface,
	VelocityJointInterface,
	PositionJointInterface,
	JointStateInterface,
	ActuatorStateInterface,
	EffortActuatorInterface,
	VelocityActuatorInterface,
	PositionActuatorInterface,
	PosVelJointInterface,
	PosVelAccJointInterface,
	ForceTorqueSensorInterface,
	IMUSensorInterface,
}

impl TryFrom<TransmissionHardwareInterface> for PyTransmissionHardwareInterface {
	type Error = pyo3::PyErr;

	fn try_from(value: TransmissionHardwareInterface) -> PyResult<PyTransmissionHardwareInterface> {
		match value {
            TransmissionHardwareInterface::JointCommandInterface => Ok(Self::JointCommandInterface),
            TransmissionHardwareInterface::EffortJointInterface => Ok(Self::EffortJointInterface),
            TransmissionHardwareInterface::VelocityJointInterface => Ok(Self::VelocityJointInterface),
            TransmissionHardwareInterface::PositionJointInterface => Ok(Self::PositionJointInterface),
            TransmissionHardwareInterface::JointStateInterface => Ok(Self::JointStateInterface),
            TransmissionHardwareInterface::ActuatorStateInterface => Ok(Self::ActuatorStateInterface),
            TransmissionHardwareInterface::EffortActuatorInterface => Ok(Self::EffortActuatorInterface),
            TransmissionHardwareInterface::VelocityActuatorInterface => Ok(Self::VelocityActuatorInterface),
            TransmissionHardwareInterface::PositionActuatorInterface => Ok(Self::PositionActuatorInterface),
            TransmissionHardwareInterface::PosVelJointInterface => Ok(Self::PosVelJointInterface),
            TransmissionHardwareInterface::PosVelAccJointInterface => Ok(Self::PosVelAccJointInterface),
            TransmissionHardwareInterface::ForceTorqueSensorInterface => Ok(Self::ForceTorqueSensorInterface),
            TransmissionHardwareInterface::IMUSensorInterface => Ok(Self::IMUSensorInterface),
            other => Err(pyo3::exceptions::PyNotImplementedError::new_err(format!("TryFrom<TransmissionHardwareInterface> is not yet implemented for variant '{other:?}'."))),
        }
	}
}

impl From<PyTransmissionHardwareInterface> for TransmissionHardwareInterface {
	fn from(value: PyTransmissionHardwareInterface) -> Self {
		use PyTransmissionHardwareInterface as PyTrHW;

		match value {
			PyTrHW::JointCommandInterface => Self::JointCommandInterface,
			PyTrHW::EffortJointInterface => Self::EffortJointInterface,
			PyTrHW::VelocityJointInterface => Self::VelocityJointInterface,
			PyTrHW::PositionJointInterface => Self::PositionJointInterface,
			PyTrHW::JointStateInterface => Self::JointStateInterface,
			PyTrHW::ActuatorStateInterface => Self::ActuatorStateInterface,
			PyTrHW::EffortActuatorInterface => Self::EffortActuatorInterface,
			PyTrHW::VelocityActuatorInterface => Self::VelocityActuatorInterface,
			PyTrHW::PositionActuatorInterface => Self::PositionActuatorInterface,
			PyTrHW::PosVelJointInterface => Self::PosVelJointInterface,
			PyTrHW::PosVelAccJointInterface => Self::PosVelAccJointInterface,
			PyTrHW::ForceTorqueSensorInterface => Self::ForceTorqueSensorInterface,
			PyTrHW::IMUSensorInterface => Self::IMUSensorInterface,
		}
	}
}
