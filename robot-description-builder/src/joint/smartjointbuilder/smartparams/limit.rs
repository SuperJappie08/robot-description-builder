use crate::joint::{
	joint_data,
	jointbuilder::JointBuilder,
	smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder},
};

/// TODO: Maybe add a continous lockout thing
pub trait LimitAllowed {}

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
			origin: self.origin,
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

	pub fn set_velocity(mut self, velocity: f32) -> Self {
		self.limit.velocity = velocity;
		self
	}

	/// TODO: maybe restrict
	pub fn set_upper_limit(mut self, upper_limit: f32) -> Self {
		self.limit.upper = Some(upper_limit);
		self
	}

	/// TODO: maybe restrict
	pub fn set_lower_limit(mut self, lower_limit: f32) -> Self {
		self.limit.lower = Some(lower_limit);
		self
	}
}
