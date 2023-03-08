use crate::joint::smartjointbuilder::SmartJointBuilder;

pub trait DynamicsAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoDynamics;

#[derive(Debug, Default, Clone)]
pub struct WithDynamics {
	damping: Option<f32>,
	friction: Option<f32>,
}

impl<Type, Axis, Calibration, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, NoDynamics, Limit, Mimic, SafetyController>
where
	Type: DynamicsAllowed,
{
	pub fn with_dynamics(
		self,
	) -> SmartJointBuilder<Type, Axis, Calibration, WithDynamics, Limit, Mimic, SafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: WithDynamics::default(),
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

impl<Type, Axis, Calibration, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, WithDynamics, Limit, Mimic, SafetyController>
where
	Type: DynamicsAllowed,
{
	pub fn set_damping(mut self, damping: f32) -> Self {
		self.dynamics.damping = Some(damping);
		self
	}

	pub fn set_friction(mut self, friction: f32) -> Self {
		self.dynamics.friction = Some(friction);
		self
	}
}
