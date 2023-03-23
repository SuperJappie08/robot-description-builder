use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{Joint, JointType},
	link::Link,
	transform_data::TransformData,
	ArcLock, WeakLock,
};

pub trait BuildJoint {
	/// Creates the joint ?? and subscribes it to the right right places
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint>;

	fn register_to_tree(
		tree: &WeakLock<KinematicTreeData>,
		joint: &ArcLock<Joint>,
	) -> Result<(), crate::cluster_objects::kinematic_data_errors::AddJointError> {
		tree.upgrade()
			.unwrap()
			.try_write()
			.unwrap()
			.try_add_joint(joint)
	}

	// fn register_to_link(parent_link: &WeakLock<Link>, joint: ArcLock<Box<dyn JointInterface + Sync + Send>>) {
	// 	parent_link.upgrade().unwrap().try_write().unwrap()
	// }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct JointBuilder {
	name: String,
	joint_type: JointType, // TODO: FINISH ME
	origin: TransformData,
}

impl JointBuilder {
	pub fn new(name: String, joint_type: JointType) -> Self {
		Self {
			name,
			joint_type,
			..Default::default()
		}
	}

	pub fn add_origin_offset(&mut self, offset: (f32, f32, f32)) -> &mut Self {
		self.origin.translation = Some(offset);
		self
	}

	pub fn add_origin_rotation(&mut self, rotation: (f32, f32, f32)) -> &mut Self {
		self.origin.rotation = Some(rotation);
		self
	}

	/// For now return a Specific Joint maybe go dyn JointInterface
	#[deprecated]
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		Arc::new_cyclic(|me| -> RwLock<Joint> {
			RwLock::new(Joint {
				name: self.name,
				tree: tree,
				parent_link: parent_link,
				child_link: child_link,
				joint_type: self.joint_type,
				origin: self.origin,
				me: Weak::clone(me),
			})
		})
	}
}

impl BuildJoint for JointBuilder {
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let joint = Arc::new_cyclic(|me| -> RwLock<Joint> {
			RwLock::new(Joint {
				name: self.name,
				tree: Weak::clone(&tree),
				parent_link: parent_link,
				child_link: child_link,
				joint_type: self.joint_type,
				origin: self.origin,
				me: Weak::clone(me),
			})
		});

		Self::register_to_tree(&tree, &joint).unwrap(); // FIX unwrap;
		joint
		// Box::new(self.build(tree, parent_link, child_link))
	}
}
