use crate::joint::smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder};

pub trait MimicAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoMimic;
impl smart_joint_datatraits::MimicDataType for NoMimic {}

///  (optional) (New with ROS Groovy. See issue)
///
/// This tag is used to specify that the defined joint mimics another existing joint. The value of this joint can be computed as value = multiplier * other_joint_value + offset.
/// TODO: Write better docs
#[derive(Debug, Default, Clone)]
pub struct WithMimic {
	/// This specifies the name of the joint to mimic.
	joint_name: String,
	/// Specifies the multiplicative factor in the formula above.
	multiplier: Option<f32>,
	/// Specifies the offset to add in the formula above. Defaults to 0 (radians for revolute joints, meters for prismatic joints)
	offset: Option<f32>,
}
impl smart_joint_datatraits::MimicDataType for WithMimic {}

impl<Type, Axis, Calibration, Dynamics, Limit, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, NoMimic, SafetyController>
where
	Type: MimicAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn with_mimic(
		self,
		mimiced_joint_name: String,
	) -> SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, WithMimic, SafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: WithMimic {
				joint_name: mimiced_joint_name,
				..Default::default()
			},
			safety_controller: self.safety_controller,
		}
	}
}

impl<Type, Axis, Calibration, Dynamics, Limit, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, WithMimic, SafetyController>
where
	Type: MimicAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn set_mimiced_joint_name(mut self, mimiced_joint_name: String) -> Self {
		self.mimic.joint_name = mimiced_joint_name;
		self
	}

	pub fn set_mimic_multiplier(mut self, multiplier: f32) -> Self {
		self.mimic.multiplier = Some(multiplier);
		self
	}

	/// Specifies the offset to add in the formula above. Defaults to 0 (radians for revolute joints, meters for prismatic joints)
	pub fn set_mimic_offset(mut self, offset: f32) -> Self {
		self.mimic.offset = Some(offset);
		self
	}
}
