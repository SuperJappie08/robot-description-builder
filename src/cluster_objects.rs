use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{joint::Joint, link::Link, material::Material};

use self::kinematic_tree_data::KinematicTreeData;

pub mod kinematic_data_errors;
mod kinematic_tree;
pub(crate) mod kinematic_tree_data;
mod robot;

pub use kinematic_tree::KinematicTree;
pub use robot::Robot;

pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;
	// fn merge(&mut self, other: dyn KinematicInterface);
	fn get_root_link(&self) -> Rc<RefCell<Link>>;
	/// TODO: Maybe make this return a Ref instead of a Rc (WAS WEAK) -> UPDATE: You can't, it is not allowed by the scoping rules
	fn get_newest_link(&self) -> Rc<RefCell<Link>>;

	#[deprecated]
	/// Maybe deprecate?
	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>>;

	// These do not have to be mutable
	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>>;
	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>>;
	fn get_material_index(&self) -> Rc<RefCell<HashMap<String, Rc<RefCell<Material>>>>>;

	fn get_link(&self, name: &str) -> Option<Rc<RefCell<Link>>>;
	fn get_joint(&self, name: &str) -> Option<Rc<RefCell<Joint>>>;
	fn get_material(&self, name: &str) -> Option<Rc<RefCell<Material>>>;

	// TODO: Expand
}
