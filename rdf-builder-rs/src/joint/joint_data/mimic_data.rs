use std::sync::Weak;

use crate::{cluster_objects::kinematic_data_tree::KinematicDataTree, joint::Joint, WeakLock};

#[derive(Debug, Clone)]
pub struct MimicData {
	pub joint: WeakLock<Joint>,
	pub multiplier: Option<f32>,
	pub offset: Option<f32>,
}

impl PartialEq for MimicData {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.joint, &other.joint)
			&& self.multiplier == other.multiplier
			&& self.offset == other.offset
	}
}

impl From<MimicData> for MimicBuilderData {
	fn from(value: MimicData) -> Self {
		Self {
			joint_name: value
				.joint
				.upgrade()
				.unwrap() // FIXME: Is unwrap Ok?
				.try_read()
				.unwrap() // FIXME: Is unwrap Ok?
				.get_name()
				.clone(),
			multiplier: value.multiplier,
			offset: value.offset,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct MimicBuilderData {
	pub joint_name: String,
	pub multiplier: Option<f32>,
	pub offset: Option<f32>,
}

impl MimicBuilderData {
	pub(crate) fn to_mimic_data(&self, tree: &Weak<KinematicDataTree>) -> MimicData {
		MimicData {
			joint: Weak::clone(
				tree.upgrade()
					.unwrap() // This unwrap is Ok
					.joints
					.try_read()
					.unwrap() // FIXME: Is this unwrap OK?
					.get(&self.joint_name)
					.unwrap(), // FIXME: Is this unwrap OK?
			),
			multiplier: self.multiplier,
			offset: self.offset,
		}
	}
}
