use std::{
	collections::HashMap,
	sync::{PoisonError, RwLockWriteGuard},
};

use crate::{
	cluster_objects::kinematic_data_errors::AddTransmissionError, joint::Joint, link::Link,
	linkbuilding::LinkBuilder, material::Material, transmission::Transmission, ArcLock,
	JointBuilder, WeakLock,
};

pub mod kinematic_data_errors;
pub(crate) mod kinematic_data_tree;
mod kinematic_tree;
mod robot;

pub use kinematic_tree::KinematicTree;
pub use robot::Robot;

pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;

	/// Returns the root link of the Kinematic Tree
	///
	/// # Example
	/// ```
	/// # use rdf_builder_rs::{KinematicInterface, Link, JointBuilder, JointType};
	/// let tree = Link::new("the root link");
	///
	/// /// This is equivalent to `get_root_link` in this case, since this is a new tree/Link.
	/// tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     Link::new("his one and only child").into(),
	///     JointBuilder::new("just a joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_root_link().try_read().unwrap().get_name(), "the root link")
	/// ```
	fn get_root_link(&self) -> ArcLock<Link>;

	/// Returns the newest link of the Kinematic Tree
	///
	/// # Example
	/// ```
	/// # use rdf_builder_rs::{KinematicInterface, Link, JointBuilder, JointType};
	/// let tree = Link::new("the root link");
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().get_name(), "the root link");
	///
	/// tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     Link::new("his one and only child").into(),
	///     JointBuilder::new("just a joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().get_name(), "his one and only child");
	///
	/// let long_sub_tree = Link::new("the other child");
	///
	/// long_sub_tree.get_newest_link().try_write().unwrap().try_attach_child(
	///     Link::new("the latest child").into(),
	///     JointBuilder::new("second joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// tree.get_root_link().try_write().unwrap().try_attach_child(long_sub_tree.into(),
	///     JointBuilder::new("third joint", JointType::Fixed)
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().try_read().unwrap().get_name(), "the latest child");
	/// ```
	fn get_newest_link(&self) -> ArcLock<Link>;

	// These do not have to be mutable
	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>>;
	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>>;
	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<Material>>>;
	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>>;

	fn get_link(&self, name: &str) -> Option<ArcLock<Link>>;
	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>>;
	fn get_material(&self, name: &str) -> Option<ArcLock<Material>>;
	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>>;

	// TODO: ADD try_add_material()
	/// TODO: NOT FINAL
	/// TODO: Maybe remove rcrefcell from transmission parameter
	fn try_add_transmission(
		&self,
		transmission: ArcLock<Transmission>,
	) -> Result<(), AddTransmissionError>;

	// TODO: Expand

	/// Cleans up orphaned/broken `Link` entries from the `links` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_links(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, WeakLock<Link>>>>>;

	/// Cleans up orphaned/broken `Joint` entries from the `joints` HashMap.
	///
	/// This mostly happens automatically, but is exposed for use in other methods.
	///
	/// TODO: DOCTEST/EXAMPLE
	fn purge_joints(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, WeakLock<Joint>>>>>;

	/// Cleans up orphaned/unused `Material` entries from `material_index` HashMap
	fn purge_materials(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<Material>>>>>;

	/// Cleans up orphaned/broken `Transmission` entries from the `transmissions` HashMap
	fn purge_transmissions(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<Transmission>>>>>;

	fn yank_link(&self, name: &str) -> Option<LinkBuilder> {
		let builder = self
			.get_link(name)
			.map(|link| link.try_read().unwrap().yank()); // FIXME: Is unwrap ok here?
		self.purge_joints().unwrap(); // FIXME: Is unwrap ok here?
		self.purge_links().unwrap(); // FIXME: Is unwrap ok here?
		builder
	}

	fn yank_joint(&self, name: &str) -> Option<JointBuilder> {
		let builder = self
			.get_joint(name)
			.map(|joint| joint.try_read().unwrap().yank()); // FIXME: Is unwrap ok here?
		self.purge_joints().unwrap(); // FIXME: Is unwrap ok here?
		self.purge_links().unwrap(); // FIXME: Is unwrap ok here?
		builder
	}
}
