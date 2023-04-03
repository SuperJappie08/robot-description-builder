use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, link::Link, ArcLock, Joint,
	KinematicTree, WeakLock,
};

mod linkbuilder;
// mod visual_builder;

pub use linkbuilder::LinkBuilder;
// pub use visual_builder::VisualBuilder;

pub trait BuildLink {
	/// TODO: THE BUILDER IS ALLOWED TO BUILD JOINTS FOR THIS BEAST, Maybe not for end users but might be usefull for cloning;
	fn build(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link>;

	fn build_tree(self) -> KinematicTree
	where
		Self: Sized,
	{
		let data = KinematicDataTree::newer_link(self);
		KinematicTree::new(data)
	}

	fn start_building_chain(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link>;
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link>;
}

impl<T: BuildLink> From<T> for KinematicTree {
	fn from(value: T) -> Self {
		value.build_tree()
	}
}
