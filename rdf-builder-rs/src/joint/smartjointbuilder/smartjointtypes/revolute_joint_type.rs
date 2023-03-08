use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{
		smartjointbuilder::{
			smartparams::{smart_joint_specification, WithLimit},
			SmartJointBuilder,
		},
		BuildJoint,
	},
	link::Link,
};

#[derive(Debug, Default, Clone)]
pub struct RevoluteType;

impl smart_joint_specification::AxisAllowed for RevoluteType {}
impl smart_joint_specification::CalibrationAllowed for RevoluteType {}
impl smart_joint_specification::DynamicsAllowed for RevoluteType {}
impl smart_joint_specification::LimitAllowed for RevoluteType {}
impl smart_joint_specification::MimicAllowed for RevoluteType {}
impl smart_joint_specification::SafetyControllerAllowed for RevoluteType {}

impl<Axis, Calibration, Dynamics, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<RevoluteType, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController>
{
	fn build(
		self,
		tree: Weak<RwLock<KinematicTreeData>>,
		parent_link: Weak<RwLock<Link>>,
		child_link: Arc<RwLock<Link>>,
	) -> crate::Joint {
		todo!()
		// crate::Joint { name: , tree: (), parent_link: (), child_link: (), joint_type: () }
	}
}
