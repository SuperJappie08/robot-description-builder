use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{Joint, JointType},
	link::Link,
	transform_data::TransformData,
	ArcLock, JointInterface, WeakLock,
};

pub trait BuildJoint {
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> Box<dyn JointInterface + Sync + Send>;
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
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> Joint {
		Joint {
			name: self.name,
			tree: tree,
			parent_link: parent_link,
			child_link: child_link,
			joint_type: self.joint_type,
			origin: self.origin,
		}
	}
}

impl BuildJoint for JointBuilder {
	fn build(
		self,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> Box<dyn JointInterface + Sync + Send> {
		Box::new(self.build(tree, parent_link, child_link))
	}
}
