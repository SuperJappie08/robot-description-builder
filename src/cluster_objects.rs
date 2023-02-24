use std::{cell::RefCell, rc::Rc};

use crate::link::Link;

use self::kinematic_tree_data::KinematicTreeData;

mod kinematic_data_errors;
pub mod kinematic_tree;
pub(crate) mod kinematic_tree_data;

pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;
	// fn merge(&mut self, other: dyn KinematicInterface);
	fn get_root_link(&self) -> Rc<RefCell<Link>>;
	fn get_kinematic_data(&self) -> KinematicTreeData;
}
