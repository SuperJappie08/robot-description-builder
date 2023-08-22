use crate::joint::{
	jointbuilder::JointBuilder,
	smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder},
};

/// A trait to label a `SmartJointType` that is allowed to have an axis.
pub trait AxisAllowed {}

/// A type to significy that no [`axis`](JointBuilder::axis) was specified.
#[derive(Debug, Default, Clone)]
pub struct NoAxis;
impl smart_joint_datatraits::AxisDataType for NoAxis {}

#[derive(Debug, Default, Clone)]
pub struct WithAxis(f32, f32, f32);
impl smart_joint_datatraits::AxisDataType for WithAxis {
	fn simplify(&self, joint_builder: &mut JointBuilder) {
		joint_builder.with_axis((self.0, self.1, self.2));
	}
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: AxisAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn with_axis(
		self,
		axis: (f32, f32, f32),
	) -> SmartJointBuilder<Type, WithAxis, Calibration, Dynamics, Limit, Mimic, SafetyController> {
		let length = f32::sqrt(axis.0 * axis.0 + axis.1 * axis.1 + axis.2 * axis.2);
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			transform: self.transform,
			axis: WithAxis(axis.0 / length, axis.1 / length, axis.2 / length),
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

impl<Type, Calibration, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, WithAxis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: AxisAllowed,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn axis(&self) -> (f32, f32, f32) {
		(self.axis.0, self.axis.1, self.axis.2)
	}
}
