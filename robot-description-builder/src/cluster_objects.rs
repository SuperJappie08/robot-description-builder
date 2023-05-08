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
	ArcLock, Chained, WeakLock,
};

pub mod kinematic_data_errors;
pub(crate) mod kinematic_data_tree;
mod kinematic_tree;
mod robot;

pub use kinematic_tree::KinematicTree;
pub use robot::Robot;

type PoisonWriteIndexError<'a, K, V> = PoisonError<RwLockWriteGuard<'a, HashMap<K, V>>>;

/// TODO: MAYBE RENAME SOME FUNCTIONS to conform to convention
pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;

	/// Returns the root link of the Kinematic Tree
	///
	/// # Example
	/// ```
	/// # use robot_description_builder::{KinematicInterface, Link, JointBuilder, JointType, linkbuilding::LinkBuilder};
	/// let tree = Link::builder("the root link").build_tree();
	///
	/// /// This is equivalent to `get_root_link` in this case, since this is a new tree/Link.
	/// tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     LinkBuilder::new("his one and only child"),
	///     JointBuilder::new("just a joint", JointType::Fixed)
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
	///     LinkBuilder::new("his one and only child"),
	///     JointBuilder::new("just a joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "his one and only child");
	///
	/// let long_sub_tree = LinkBuilder::new("the other child").build_tree();
	///
	/// long_sub_tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     Link::builder("the latest child"),
	///     JointBuilder::new("second joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// tree.get_root_link().try_write().unwrap().try_attach_child(long_sub_tree,
	///     JointBuilder::new("third joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "the latest child");
	/// ```
	fn get_newest_link(&self) -> ArcLock<Link>;

	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>>;
	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>>;
	/// FIXME: This not Ok end-user should not interact wiht MaterialData
	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<MaterialData>>>;
	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>>;

	fn get_link(&self, name: &str) -> Option<ArcLock<Link>>;
	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>>;
	fn get_material(&self, name: &str) -> Option<Material>;
	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>>;

	/// TODO: NOT FINAL
	/// TODO: Maybe remove rcrefcell from transmission parameter
	fn try_add_transmission(
		&self,
		transmission: TransmissionBuilder<WithJoints, WithActuator>,
	) -> Result<(), AddTransmissionError>;

	// TODO: Expand

	/// Cleans up orphaned/broken `Link` entries from the `links` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_links(&self) -> Result<(), PoisonWriteIndexError<String, WeakLock<Link>>>;

	/// Cleans up orphaned/broken `Joint` entries from the `joints` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_joints(&self) -> Result<(), PoisonWriteIndexError<String, WeakLock<Joint>>>;

	/// Cleans up orphaned/unused `Material` entries from `material_index` HashMap
	fn purge_materials(&self) -> Result<(), PoisonWriteIndexError<String, ArcLock<MaterialData>>>;

	/// Cleans up orphaned/broken `Transmission` entries from the `transmissions` HashMap
	fn purge_transmissions(
		&self,
	) -> Result<(), PoisonWriteIndexError<String, ArcLock<Transmission>>>;

	/// TODO: DOCS
	///
	/// NOTE: after yanking the joints parent link is the `newest_link`
	fn yank_link(&self, name: &str) -> Option<Chained<LinkBuilder>> {
		let builder = self
			.get_link(name)
			.map(|link| link.read().unwrap().yank()) // FIXME: Is unwrap ok here?
			.map(Chained);
		self.purge_joints().unwrap(); // FIXME: Is unwrap ok here?
		self.purge_links().unwrap(); // FIXME: Is unwrap ok here?
		builder
	}

	fn yank_joint(&self, name: &str) -> Option<Chained<JointBuilder>> {
		let builder = self
			.get_joint(name)
			.map(|joint| joint.read().unwrap().yank()) // FIXME: Is unwrap ok here?
			.map(Chained);
		self.purge_joints().unwrap(); // FIXME: Is unwrap ok here?
		self.purge_links().unwrap(); // FIXME: Is unwrap ok here?
		builder
	}

	// TODO: MAYBE yank_root or a rebuild?
}
