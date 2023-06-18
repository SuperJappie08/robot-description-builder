// It is not needed to use super expansion macro for this type
use robot_description_builder::{
	smart_joint_extension::{smartparams::*, types::FixedType},
	SmartJointBuilder,
};

// TODO:
pub struct PyFixedJointBuilder {
	inner: SmartJointBuilder<
		FixedType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	>,
}
