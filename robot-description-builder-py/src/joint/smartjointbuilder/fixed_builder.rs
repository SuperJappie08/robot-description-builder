use rdb_py_macro::enum_generic_state;
use robot_description_builder::smart_joint_extension::smartparams::*;
use robot_description_builder::{smart_joint_extension::types::FixedType, SmartJointBuilder};

enum_generic_state!(SmartJointBuilder {[FixedType], [NoAxis], [NoCalibration], [NoDynamics], [NoLimit], [NoMimic], [NoSafetyController]});
