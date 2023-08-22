use crate::joint::{
	joint_data,
	jointbuilder::JointBuilder,
	smartjointbuilder::{smart_joint_datatraits, SmartJointBuilder},
};

/// A trait to signify if the `SafetyController` element is allowed on a specific `JointType` for the `SmartJointBuilder`.
// TODO: EXPAND
pub trait SafetyControllerAllowed {}

/// A type to significy that no [`SafetyController`](joint_data::SafetyControllerData) was specified.
#[derive(Debug, Default, Clone)]
pub struct NoSafetyController;
impl smart_joint_datatraits::SafetyControllerDataType for NoSafetyController {}

#[derive(Debug, Default, Clone)]
pub struct WithSafetyController {
	// (optional, defaults to 0)
	//
	// An attribute specifying the lower joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be larger than the lower joint limit (see above). See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	soft_lower_limit: Option<f32>,
	// (optional, defaults to 0)
	//
	// An attribute specifying the upper joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be smaller than the upper joint limit (see above). See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	soft_upper_limit: Option<f32>,
	//  (optional, defaults to 0)
	//
	// An attribute specifying the relation between position and velocity limits. See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	k_position: Option<f32>,
	// An attribute specifying the relation between effort and velocity limits. See See safety limits for more details.
	k_velocity: f32,
}

impl From<WithSafetyController> for joint_data::SafetyControllerData {
	fn from(value: WithSafetyController) -> Self {
		Self {
			soft_lower_limit: value.soft_lower_limit,
			soft_upper_limit: value.soft_upper_limit,
			k_position: value.k_position,
			k_velocity: value.k_velocity,
		}
	}
}

impl smart_joint_datatraits::SafetyControllerDataType for WithSafetyController {
	fn simplify(&self, joint_builder: &mut JointBuilder) {
		joint_builder.with_safety_controller(self.clone().into());
	}
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, NoSafetyController>
where
	Type: SafetyControllerAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
{
	pub fn with_safety_controller(
		self,
		k_velocity: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, WithSafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			transform: self.transform,

			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: WithSafetyController {
				k_velocity,
				..Default::default()
			},
		}
	}
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, WithSafetyController>
where
	Type: SafetyControllerAllowed,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
{
	// Defaults 0
	pub fn set_k_position(mut self, k_position: f32) -> Self {
		self.safety_controller.k_position = Some(k_position);
		self
	}

	// Defaults 0
	pub fn k_position(&self) -> Option<f32> {
		self.safety_controller.k_position
	}

	/// Sets the k_velocity limit to the specified value in m/s or rad/s ([`k_velocity`](crate::joint::joint_data::SafetyControllerData::k_velocity)).
	pub fn set_k_velocity(mut self, k_velocity: f32) -> Self {
		self.safety_controller.k_velocity = k_velocity;
		self
	}

	/// Retrieves the set k_velocity limit in m/s or rad/s ([`k_velocity`](crate::joint::joint_data::SafetyControllerData::k_velocity)).
	pub fn k_velocity(&self) -> f32 {
		self.safety_controller.k_velocity
	}
}

/// The (soft) limits are only available on non continuous `JointType`s.
impl<Type, Axis, Calibration, Dynamics, Limit, Mimic>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, WithSafetyController>
where
	Type: SafetyControllerAllowed + smart_joint_datatraits::SmartJointTypeTrait<false>,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
{
	/// Sets the soft lower limit ([`soft_lower_limit`](crate::joint::joint_data::SafetyControllerData::soft_lower_limit)).
	pub fn set_soft_lower_limit(mut self, soft_lower_limit: f32) -> Self {
		self.safety_controller.soft_lower_limit = Some(soft_lower_limit);
		self
	}

	/// Retrieve the specified soft lower limit ([`soft_lower_limit`](crate::joint::joint_data::SafetyControllerData::soft_lower_limit)).
	pub fn soft_lower_limit(&self) -> Option<f32> {
		self.safety_controller.soft_lower_limit
	}

	/// Sets the soft upper limit ([`soft_upper_limit`](crate::joint::joint_data::SafetyControllerData::soft_upper_limit)).
	pub fn set_soft_upper_limit(mut self, soft_upper_limit: f32) -> Self {
		self.safety_controller.soft_upper_limit = Some(soft_upper_limit);
		self
	}

	/// Retrieve the specified soft upper limit ([`soft_upper_limit`](crate::joint::joint_data::SafetyControllerData::soft_upper_limit)).
	pub fn soft_upper_limit(&self) -> Option<f32> {
		self.safety_controller.soft_upper_limit
	}
}
