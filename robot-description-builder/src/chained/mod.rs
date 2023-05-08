mod chained_jointbuilder;
mod chained_linkbuilder;

use std::{
	fmt::Debug,
	ops::{Deref, DerefMut},
};

use crate::identifiers::GroupIDChanger;

#[derive(Debug, PartialEq, Clone)]
pub struct Chained<Builder: ChainableBuilder>(pub(crate) Builder);

impl<Builder> Chained<Builder>
where
	Builder: ChainableBuilder,
{
	/// TODO: Maybe deprecate
	pub fn builder(&self) -> &Builder {
		&self.0
	}

	/// FIXME: This is not very usefull since most JointBuilder Methods are consuming
	/// TODO: Maybe deprecate
	pub fn builder_mut(&mut self) -> &mut Builder {
		&mut self.0
	}

	/// TODO: Maybe deprecate since Deref and DerefMut
	/// Allows the internal `Builder` and it's chain to be changed by a closure.
	///
	/// If in this process the `Builder` has lost it's chain, for example due to overwriting.
	/// An error of type `Builder` is retured so the updated `Builder` can still be used.
	pub fn modify_builder<F>(self, mut f: F) -> Result<Self, Builder>
	where
		F: FnMut(Builder) -> Builder,
	{
		let builder = f(self.0);

		// When we are tricked into loosing our chain
		match builder.has_chain() {
			// The modified `Builder` still has a chain, therefor we can continue to assume our `Builder` has a chain and make a new `Chained<Builder>`.
			true => Ok(Self(builder)),
			// The modified `Builder` has does not have a chain, therefor we can not build chains with it.
			false => Err(builder),
		}
	}
}

// To allow for calling functions on the internal builders
// TODO: Figure out if builderfunctions do not free the internal builder from it `Chained<Builder>` Identifier
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

	/// For my own conncept of `DerefMut`
	#[test]
	fn deref_mut_test() {
		let leg_tree = Link::builder("leg_[[L01]]_l1").build_tree();
		leg_tree
			.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::builder("leg_[[L01]]_l2"),
				SmartJointBuilder::new_fixed("leg_[[L01]]_j1"),
			)
			.unwrap();

		let tree = Link::builder("root").build_tree();
		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(leg_tree, SmartJointBuilder::new_fixed("leg_[[L01]]_j0"))
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
