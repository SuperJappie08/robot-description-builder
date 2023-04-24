use std::sync::Weak;

use crate::{
	chained::{ChainableBuilder, Chained},
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	linkbuilding::{BuildLink, LinkBuilder},
	ArcLock, Joint, KinematicInterface, Link, WeakLock,
};

impl ChainableBuilder for LinkBuilder {
	fn has_chain(&self) -> bool {
		!self.joints.is_empty()
	}
}

impl BuildLink for Chained<LinkBuilder> {
	fn build(self, _tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		unimplemented!("build should not be able to be called?")
	}

	fn start_building_chain(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		self.0.start_building_chain(tree)
	}

	fn build_chain(
		self,
		_tree: &Weak<KinematicDataTree>,
		_parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link> {
		unimplemented!("build_chain should not be able to be called?")
	}

	fn get_shape_data(&self) -> crate::link::LinkShapeData {
		unimplemented!("get_shape_data should not be able to be called?")
	}
}

/// Since Link's can end a chain, a `LinkBuilder` can always be converted to a `Chained<LinkBuilder>`
impl From<LinkBuilder> for Chained<LinkBuilder> {
	fn from(value: LinkBuilder) -> Self {
		Self(value)
	}
}

impl From<Chained<LinkBuilder>> for LinkBuilder {
	fn from(value: Chained<LinkBuilder>) -> Self {
		value.0
	}
}

impl<KI> From<KI> for Chained<LinkBuilder>
where
	KI: KinematicInterface,
{
	fn from(value: KI) -> Self {
		// FIXME: Is unwrap Ok Here?
		// FIXME: Maybe use the non-blocking read, for production?
		Self(value.get_root_link().try_read().unwrap().yank())
	}
}
