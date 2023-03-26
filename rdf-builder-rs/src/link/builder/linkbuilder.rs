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
	/// TODO: Figure out if we make this immutable on a `Link` and only allow editting throug the builder.
	pub(crate) visuals: Vec<link_data::Visual>,
	pub(crate) colliders: Vec<link_data::Collision>,
	// TODO: Calulate InertialData?
	// pub(crate) intertial: Option<link_data::InertialData>,
	pub(crate) joints: Vec<JointBuilder>,
}

impl LinkBuilder {
	pub fn new<Name: Into<String>>(name: Name) -> LinkBuilder {
		Self {
			name: name.into(),
			..Default::default()
		}
	}

	pub fn add_visual(&mut self, visual: link_data::Visual) -> &mut Self {
		self.visuals.push(visual);
		self
	}

	pub fn add_collider(&mut self, collider: link_data::Collision) -> &mut Self {
		self.colliders.push(collider);
		self
	}

	// ===== NON BUILDER METHODS =====

	pub fn get_visuals(&self) -> &Vec<link_data::Visual> {
		&self.visuals
	}

	pub fn get_visuals_mut(&mut self) -> &mut Vec<link_data::Visual> {
		&mut self.visuals
	}

	pub fn get_colliders(&self) -> &Vec<link_data::Collision> {
		&self.colliders
	}

	pub fn get_colliders_mut(&mut self) -> &mut Vec<link_data::Collision> {
		&mut self.colliders
	}

	pub fn get_joints(&self) -> &Vec<JointBuilder> {
		&self.joints
	}

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
				direct_parent: LinkParent::KinematicTree(Weak::clone(tree)),
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
		// This unwrap is Ok since the Link has just been build
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
				direct_parent: LinkParent::Joint(Weak::clone(parent_joint)),
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
