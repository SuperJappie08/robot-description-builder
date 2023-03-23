use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{JointBuilder, JointInterface},
	link::Link,
	transform_data::TransformData,
	ArcLock, JointType, WeakLock,
};

#[derive(Debug)]
pub struct FixedJoint {
	name: String,
	tree: WeakLock<KinematicTreeData>,
	parent_link: WeakLock<Link>,
	child_link: ArcLock<Link>,
	origin: TransformData,
	me: WeakLock<Box<dyn JointInterface + Sync + Send>>,
}

impl FixedJoint {
	pub(crate) fn new(
		name: String,
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		origin: TransformData,
	) -> ArcLock<Box<dyn JointInterface + Send + Sync>> {
		Arc::new_cyclic(|me| {
			RwLock::new(
				Self {
					name,
					tree,
					parent_link,
					child_link,
					origin,
					me: Weak::clone(me),
				}
				.into(),
			)
		})
	}
}

impl JointInterface for FixedJoint {
	fn get_name(&self) -> &String {
		&self.name
	}

	fn get_jointtype(&self) -> JointType {
		JointType::Fixed
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

	fn get_origin(&self) -> &TransformData {
		&self.origin
	}

	fn rebuild(&self) -> JointBuilder {
		let mut builder = JointBuilder::new(self.name.clone(), JointType::Fixed);
		dbg!(self.get_origin());
		if let Some(translation) = self.get_origin().translation {
			builder.add_origin_offset(translation);
		}

		if let Some(rotation) = self.get_origin().rotation {
			builder.add_origin_rotation(rotation);
		}

		builder
	}

	fn get_self(&self) -> ArcLock<Box<dyn JointInterface + Sync + Send>> {
		self.me.upgrade().unwrap()
	}
}

impl Into<Box<dyn JointInterface + Sync + Send>> for FixedJoint {
	fn into(self) -> Box<dyn JointInterface + Sync + Send> {
		Box::new(self)
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
