use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{BuildJointChain, Joint, JointBuilder},
	link::{builder::BuildLink, Link},
	link_data::{self, LinkParent},
	ArcLock, WeakLock,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkBuilder {
	// All fields are pub(crate) so I can struct initialize in rebuild
	pub(crate) name: String,
	pub(crate) visuals: Vec<link_data::Visual>,
	pub(crate) colliders: Vec<link_data::Collision>,
	pub(crate) joints: Vec<JointBuilder>,
}

impl LinkBuilder {
	pub fn new(name: String) -> LinkBuilder {
		Self {
			name,
			..Default::default()
		}
	}

	// /// MAybe
	// pub(crate) fn build(self, tree: ArcLock<KinematicTreeData>) -> ArcLock<Link> {
	//     // Not sure How i wanna do this yet,
	//     // Maybe with colliders and visuals, stacking and calculating the always calculating the endpoint or not?
	// }
}

impl BuildLink for LinkBuilder {
	fn build(self, tree: &WeakLock<KinematicTreeData>) -> ArcLock<Link> {
		#[cfg(any(feature = "logging", test))]
		log::info!("Making a Link[name = \"{}\"]", self.name);

		Arc::new_cyclic(|me| {
			RwLock::new(Link {
				name: self.name,
				tree: Weak::clone(tree),
				direct_parent: Some(LinkParent::KinematicTree(Weak::clone(tree))),
				child_joints: Vec::new(),
				inertial: None, //TODO:
				visuals: self.visuals,
				colliders: self.colliders,
				end_point: None,
				me: Weak::clone(me),
			})
		})
	}

	fn start_building_chain(self, tree: &WeakLock<KinematicTreeData>) -> ArcLock<Link> {
		let joint_builders = self.joints.clone();
		let root = self.build(tree);
		root.write().unwrap().child_joints = joint_builders
			.into_iter()
			.map(|joint_builder| joint_builder.build_chain(tree, &Arc::downgrade(&root)))
			.collect();
		root
	}

	fn build_chain(
		self,
		tree: &WeakLock<KinematicTreeData>,
		parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link> {
		Arc::new_cyclic(|me| {
			RwLock::new(Link {
				name: self.name,
				tree: Weak::clone(tree),
				direct_parent: Some(LinkParent::Joint(Weak::clone(parent_joint))),
				child_joints: self
					.joints
					.into_iter()
					.map(|joint_builder| joint_builder.build_chain(tree, me))
					.collect(),
				inertial: None, // FIXME: Fix this
				visuals: self.visuals,
				colliders: self.colliders,
				end_point: None, // FIXME: Fix this
				me: Weak::clone(me),
			})
		})
	}
}

#[cfg(test)]
mod tests {
	// use test_log::test;

	//TODO: Write test
}
