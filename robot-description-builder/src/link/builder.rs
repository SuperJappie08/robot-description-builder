use std::sync::Weak;

use super::{Link, LinkShapeData};
use crate::{
	cluster_objects::{kinematic_data_tree::KinematicDataTree, KinematicTree},
	joint::Joint,
	utils::{ArcLock, WeakLock},
};

mod collision_builder;
mod linkbuilder;
mod visual_builder;

pub use collision_builder::CollisionBuilder;
pub use linkbuilder::LinkBuilder;
pub use visual_builder::VisualBuilder;

// FIXME: Split the trait into multiple traits, because it does not make sense in all situations
pub(crate) trait BuildLink {
	// TODO: THE BUILDER IS ALLOWED TO BUILD JOINTS FOR THIS BEAST, Maybe not for end users but might be usefull for cloning;
	fn build(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link>;

	fn build_tree(self) -> KinematicTree
	where
		Self: Sized,
	{
		let data = KinematicDataTree::new(self);
		KinematicTree::new(data)
	}

	// TODO: Make internal
	// TODO: Maybe move to `LinkChainBuilder`
	/// Starts building the Kinematic Chain, the `tree` argument is not yet intitialized at this point.
	fn start_building_chain(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link>;

	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link>;

	// TODO: Rename?
	fn get_shape_data(&self) -> LinkShapeData;
}

impl<T: BuildLink> From<T> for KinematicTree {
	fn from(value: T) -> Self {
		value.build_tree()
	}
}
