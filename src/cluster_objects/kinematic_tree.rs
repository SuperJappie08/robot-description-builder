use std::{cell::RefCell, rc::Rc};

use crate::cluster_objects::kinematic_tree_data::KinematicTreeData;

#[derive(Debug)]
pub struct KinematicTree(Rc<RefCell<KinematicTreeData>>);

impl KinematicTree {
	pub fn new(data: Rc<RefCell<KinematicTreeData>>) -> KinematicTree {
		KinematicTree(data)
	}
}
