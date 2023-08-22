use crate::joint::{
	joint_data,
	jointbuilder::JointBuilder,
	smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder},
};

// TODO: Maybe add a continous lockout thing
/// A trait to label a `SmartJointType` that is allowed to have a limit specified.
pub trait LimitAllowed {}

/// A type to significy that no [`Limit`](joint_data::LimitData) was specified.
#[derive(Debug, Default, Clone)]
pub struct NoLimit;
impl smart_joint_datatraits::LimitDataType for NoLimit {}

#[derive(Debug, Default, Clone)]
pub struct WithLimit {
	/// An attribute specifying the lower joint limit (in radians for revolute joints, in metres for prismatic joints). Omit if joint is continuous.
	lower: Option<f32>,
	/// An attribute specifying the upper joint limit (in radians for revolute joints, in metres for prismatic joints). Omit if joint is continuous.
	upper: Option<f32>,
	/// An attribute for enforcing the maximum joint effort (|applied effort| < |effort|).
	effort: f32,
	/// An attribute for enforcing the maximum joint velocity (in radians per second [rad/s] for revolute joints, in metres per second [m/s] for prismatic joints).
	velocity: f32,
}

impl From<WithLimit> for joint_data::LimitData {
	fn from(value: WithLimit) -> Self {
		Self {
			lower: value.lower,
			upper: value.upper,
			effort: value.effort,
			velocity: value.velocity,
		}
	}
}

impl smart_joint_datatraits::LimitDataType for WithLimit {
	fn simplify(&self, joint_builder: &mut JointBuilder, is_continous: bool) {
		joint_builder.with_limit_data(joint_data::LimitData {
			lower: match is_continous {
				true => None,
				false => self.lower,
			},
			upper: match is_continous {
				true => None,
				false => self.upper,
			},
			effort: self.effort,
			velocity: self.velocity,
		})
	}
}

impl<Type, Axis, Calibration, Dynamics, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, NoLimit, Mimic, SafetyController>
where
	Type: LimitAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn with_limit(
		self,
		effort: f32,
		velocity: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: WithLimit {
				lower: None,
				upper: None,
				effort,
				velocity,
			},
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

impl<Type, Axis, Calibration, Dynamics, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController>
where
	Type: LimitAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn set_effort(mut self, effort: f32) -> Self {
		self.limit.effort = effort;
		self
	}

	pub fn effort(&self) -> f32 {
		self.limit.effort
	}

	/// Sets the velocity limit to the specified value in m/s or rad/s ([`velocity`](crate::joint::joint_data::LimitData::velocity)).
	pub fn set_velocity(mut self, velocity: f32) -> Self {
		self.limit.velocity = velocity;
		self
	}

	/// Retrieves the set velocity limit in m/s or rad/s ([`velocity`](crate::joint::joint_data::LimitData::velocity)).
	pub fn velocity(&self) -> f32 {
		self.limit.velocity
	}
}

/// The limits are only available on non continuous `JointType`s.
impl<Type, Axis, Calibration, Dynamics, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController>
where
	Type: LimitAllowed + smart_joint_datatraits::SmartJointTypeTrait<false>,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	/// Sets the upper limit ([`upper`](crate::joint::joint_data::LimitData::upper)) in meters or radians.
	pub fn set_upper_limit(mut self, upper_limit: f32) -> Self {
		self.limit.upper = Some(upper_limit);
		self
	}

	/// Retrieves the upper limit ([`upper`](crate::joint::joint_data::LimitData::upper)) in meters or radians.
	pub fn upper_limit(&self) -> Option<f32> {
		self.limit.upper
	}

	/// Sets the lower limit ([`lower`](crate::joint::joint_data::LimitData::lower)) in meters or radians.
	pub fn set_lower_limit(mut self, lower_limit: f32) -> Self {
		self.limit.lower = Some(lower_limit);
		self
	}

	/// Retrieves the lower limit ([`lower`](crate::joint::joint_data::LimitData::lower)) in meters or radians.
	pub fn lower_limit(&self) -> Option<f32> {
		self.limit.lower
	}
}
