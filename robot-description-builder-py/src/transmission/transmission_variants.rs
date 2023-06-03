use pyo3::prelude::*;

use robot_description_builder::transmission::{TransmissionHardwareInterface, TransmissionType};

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

impl PyTransmissionType {
	pub(super) fn repr(&self) -> &str {
		self.__pyo3__repr__()
	}
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

impl PyTransmissionHardwareInterface {
	pub(super) fn repr(&self) -> &str {
		self.__pyo3__repr__()
	}
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
