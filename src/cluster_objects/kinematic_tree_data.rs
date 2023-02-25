use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{joint::Joint, link::Link, Material, Transmission};
// use crate::link::LinkTrait;

use super::kinematic_data_errors::{
	AddJointError, AddLinkError, TryAddMaterialError, TryMergeTreeError,
};

// pub(crate) trait KinematicTreeTrait {}

#[derive(Debug)]
pub struct KinematicTreeData {
	pub root_link: Rc<RefCell<Link>>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	// material_index: Rc<HashMap<String, Rc<RefCell<Material>>>>,

	// TODO: Might change this to be public
	pub(crate) links: Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>>,
	pub(crate) joints: Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>>,
	// transmissions: Rc<HashMap<String, Rc<RefCell<Transmission>>>>,
	pub(crate) newest_link: Weak<RefCell<Link>>,
	// is_rigid: bool // ? For gazebo
}

impl KinematicTreeData {
	pub(crate) fn new_link(root_link: Link) -> Rc<RefCell<KinematicTreeData>> {
		let root_link = Rc::new(RefCell::new(root_link));
		// let material_index = Rc::new(HashMap::new());
		let mut links = HashMap::new();
		let joints = HashMap::new();
		// let transmissions = Rc::new(HashMap::new());

		links.insert(
			root_link.try_borrow().unwrap().get_name(),
			Rc::downgrade(&root_link.clone()),
		);

		// There exist no child links, because a new link is being made.
		// let mut extra_links = Vec::new();

		// for joint in root_link.borrow().get_joints() {
		// 	if joints.contains_key(&joint.borrow().name) {
		// 		panic!("Joint name not unique: {:?}", joint)
		// 	}
		// 	joints.insert(joint.borrow().name.clone(), Rc::downgrade(&joint));

		// 	extra_links.push(Rc::clone(&joint.borrow().child_link));
		// }

		let tree = Rc::new(RefCell::new(Self {
			newest_link: Rc::downgrade(&root_link),
			root_link,
			// material_index,
			links: Rc::new(RefCell::new(links)),
			joints: Rc::new(RefCell::new(joints)),
			// transmissions,
		}));

		{
			tree.try_borrow()
				.unwrap()
				.root_link
				.try_borrow_mut()
				.unwrap()
				.set_parent(Rc::downgrade(&tree).into());

			tree.try_borrow()
				.unwrap()
				.root_link
				.try_borrow_mut()
				.unwrap()
				.tree = Rc::downgrade(&tree).into();
		}

		tree
	}

	// pub fn try_add_material(
	// 	&mut self,
	// 	material: Rc<RefCell<Material>>,
	// ) -> Result<(), TryAddMaterialError> {
	// 	let name = material.try_borrow()?.name.clone();
	// 	if let Some(preexisting_material) = self.material_index.get(&name) {
	// 		if *preexisting_material.try_borrow()? != *material.try_borrow()? {
	// 			Err(TryAddMaterialError::MaterialConflict(name))
	// 		} else {
	// 			Ok(())
	// 		}
	// 	} else {
	// 		self.material_index.insert(name.to_string(), material);
	// 		Ok(())
	// 	}
	// }

	pub fn try_add_link(&mut self, link: Rc<RefCell<Link>>) -> Result<(), AddLinkError> {
		let name = link.try_borrow()?.name.clone();
		let other = { self.links.try_borrow()?.get(&name) }
			.and_then(|weak_link| Some(Weak::clone(weak_link)));
		if let Some(preexisting_link) = other.and_then(|weak_link| weak_link.upgrade()) {
			if *preexisting_link.try_borrow()? != *link.try_borrow()? {
				Err(AddLinkError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.links
				.try_borrow_mut()?
				.insert(name, Rc::downgrade(&link))
				.is_none());
			self.newest_link = Rc::downgrade(&link);
			Ok(())
		}
	}

	pub fn try_add_joint(&mut self, joint: Rc<RefCell<Joint>>) -> Result<(), AddJointError> {
		let name = joint.try_borrow()?.name.clone();
		let other = { self.joints.borrow().get(&name) }
			.and_then(|weak_joint| Some(Weak::clone(weak_joint)));
		if let Some(preexisting_joint) = other.and_then(|weak_joint| weak_joint.upgrade()) {
			if *preexisting_joint.try_borrow()? != *joint.try_borrow()? {
				Err(AddJointError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			self.joints
				.try_borrow_mut()?
				.insert(name.to_string(), Rc::downgrade(&joint));
			Ok(())
		}
	}

	pub(crate) fn try_merge(
		&mut self,
		other_tree: KinematicTreeData,
	) -> Result<(), TryMergeTreeError> {
		todo!()
		// self.newest_link = other_tree.newest_link;
	}
}

impl PartialEq for KinematicTreeData {
	fn eq(&self, other: &Self) -> bool {
		self.root_link == other.root_link
		// && self.material_index == other.material_index
		// && self.transmissions == other.transmissions
	}
}
// impl KinematicTreeTrait for KinematicTreeData {}
