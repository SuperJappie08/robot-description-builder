use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{Joint, JointType},
	link::Link,
	linkbuilding::{BuildLink, LinkBuilder},
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

pub(crate) trait BuildJointChain: BuildJoint {
	fn build_chain(
		self,
		tree: &WeakLock<KinematicTreeData>,
		parent_link: &WeakLock<Link>,
	) -> ArcLock<Joint>;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct JointBuilder {
	pub(crate) name: String,
	pub(crate) joint_type: JointType, // TODO: FINISH ME
	pub(crate) origin: TransformData,
	pub(crate) child: Option<LinkBuilder>,
}

impl JointBuilder {
	pub fn new<Name: Into<String>>(name: Name, joint_type: JointType) -> Self {
		Self {
			name: name.into(),
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

	/// Nominated for Deprication
	pub(crate) fn with_origin(&mut self, origin: TransformData) -> &mut Self {
		self.origin = origin;
		self
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
				parent_link,
				child_link,
				joint_type: self.joint_type,
				origin: self.origin,
				me: Weak::clone(me),
			})
		});

		Self::register_to_tree(&tree, &joint).unwrap(); // FIX unwrap;
		joint
	}
}

impl BuildJointChain for JointBuilder {
	fn build_chain(
		self,
		tree: &WeakLock<KinematicTreeData>,
		parent_link: &WeakLock<Link>,
	) -> ArcLock<Joint> {
		#[cfg(any(feature = "logging", test))]
		log::trace!("Building a Joint[name ='{}']", self.name);

		Arc::new_cyclic(|me| {
			RwLock::new(Joint {
				name: self.name,
				tree: Weak::clone(tree),
				parent_link: Weak::clone(parent_link),
				child_link: self.child.unwrap().build_chain(tree, me),
				joint_type: self.joint_type,
				origin: self.origin,
				me: Weak::clone(me),
			})
		})
	}
}
