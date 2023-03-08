use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLock, RwLockWriteGuard, Weak},
};

use crate::{
	joint::JointInterface, link::Link, material::Material, ArcLock, Transmission, WeakLock,
};

use crate::cluster_objects::kinematic_data_errors::*;

// pub(crate) trait KinematicTreeTrait {}

#[derive(Debug)]
pub struct KinematicTreeData {
	pub root_link: ArcLock<Link>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	pub(crate) material_index: ArcLock<HashMap<String, ArcLock<Material>>>,
	// TODO: Why is this an `Rc<RefCell<_>`?
	pub(crate) links: ArcLock<HashMap<String, WeakLock<Link>>>,
	pub(crate) joints: ArcLock<HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
	pub(crate) transmissions: ArcLock<HashMap<String, ArcLock<Transmission>>>,
	pub(crate) newest_link: WeakLock<Link>,
	// is_rigid: bool // ? For gazebo -> TO AdvancedSimulationData [ASD]
}

impl KinematicTreeData {
	pub(crate) fn new_link(root_link: Link) -> ArcLock<KinematicTreeData> {
		let root_link = Arc::new(RwLock::new(root_link));
		let material_index = HashMap::new();
		let mut links = HashMap::new();
		let joints = HashMap::new();
		let transmissions = HashMap::new();

		links.insert(
			root_link.read().unwrap().get_name(),
			Arc::downgrade(&root_link),
		);

		// There exist no child links, because a new link is being made.

		let tree = Arc::new(RwLock::new(Self {
			newest_link: Arc::downgrade(&root_link),
			root_link,
			material_index: Arc::new(RwLock::new(material_index)),
			links: Arc::new(RwLock::new(links)),
			joints: Arc::new(RwLock::new(joints)),
			transmissions: Arc::new(RwLock::new(transmissions)),
		}));

		{
			tree.read()
				.unwrap()
				.root_link
				.write()
				.unwrap()
				.set_parent(Arc::downgrade(&tree).into());

			tree.read().unwrap().root_link.write().unwrap().tree = Arc::downgrade(&tree);
		}

		tree
	}

	pub(crate) fn try_add_material(
		&mut self,
		material: ArcLock<Material>,
	) -> Result<(), AddMaterialError> {
		let name = material.read()?.get_name();
		if name.is_none() {
			return Err(AddMaterialError::NoName);
		}
		let other_material =
			{ self.material_index.read()?.get(name.as_ref().unwrap()) }.map(Arc::clone);
		if let Some(preexisting_material) = other_material {
			if Arc::ptr_eq(&preexisting_material, &material) {
				Err(AddMaterialError::Conflict(name.unwrap()))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.material_index
				.write()?
				.insert(name.unwrap(), Arc::clone(&material))
				.is_none());
			Ok(())
		}
	}

	pub(crate) fn try_add_link(&mut self, link: ArcLock<Link>) -> Result<(), AddLinkError> {
		let name = link.read()?.get_name();
		let other = { self.links.read()?.get(&name) }.map(Weak::clone);
		if let Some(preexisting_link) = other.and_then(|weak_link| weak_link.upgrade()) {
			if Arc::ptr_eq(&preexisting_link, &link) {
				Err(AddLinkError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.links
				.write()?
				.insert(name, Arc::downgrade(&link))
				.is_none());
			self.newest_link = Arc::downgrade(&link);
			Ok(())
		}
	}

	pub(crate) fn try_add_joint(
		&mut self,
		joint: ArcLock<Box<dyn JointInterface + Sync + Send>>,
	) -> Result<(), AddJointError> {
		let name = joint.read()?.get_name();
		let other = { self.joints.read()?.get(&name) }.map(Weak::clone);
		if let Some(preexisting_joint) = other.and_then(|weak_joint| weak_joint.upgrade()) {
			if Arc::ptr_eq(&preexisting_joint, &joint) {
				Err(AddJointError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.joints
				.write()?
				.insert(name, Arc::downgrade(&joint))
				.is_none());
			Ok(())
		}
	}

	pub(crate) fn try_add_transmission(
		&mut self,
		transmission: ArcLock<Transmission>,
	) -> Result<(), AddTransmissionError> {
		let name = transmission.read()?.name.clone();
		let other_transmission = { self.transmissions.read()?.get(&name) }.map(Arc::clone);
		if let Some(preexisting_transmission) = other_transmission {
			if Arc::ptr_eq(&preexisting_transmission, &transmission) {
				Err(AddTransmissionError::Conflict(name))
			} else {
				Ok(())
			}
		} else {
			assert!(self
				.transmissions
				.write()?
				.insert(name, transmission)
				.is_none());
			Ok(())
		}
	}

	/// Cleans up broken `Joint` entries from the `joints` HashMap.
	///
	/// TODO: Maybe make pub(crate), since you can not remove a `joint` normally from outside the crate. and cleanup should be done by the crate.
	pub fn purge_joints(
		&mut self,
	) -> Result<
		(),
		PoisonError<
			RwLockWriteGuard<'_, HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
		>,
	> {
		let mut joints = self.joints.write()?; // TODO: Not so nice -> So Error
		joints.retain(|_, weak_joint| weak_joint.upgrade().is_some());
		joints.shrink_to_fit();
		Ok(())
	}

	/// Cleans up broken `Link` entries from the `links` HashMap.
	///
	/// TODO: Maybe make pub(crate), since you can not remove a `link` normally from outside the crate. and cleanup should be done by the crate.
	pub fn purge_links(
		&mut self,
	) -> Result<(), PoisonError<RwLockWriteGuard<'_, HashMap<String, WeakLock<Link>>>>> {
		let mut links = self.links.write()?;
		links.retain(|_, weak_link| weak_link.upgrade().is_some());
		links.shrink_to_fit();
		Ok(())
	}

	/// Cleans up broken `Joint` and `Link` entries from the `links` and `joints` HashMaps.
	///
	/// TODO: Rewrite DOC
	pub fn purge(&mut self) {
		self.purge_joints().unwrap(); //FIXME: UNWRAP?

		self.purge_links().unwrap(); //FIXME: UNWRAP?

		//TODO: UPDATE FOR MATERIALS
	}
}

impl PartialEq for KinematicTreeData {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.root_link, &other.root_link)
		// && self.material_index == other.material_index
		// && self.transmissions == other.transmissions
	}
}
// impl KinematicTreeTrait for KinematicTreeData {}
