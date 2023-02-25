use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{joint::Joint, link::Link};

use self::kinematic_tree_data::KinematicTreeData;

pub mod kinematic_data_errors;
pub mod kinematic_tree;
pub(crate) mod kinematic_tree_data;
pub mod robot;

pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;
	// fn merge(&mut self, other: dyn KinematicInterface);
	fn get_root_link(&self) -> Rc<RefCell<Link>>;
	/// TODO: Maybe make this return a Ref instead of a Rc (WAS WEAK)
	fn get_newest_link(&self) -> Rc<RefCell<Link>>;

	//#[deprecated]
	/// Maybe deprecate?
	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>>;

	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>>;
	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>>;

	// TODO: Expand
}
