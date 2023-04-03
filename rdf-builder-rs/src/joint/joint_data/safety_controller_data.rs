#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SafetyControllerData {
	///(optional, defaults to 0)
	///
	/// An attribute specifying the lower joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be larger than the lower joint limit (see above). See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	pub soft_lower_limit: Option<f32>,
	/// (optional, defaults to 0)
	///
	/// An attribute specifying the upper joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be smaller than the upper joint limit (see above). See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	pub soft_upper_limit: Option<f32>,
	///  (optional, defaults to 0)
	///
	/// An attribute specifying the relation between position and velocity limits. See See safety limits for more details.
	/// TODO: FIX DOCUMENTATION
	pub k_position: Option<f32>,
	/// An attribute specifying the relation between effort and velocity limits. See See safety limits for more details.
	pub k_velocity: f32,
}
