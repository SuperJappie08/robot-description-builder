use crate::joint::smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder};

pub trait AxisAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoAxis;
impl smart_joint_datatraits::AxisDataType for NoAxis {}

#[derive(Debug, Default, Clone)]
pub struct WithAxis(f32, f32, f32);
impl smart_joint_datatraits::AxisDataType for WithAxis {}

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
			offset: self.offset,
			rotation: self.rotation,
			axis: WithAxis(axis.0 / length, axis.1 / length, axis.2 / length),
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	// #[deprecated]
	/// TODO: Maybe Deprecate
	pub fn set_axis(
		self,
		axis: (f32, f32, f32),
	) -> SmartJointBuilder<Type, WithAxis, Calibration, Dynamics, Limit, Mimic, SafetyController> {
		self.with_axis(axis)
	}
}