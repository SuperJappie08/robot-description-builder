use crate::joint::smartjointbuilder::SmartJointBuilder;

/// TODO: Maybe add a continous lockout thing
pub trait LimitAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoLimit;

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

impl<Type, Axis, Calibration, Dynamics, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, NoLimit, Mimic, SafetyController>
where
	Type: LimitAllowed,
{
	pub fn with_limit(
		self,
		effort: f32,
		velocity: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
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
