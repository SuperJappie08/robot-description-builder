use std::collections::HashMap;

use crate::{
	cluster_objects::{
		kinematic_data_errors::AddTransmissionError, kinematic_tree_data::KinematicTreeData,
	},
	joint::Joint,
	link::Link,
	material::Material,
	transmission::Transmission,
	ArcLock, WeakLock,
};

pub mod kinematic_data_errors;
mod kinematic_tree;
pub(crate) mod kinematic_tree_data;
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

	#[deprecated]
	/// Maybe deprecate?
	fn get_kinematic_data(&self) -> ArcLock<KinematicTreeData>;

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
}
