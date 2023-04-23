use std::sync::Weak;

use crate::{
	chained::{ChainableBuilder, Chained},
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{BuildJointChain, Joint, JointBuilder},
	link::{Link, LinkShapeData},
	linkbuilding::LinkBuilder,
	ArcLock, WeakLock,
};

impl ChainableBuilder for JointBuilder {
	fn has_chain(&self) -> bool {
		self.child.is_some()
	}
}

impl BuildJointChain for Chained<JointBuilder> {
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_link: &WeakLock<Link>,
		parent_link_size_data: LinkShapeData,
	) -> ArcLock<Joint> {
		self.0.build_chain(tree, parent_link, parent_link_size_data)
	}
}

impl<JointB, LinkB> From<(JointB, LinkB)> for Chained<JointBuilder>
where
	JointB: Into<JointBuilder>,
	LinkB: Into<LinkBuilder>,
{
	fn from(value: (JointB, LinkB)) -> Self {
		Chained(JointBuilder {
			child: Some(value.1.into()),
			..value.0.into()
		})
	}
}
