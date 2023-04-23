mod chained_jointbuilder;
mod chained_linkbuilder;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Chained<Builder: ChainableBuilder>(pub(crate) Builder);

impl<Builder> Chained<Builder>
where
	Builder: ChainableBuilder,
{
	pub fn get_builder(&self) -> &Builder {
		&self.0
	}

	/// FIXME: This is not very usefull since most JointBuilder Methods are consuming
	pub fn get_builder_mut(&mut self) -> &mut Builder {
		&mut self.0
	}

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

pub trait ChainableBuilder: Debug + PartialEq + Clone {
	/// Returns `true` if the builder has a chain/one or more childeren.
	fn has_chain(&self) -> bool;
}
