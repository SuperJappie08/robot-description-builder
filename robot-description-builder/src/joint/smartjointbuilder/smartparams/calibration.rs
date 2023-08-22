use crate::joint::{
	joint_data,
	jointbuilder::JointBuilder,
	smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder},
};

/// A trait to label a `SmartJointType` that is allowed to have a calibration element.
pub trait CalibrationAllowed {}

/// A type to significy that no [`Calibration`](joint_data::CalibrationData) was specified.
#[derive(Debug, Default, Clone)]
pub struct NoCalibration;
impl smart_joint_datatraits::CalibrationDataType for NoCalibration {}

#[derive(Debug, Default, Clone)]
pub struct WithCalibration {
	rising: Option<f32>,
	falling: Option<f32>,
}

impl From<WithCalibration> for joint_data::CalibrationData {
	fn from(value: WithCalibration) -> Self {
		Self {
			rising: value.rising,
			falling: value.falling,
		}
	}
}

impl smart_joint_datatraits::CalibrationDataType for WithCalibration {
	fn simplify(&self, joint_builder: &mut JointBuilder) {
		joint_builder.with_calibration_data(self.clone().into());
	}
}

impl<Type, Axis, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, NoCalibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: CalibrationAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn with_calibration(
		self,
	) -> SmartJointBuilder<Type, Axis, WithCalibration, Dynamics, Limit, Mimic, SafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			transform: self.transform,
			axis: self.axis,
			calibration: WithCalibration::default(),
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

impl<Type, Axis, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, WithCalibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: CalibrationAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn set_rising_calibration(mut self, rising: f32) -> Self {
		self.calibration.rising = Some(rising);
		self
	}

	pub fn rising_calibration(&self) -> Option<f32> {
		self.calibration.rising
	}

	pub fn set_falling_calibration(mut self, falling: f32) -> Self {
		self.calibration.falling = Some(falling);
		self
	}

	pub fn falling_calibration(&self) -> Option<f32> {
		self.calibration.falling
	}
}
