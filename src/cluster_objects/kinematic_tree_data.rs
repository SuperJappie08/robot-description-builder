use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{joint::Joint, link::Link, Material, Transmission};

use super::kinematic_data_errors::{AddJointError, AddLinkError, AddMaterialError};

// pub(crate) trait KinematicTreeTrait {}

#[derive(Debug)]
pub struct KinematicTreeData {
	pub root_link: Rc<RefCell<Link>>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	// material_index: Rc<RefCell<HashMap<String, Rc<RefCell<Material>>>>>,

	// TODO: Why is this an `Rc<RefCell<_>`?
	pub(crate) links: Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>>,
	pub(crate) joints: Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>>,
	// transmissions: Rc<RefCell<HashMap<String, Rc<RefCell<Transmission>>>>>,
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
				.tree = Rc::downgrade(&tree);
		}

		tree
	}

	// pub fn try_add_material(
	// 	&mut self,
	// 	material: Rc<RefCell<Material>>,
	// ) -> Result<(), AddMaterialError> {
	// 	let name = material.try_borrow()?.name.clone();
	// 	if let Some(preexisting_material) = self.material_index.get(&name) {
	// 		if *preexisting_material.try_borrow()? != *material.try_borrow()? {
	// 			Err(AddMaterialError::MaterialConflict(name))
	// 		} else {
	// 			Ok(())
	// 		}
	// 	} else {
	// 		self.material_index.insert(name.to_string(), material);
	// 		Ok(())
	// 	}
	// }

	pub(crate) fn try_add_link(&mut self, link: Rc<RefCell<Link>>) -> Result<(), AddLinkError> {
		let name = link.try_borrow()?.name.clone();
		let other = { self.links.try_borrow()?.get(&name) }.map(Weak::clone);
		if let Some(preexisting_link) = other.and_then(|weak_link| weak_link.upgrade()) {
			if Rc::ptr_eq(&preexisting_link, &link) {
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

	pub(crate) fn try_add_joint(&mut self, joint: Rc<RefCell<Joint>>) -> Result<(), AddJointError> {
		let name = joint.try_borrow()?.name.clone();
		let other = { self.joints.borrow().get(&name) }.map(Weak::clone);
		if let Some(preexisting_joint) = other.and_then(|weak_joint| weak_joint.upgrade()) {
			if Rc::ptr_eq(&preexisting_joint, &joint) {
				Err(AddJointError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.joints
				.try_borrow_mut()?
				.insert(name, Rc::downgrade(&joint))
				.is_none());
			Ok(())
		}
	}

	/// Cleans up broken `Joint` and `Link` entries from the `links` and `joints` HashMaps.
	pub fn purge(&mut self) {
		self.joints
			.borrow_mut()
			.retain(|_, weak_joint| weak_joint.upgrade().is_some());
		self.joints.borrow_mut().shrink_to_fit();

		self.links
			.borrow_mut()
			.retain(|_, weak_link| weak_link.upgrade().is_some());
		self.links.borrow_mut().shrink_to_fit();
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
