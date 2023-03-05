use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

use crate::{
	cluster_objects::{
		kinematic_data_errors::AddTransmissionError, kinematic_tree_data::KinematicTreeData,
	},
	joint::Joint,
	link::Link,
	material::Material,
	Transmission,
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
	/// FIXME: REPAIR DOC TEST
	/// ```ignore 
	/// # use rdf_builder_rs::{KinematicInterface, Link, JointType};
	/// let tree = Link::new("the root link".to_owned());
	///
	/// /// This is equivalent to `get_root_link` in this case, since this is a new tree/Link.
	/// tree.get_newest_link().borrow_mut().try_attach_child(
	///     Link::new("his one and only child".to_owned()).into(),
	///     "just a joint".to_owned(),
	///     JointType::Fixed
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_root_link().borrow().get_name(), "the root link")
	/// ```
	fn get_root_link(&self) -> Arc<RwLock<Link>>;

	/// Returns the newest link of the Kinematic Tree
	///
	/// # Example
	/// FIXME: REPAIR EXAMPLE
	/// ```ignore
	/// # use rdf_builder_rs::{KinematicInterface, Link, JointType};
	/// let tree = Link::new("the root link".to_owned());
	///
	/// assert_eq!(tree.get_newest_link().borrow().get_name(), "the root link");
	///
	/// tree.get_newest_link().borrow_mut().try_attach_child(
	///     Link::new("his one and only child".to_owned()).into(),
	///     "just a joint".to_owned(), JointType::Fixed
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().borrow().get_name(), "his one and only child");
	///
	/// let long_sub_tree = Link::new("the other child". to_owned());
	///
	/// long_sub_tree.get_newest_link().borrow_mut().try_attach_child(
	///     Link::new("the latest child".to_owned()).into(),
	///     "second joint".to_owned(), JointType::Fixed
	/// ).unwrap();
	///
	/// tree.get_root_link().borrow_mut().try_attach_child(long_sub_tree.into(),
	///     "third joint".to_owned(), JointType::Fixed
	/// ).unwrap();
	///
	/// assert_eq!(tree.get_newest_link().borrow().get_name(), "the latest child");
	/// ```
	fn get_newest_link(&self) -> Arc<RwLock<Link>>;

	#[deprecated]
	/// Maybe deprecate?
	fn get_kinematic_data(&self) -> Arc<RwLock<KinematicTreeData>>;

	// These do not have to be mutable
	fn get_links(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>>;
	fn get_joints(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>;
	fn get_materials(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Material>>>>>;
	fn get_transmissions(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Transmission>>>>>;

	fn get_link(&self, name: &str) -> Option<Arc<RwLock<Link>>>;
	fn get_joint(&self, name: &str) -> Option<Arc<RwLock<Joint>>>;
	fn get_material(&self, name: &str) -> Option<Arc<RwLock<Material>>>;
	fn get_transmission(&self, name: &str) -> Option<Arc<RwLock<Transmission>>>;

	// TODO: ADD try_add_material()
	/// TODO: NOT FINAL
	/// TODO: Maybe remove rcrefcell from transmission parameter
	fn try_add_transmission(
		&self,
		transmission: Arc<RwLock<Transmission>>,
	) -> Result<(), AddTransmissionError>;

	// TODO: Expand
}
