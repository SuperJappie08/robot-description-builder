mod chained_jointbuilder;
mod chained_linkbuilder;

use std::{
	fmt::Debug,
	ops::{Deref, DerefMut},
};

use crate::identifiers::GroupIDChanger;

/// A Wrapper to indicate if the current builder is a chain.
///
/// TODO: EXPAND
#[derive(Debug, PartialEq, Clone)]
#[repr(transparent)]
pub struct Chained<Builder: ChainableBuilder>(pub(crate) Builder);

// To allow for calling functions on the internal builders
// TODO: Figure out if builderfunctions do not free the internal builder from it `Chained<Builder>` Identifier
//
// UPDATE: We do not escape however, the functions that consume can not be called either, see test chained::chained_joint_builder::test::chained escaping
// TODO: Solution?: Add inplace methods???
//
// FIXME: but deref should theoretically only be implemented for smartpointers which do not add methods
// https://rust-lang.github.io/api-guidelines/predictability.html?highlight=deref#only-smart-pointers-implement-deref-and-derefmut-c-deref
// https://rust-lang.github.io/api-guidelines/predictability.html?highlight=deref#smart-pointers-do-not-add-inherent-methods-c-smart-ptr
impl<Builder> Deref for Chained<Builder>
where
	Builder: ChainableBuilder,
{
	type Target = Builder;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

// To allow for calling functions on the internal builders
// TODO: See above
impl<Builder> DerefMut for Chained<Builder>
where
	Builder: ChainableBuilder,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[cfg(feature = "wrapper")]
impl<Builder> Chained<Builder>
where
	Builder: ChainableBuilder,
{
	/// Create a Chained Builder, needs to be a chain.
	pub unsafe fn new(builder: Builder) -> Self {
		Chained(builder)
	}
}

pub trait ChainableBuilder: Debug + PartialEq + Clone + GroupIDChanger {
	/// Returns `true` if the builder has a chain/one or more childeren.
	fn has_chain(&self) -> bool;
}

#[cfg(test)]
mod tests {
	use super::Chained;
	use crate::{
		joint::{JointBuilder, SmartJointBuilder},
		link::{builder::LinkBuilder, Link},
		prelude::*,
	};
	use test_log::test;

	// For my own concept of `DerefMut`
	#[test]
	fn deref_mut_test() {
		let leg_tree = Link::builder("leg_[[L01]]_l1").build_tree();
		leg_tree
			.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new_fixed("leg_[[L01]]_j1"),
				Link::builder("leg_[[L01]]_l2"),
			)
			.unwrap();

		let tree = Link::builder("root").build_tree();
		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(SmartJointBuilder::new_fixed("leg_[[L01]]_j0"), leg_tree)
			.unwrap();

		let builder_chain = tree.yank_joint("leg_[[L01]]_j0").unwrap();

		assert_eq!(
			builder_chain,
			Chained(JointBuilder {
				name: "leg_[[L01]]_j0".into(),
				child: Some(LinkBuilder {
					name: "leg_[[L01]]_l1".into(),
					joints: vec![JointBuilder {
						name: "leg_[[L01]]_j1".into(),
						child: Some(LinkBuilder {
							name: "leg_[[L01]]_l2".into(),
							..Default::default()
						}),
						..Default::default()
					}],
					..Default::default()
				}),
				..Default::default()
			})
		);

		let mut mirrored_chain = builder_chain
			.clone()
			.mirror(crate::transform::MirrorAxis::X);
		// TODO: Add chainable version?
		mirrored_chain.change_group_id("R01").unwrap();

		let tree = Link::builder("root").build_tree();
		tree.get_root_link()
			.try_write()
			.unwrap()
			.attach_joint_chain(builder_chain)
			.unwrap();
		tree.get_root_link()
			.try_write()
			.unwrap()
			.attach_joint_chain(mirrored_chain)
			.unwrap();

		assert_eq!(
			tree.yank_link("root").unwrap(),
			Chained(LinkBuilder {
				name: "root".into(),
				joints: vec![
					JointBuilder {
						name: "leg_[[L01]]_j0".into(),
						child: Some(LinkBuilder {
							name: "leg_[[L01]]_l1".into(),
							joints: vec![JointBuilder {
								name: "leg_[[L01]]_j1".into(),
								child: Some(LinkBuilder {
									name: "leg_[[L01]]_l2".into(),
									..Default::default()
								}),
								..Default::default()
							}],
							..Default::default()
						}),
						..Default::default()
					},
					JointBuilder {
						name: "leg_[[R01]]_j0".into(),
						child: Some(LinkBuilder {
							name: "leg_[[R01]]_l1".into(),
							joints: vec![JointBuilder {
								name: "leg_[[R01]]_j1".into(),
								child: Some(LinkBuilder {
									name: "leg_[[R01]]_l2".into(),
									..Default::default()
								}),
								..Default::default()
							}],
							..Default::default()
						}),
						..Default::default()
					}
				],
				..Default::default()
			})
		)
	}
}
