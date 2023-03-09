use std::sync::{Arc, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{JointBuilder, JointInterface},
	link::Link,
	transform_data::TransformData,
	ArcLock, WeakLock,
};

#[derive(Debug)]
pub struct FixedJoint {
	name: String,
	tree: WeakLock<KinematicTreeData>,
	parent_link: WeakLock<Link>,
	child_link: ArcLock<Link>,
	origin: TransformData,
}

impl FixedJoint {
	pub(crate) fn new(
		name: String,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		origin: TransformData,
	) -> Self {
		Self {
			name,
			tree,
			parent_link,
			child_link,
			origin,
		}
	}
}

impl JointInterface for FixedJoint {
	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_parent_link(&self) -> ArcLock<Link> {
		Weak::upgrade(&self.parent_link).unwrap()
	}

	fn get_child_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.child_link)
	}

	fn set_tree(&mut self, tree: WeakLock<KinematicTreeData>) {
		self.tree = tree;
	}

	fn get_transform_data(&self) -> &TransformData {
		&self.origin
	}

	fn rebuild(&self) -> JointBuilder {
		let mut builder = JointBuilder::new(self.name.clone(), crate::JointType::Fixed);
		dbg!(self.get_transform_data());
		if let Some(translation) = self.get_transform_data().translation {
			builder.add_origin_offset(translation);
		}

		if let Some(rotation) = self.get_transform_data().rotation {
			builder.add_origin_rotation(rotation);
		}

		builder
	}
}

#[cfg(test)]
mod tests {
	use crate::{JointBuilder, KinematicInterface, Link, OffsetMode, SmartJointBuilder};

	#[test]
	fn rebuild() {
		let tree = Link::new("root".to_owned());
		tree.get_newest_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::new("child".to_owned()).into(),
				SmartJointBuilder::new("Joint1".to_owned())
					.fixed()
					.add_offset(OffsetMode::Offset(2.0, 3.0, 5.0)),
			)
			.unwrap();

		let rebuilder = tree
			.get_joint("Joint1")
			.unwrap()
			.try_read()
			.unwrap()
			.rebuild();
		assert_eq!(
			rebuilder,
			*JointBuilder::new("Joint1".to_owned(), crate::JointType::Fixed)
				.add_origin_offset((2.0, 3.0, 5.0))
		)
	}
}
