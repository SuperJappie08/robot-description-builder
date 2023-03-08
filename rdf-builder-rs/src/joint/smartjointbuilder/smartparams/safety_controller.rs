use crate::joint::smartjointbuilder::SmartJointBuilder;

pub trait SafetyControllerAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoSafetyController;

#[derive(Debug, Default, Clone)]
pub struct WithSafetyController {
	///(optional, defaults to 0)
	///
	/// An attribute specifying the lower joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be larger than the lower joint limit (see above). See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	soft_lower_limit: Option<f32>,
	/// (optional, defaults to 0)
	///
	/// An attribute specifying the upper joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be smaller than the upper joint limit (see above). See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	soft_upper_limit: Option<f32>,
	///  (optional, defaults to 0)
	///
	/// An attribute specifying the relation between position and velocity limits. See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	k_position: Option<f32>,
	/// An attribute specifying the relation between effort and velocity limits. See See safety limits for more details.
	k_velocity: f32,
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, NoSafetyController>
where
	Type: SafetyControllerAllowed,
{
	pub fn with_safety_controller(
		self,
		k_velocity: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, WithSafetyController> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
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
{
	/// TODO: Add check?
	pub fn set_soft_lower_limit(mut self, soft_lower_limit: f32) -> Self {
		self.safety_controller.soft_lower_limit = Some(soft_lower_limit);
		self
	}

	/// TODO: Add check?
	pub fn set_soft_upper_limit(mut self, soft_upper_limit: f32) -> Self {
		self.safety_controller.soft_upper_limit = Some(soft_upper_limit);
		self
	}

	pub fn set_k_position(mut self, k_position: f32) -> Self {
		self.safety_controller.k_position = Some(k_position);
		self
	}

	pub fn set_k_velocity(mut self, k_velocity: f32) -> Self {
		self.safety_controller.k_velocity = k_velocity;
		self
	}
}
