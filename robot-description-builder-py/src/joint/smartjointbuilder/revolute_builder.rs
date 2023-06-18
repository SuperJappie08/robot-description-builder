use rdb_py_macro::enum_generic_state;
use robot_description_builder::{
	smart_joint_extension::{smartparams::*, types::RevoluteType},
	SmartJointBuilder,
};

enum_generic_state!(SmartJointBuilder RevoluteType
	[NoAxis, WithAxis]
	[NoCalibration, WithCalibration]
	[NoDynamics, WithDynamics]
	[NoLimit, WithLimit]
	[NoMimic, WithMimic]
	[NoSafetyController, WithSafetyController]
);
