use std::{
	collections::HashMap,
	sync::{PoisonError, RwLockWriteGuard},
};

use crate::{
	cluster_objects::kinematic_data_errors::AddTransmissionError,
	joint::{Joint, JointBuilder},
	link::{builder::LinkBuilder, Link},
	material::{data::MaterialData, Material},
	transmission::{
		transmission_builder_state::{WithActuator, WithJoints},
		Transmission, TransmissionBuilder,
	},
	utils::{ArcLock, ArcRW, WeakLock},
	yank_errors::{YankJointError, YankLinkError},
	Chained,
};

pub mod kinematic_data_errors;
pub(crate) mod kinematic_data_tree;
mod kinematic_tree;
mod robot;

pub use kinematic_tree::KinematicTree;
pub use robot::Robot;

type PoisonWriteIndexError<'a, K, V> = PoisonError<RwLockWriteGuard<'a, HashMap<K, V>>>;

/// A trait which allows for the existance of multiple different Kinematic Structures, such as `Robot` and `KinematicTree`.
pub trait KinematicInterface: Sized {
	/// Returns the root link of the Kinematic Tree
	///
	/// # Example
	/// ```
	/// # use robot_description_builder::{KinematicInterface, Link, JointBuilder, JointType, linkbuilding::LinkBuilder};
	/// let tree = Link::builder("the root link").build_tree();
	///
	/// /// This is equivalent to `get_root_link` in this case, since this is a new tree/Link.
	/// tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     JointBuilder::new("just a joint", JointType::Fixed),
	///     LinkBuilder::new("his one and only child")
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_root_link().try_read().unwrap().name(), "the root link")
	/// ```
	fn get_root_link(&self) -> ArcLock<Link>;

	/// Returns the newest link of the Kinematic Tree
	///
	/// # Example
	/// ```
	/// # use robot_description_builder::{KinematicInterface, Link, JointBuilder, JointType, linkbuilding::LinkBuilder};
	/// let tree = Link::builder("the root link").build_tree();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "the root link");
	///
	/// tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     JointBuilder::new("just a joint", JointType::Fixed),
	///     LinkBuilder::new("his one and only child")
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "his one and only child");
	///
	/// let long_sub_tree = LinkBuilder::new("the other child").build_tree();
	///
	/// long_sub_tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     JointBuilder::new("second joint", JointType::Fixed),
	///     Link::builder("the latest child")
	/// ).unwrap();
	///
	/// tree.get_root_link().try_write().unwrap().try_attach_child(
	///     JointBuilder::new("third joint", JointType::Fixed),
	///     long_sub_tree
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "the latest child");
	/// ```
	fn get_newest_link(&self) -> ArcLock<Link>;

	/// Retrieves the `Link`-index of the Kinematic structure.
	// TODO: EXPAND?
	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>>;
	/// Retrieves the `Joint`-index of the Kinematic structure.
	// TODO: EXPAND?
	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>>;
	// FIXME: This not Ok end-user should not interact wiht MaterialData
	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<MaterialData>>>;
	#[doc(hidden)]
	// FIXME: Hidden until implemented
	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>>;

	// TODO: EXAMPLE
	/// Get the `Link` with the specified `name`.
	///
	/// If the [`Link`] does not exist `None` is returned.
	fn get_link(&self, name: &str) -> Option<ArcLock<Link>>;
	// TODO: EXAMPLE
	/// Get the `Joint` with the specified `name`.
	///
	/// If the [`Joint`] does not exist `None` is returned.
	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>>;
	// TODO: EXAMPLE
	/// Get the `Material` with the specified `name`.
	///
	/// If the [`Material`] does not exist `None` is returned.
	fn get_material(&self, name: &str) -> Option<Material>;
	#[doc(hidden)]
	// FIXME: Hidden until implemented transmissions
	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>>;

	#[doc(hidden)]
	// FIXME: Hidden until implemented
	// TODO: NOT FINAL
	fn try_add_transmission(
		&self,
		transmission: TransmissionBuilder<WithJoints, WithActuator>,
	) -> Result<(), AddTransmissionError>;

	// TODO: Maybe depricate
	/// Cleans up orphaned/broken `Link` entries from the `links` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_links(&self);

	// TODO: Maybe depricate
	/// Cleans up orphaned/broken `Joint` entries from the `joints` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_joints(&self);

	/// Cleans up orphaned/unused `Material` entries from `material_index` HashMap
	fn purge_materials(&self) -> Result<(), PoisonWriteIndexError<String, ArcLock<MaterialData>>>;

	#[doc(hidden)]
	// FIXME: Hidden until implemented
	/// Cleans up orphaned/broken `Transmission` entries from the `transmissions` HashMap
	fn purge_transmissions(
		&self,
	) -> Result<(), PoisonWriteIndexError<String, ArcLock<Transmission>>>;

	// TODO: Maybe Error is removable
	// NOTE: after yanking the joints parent link is the `newest_link`
	fn yank_link(&self, name: &str) -> Option<Chained<LinkBuilder>> {
		// Result<Option<Chained<LinkBuilder>>, YankLinkError> {
		// Maybe the option should be on the outside.
		let builder = self
			.get_link(name)
			// FIXME: UNWRAP NOT OK
			.map(|link| -> Result<_, YankLinkError> { Ok(Chained(link.mread().unwrap().yank()?)) })
			.transpose(); // TODO: Maybe don't transpose?
		self.purge_joints();
		self.purge_links();
		builder.unwrap() // FIXME: Is unwrap ok here?
	}

	/// Cosumes the `KinematicInterface` implementor and creates a `Chained<LinkBuilder>` to rebuild it.
	///
	/// This has the same result as yanking the `root_link`, with the additional effect that the current tree is consumed.
	///
	/// # Example
	///
	/// ```
	/// # use robot_description_builder::{prelude::*, Link};
	///
	/// let builder = Link::builder("root-link");
	///
	/// assert_eq!(*builder.clone().build_tree().yank_root().unwrap(), builder);
	///
	/// /// It is equivalent to yanking the "root_link"
	/// assert_eq!(builder.clone().build_tree().yank_root().unwrap(), builder.build_tree().yank_link("root-link").unwrap())
	/// ```
	fn yank_root(self) -> Result<Chained<LinkBuilder>, YankLinkError> {
		// FIXME: UNWRAP NOT OK ? Maybe it is removeable
		let builder = self.get_root_link().mread().unwrap().yank()?;
		Ok(Chained(builder))
	}

	fn yank_joint(&self, name: &str) -> Option<Chained<JointBuilder>> {
		// Option<Result<...>> could make sense since if the joint is not found it cannot be yanked
		// Or a custom error type
		let builder = self
			.get_joint(name)
			.map(|joint| -> Result<_, YankJointError> { Ok(Chained(joint.mread()?.yank()?)) })
			.transpose(); // TODO: Maybe don't transpose?
		self.purge_joints();
		self.purge_links();
		builder.unwrap() // FIXME: Is unwrap ok here? NO
	}

	// TODO: or a rebuild?
}
