use crate::joint::smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder};

pub trait DynamicsAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoDynamics;
impl smart_joint_datatraits::DynamicsDataType for NoDynamics {}

#[derive(Debug, Default, Clone)]
pub struct WithDynamics {
	damping: Option<f32>,
	friction: Option<f32>,
}
impl smart_joint_datatraits::DynamicsDataType for WithDynamics {}

impl<Type, Axis, Calibration, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, NoDynamics, Limit, Mimic, SafetyController>
where
	Type: DynamicsAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
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
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
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
