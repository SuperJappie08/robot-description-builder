use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData, joint::Joint, link::Link,
	material::Material, Transmission,
};

use super::{kinematic_data_errors::AddTransmissionError, KinematicInterface};

#[derive(Debug)]
pub struct KinematicTree(Rc<RefCell<KinematicTreeData>>);

impl KinematicTree {
	pub fn new(data: Rc<RefCell<KinematicTreeData>>) -> KinematicTree {
		KinematicTree(data)
	}
}

impl KinematicInterface for KinematicTree {
	fn get_root_link(&self) -> Rc<RefCell<Link>> {
		Rc::clone(&self.0.borrow().root_link)
	}

	fn get_newest_link(&self) -> Rc<RefCell<Link>> {
		self.0.borrow().newest_link.upgrade().unwrap()
	}

	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>> {
		Rc::clone(&self.0)
	}

	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>> {
		Rc::clone(&self.0.borrow().links)
	}

	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>> {
		Rc::clone(&self.0.borrow().joints)
	}

	fn get_materials(&self) -> Rc<RefCell<HashMap<String, Rc<RefCell<Material>>>>> {
		Rc::clone(&self.0.borrow().material_index)
	}

	fn get_transmissions(&self) -> Rc<RefCell<HashMap<String, Rc<RefCell<Transmission>>>>> {
		Rc::clone(&self.0.borrow().transmissions)
	}

	fn get_link(&self, name: &str) -> Option<Rc<RefCell<Link>>> {
		self.0
			.borrow()
			.links
			.borrow()
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<Rc<RefCell<Joint>>> {
		self.0
			.borrow()
			.joints
			.borrow()
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Rc<RefCell<Material>>> {
		self.0
			.borrow()
			.material_index
			.borrow()
			.get(name)
			.and_then(|material_rc| Some(Rc::clone(material_rc)))
	}

	fn get_transmission(&self, name: &str) -> Option<Rc<RefCell<Transmission>>> {
		self.0
			.borrow()
			.transmissions
			.borrow()
			.get(name)
			.and_then(|transmission_rc| Some(Rc::clone(transmission_rc)))
	}

	fn try_add_transmission(
		&self,
		transmission: Rc<RefCell<Transmission>>,
	) -> Result<(), AddTransmissionError> {
		self.0.borrow_mut().try_add_transmission(transmission)
	}
}

impl Clone for KinematicTree {
	fn clone(&self) -> Self {
		// TODO: Maybe update identifier?
		let tree = Link::new(self.get_root_link().borrow().name.clone());

		let mut change = true;
		while change {
			let keys: Vec<String> = tree
				.get_links()
				.borrow()
				.keys()
				.map(|key| key.clone())
				.collect();
			let key_count = keys.len();

			for key in keys {
				let binding = tree.get_link(&key).unwrap();
				let mut current_link = binding.borrow_mut();
				if current_link.get_joints().len()
					== self.get_link(&key).unwrap().borrow().get_joints().len()
				{
					// TODO: Clone other internal data
					continue;
				} else {
					for joint in self
						.get_link(&key)
						.unwrap()
						.borrow()
						.get_joints()
						.iter()
						.map(|joint| joint.borrow())
					{
						current_link
							.try_attach_child(
								Link::new(joint.get_child_link().borrow().name.clone()).into(),
								joint.name.clone(),
								joint.joint_type.clone(),
							)
							.unwrap()
					}
				}
			}

			change = key_count != tree.get_links().borrow().len();
		}
		tree
	}
}

impl Into<Box<dyn KinematicInterface>> for KinematicTree {
	fn into(self) -> Box<dyn KinematicInterface> {
		Box::new(self)
	}
}

#[cfg(test)]
mod tests {
	use std::rc::{Rc, Weak};

	use crate::{
		link::{Link, LinkParent},
		JointType, KinematicInterface,
	};

	#[test]
	fn clone_single() {
		let tree = Link::new("example-link".into());
		let cloned_tree = tree.clone();

		println!("tree->data        | ptr: {:#?}", Rc::as_ptr(&tree.0));
		println!(
			"cloned_tree->data | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.0)
		);
		assert!(!Rc::ptr_eq(&tree.0, &cloned_tree.0));

		println!(
			"tree->..->root_link        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_root_link())
		);
		println!(
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Rc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		println!(
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().borrow().name.as_ptr()
		);
		println!(
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree.get_root_link().borrow().name.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().borrow().get_name(),
			&cloned_tree.get_root_link().borrow().get_name()
		);

		println!(
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().borrow().tree)
		);
		println!(
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().borrow().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().borrow().tree,
			&cloned_tree.get_root_link().borrow().tree
		));

		println!(
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(match &tree.get_root_link().borrow().get_parent().unwrap() {
				LinkParent::KinematicTree(weak_tree) => weak_tree,
				LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
			})
		);
		println!(
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree.get_root_link().borrow().get_parent().unwrap() {
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().borrow().get_parent(),
			&cloned_tree.get_root_link().borrow().get_parent()
		);

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().borrow().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().borrow().get_joints()
		);
		assert_eq!(
			tree.get_root_link().borrow().get_joints().len(),
			cloned_tree.get_root_link().borrow().get_joints().len()
		);

		println!(
			"tree->..->links        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_links())
		);
		println!(
			"cloned_tree->..->links | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_links())
		);
		assert!(!Rc::ptr_eq(&tree.get_links(), &cloned_tree.get_links()));
		assert_eq!(
			tree.get_links().borrow().len(),
			cloned_tree.get_links().borrow().len()
		);

		println!(
			"tree->..->links[\"example-link\"]        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_links().borrow().get("example-link").unwrap())
		);
		println!(
			"cloned_tree->..->links[\"example-link\"] | ptr: {:#?}\n",
			Weak::as_ptr(
				&cloned_tree
					.get_links()
					.borrow()
					.get("example-link")
					.unwrap()
			)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_links().borrow().get("example-link").unwrap(),
			&cloned_tree
				.get_links()
				.borrow()
				.get("example-link")
				.unwrap()
		));

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().borrow().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().borrow().get_joints()
		);
		assert_eq!(
			tree.get_root_link().borrow().get_joints().len(),
			cloned_tree.get_root_link().borrow().get_joints().len()
		);

		println!(
			"tree->..->joints        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_joints())
		);
		println!(
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Rc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().borrow().len(),
			cloned_tree.get_joints().borrow().len()
		);

		println!(
			"tree->..->newest_link        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_newest_link())
		);
		println!(
			"cloned_tree->..->newest_link | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_newest_link())
		);
		assert!(!Rc::ptr_eq(
			&tree.get_newest_link(),
			&cloned_tree.get_newest_link()
		));
	}

	#[test]
	fn clone_multi() {
		let tree = Link::new("example-link".into());
		let other_tree = Link::new("other-link".into());
		other_tree
			.get_newest_link()
			.borrow_mut()
			.try_attach_child(
				Link::new("other-child".into()).into(),
				"other-child-joint".into(),
				JointType::Fixed,
			)
			.unwrap();

		tree.get_root_link()
			.borrow_mut()
			.try_attach_child(other_tree.into(), "other-joint".into(), JointType::Fixed)
			.unwrap();

		tree.get_root_link()
			.borrow_mut()
			.try_attach_child(
				Link::new("3".into()).into(),
				"three".into(),
				JointType::Fixed,
			)
			.unwrap();

		let cloned_tree = tree.clone();

		println!("tree->data        | ptr: {:#?}", Rc::as_ptr(&tree.0));
		println!(
			"cloned_tree->data | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.0)
		);
		assert!(!Rc::ptr_eq(&tree.0, &cloned_tree.0));

		println!(
			"tree->..->root_link        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_root_link())
		);
		println!(
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Rc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		println!(
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().borrow().name.as_ptr()
		);
		println!(
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree.get_root_link().borrow().name.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().borrow().get_name(),
			&cloned_tree.get_root_link().borrow().get_name()
		);

		println!(
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().borrow().tree)
		);
		println!(
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().borrow().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().borrow().tree,
			&cloned_tree.get_root_link().borrow().tree
		));

		println!(
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(match &tree.get_root_link().borrow().get_parent().unwrap() {
				LinkParent::KinematicTree(weak_tree) => weak_tree,
				LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
			})
		);
		println!(
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree.get_root_link().borrow().get_parent().unwrap() {
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().borrow().get_parent(),
			&cloned_tree.get_root_link().borrow().get_parent()
		);

		println!(
			"tree->..->root_link->child_joints:        {:?}",
			&tree
				.get_root_link()
				.borrow()
				.get_joints()
				.iter()
				.map(|joint| joint.borrow().name.clone())
				.collect::<Vec<String>>()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:?}\n",
			&cloned_tree
				.get_root_link()
				.borrow()
				.get_joints()
				.iter()
				.map(|joint| joint.borrow().name.clone())
				.collect::<Vec<String>>()
		);
		assert_eq!(
			tree.get_root_link().borrow().get_joints().len(),
			cloned_tree.get_root_link().borrow().get_joints().len()
		);

		println!(
			"tree->..->links        | ptr: {:#?} | keys: {:?}",
			Rc::as_ptr(&tree.get_links()),
			&tree
				.get_links()
				.borrow()
				.keys()
				.map(|key| key.clone())
				.collect::<Vec<String>>()
		);
		println!(
			"cloned_tree->..->links | ptr: {:#?} | keys: {:?}\n",
			Rc::as_ptr(&cloned_tree.get_links()),
			&cloned_tree
				.get_links()
				.borrow()
				.keys()
				.map(|key| key.clone())
				.collect::<Vec<String>>()
		);
		assert!(!Rc::ptr_eq(&tree.get_links(), &cloned_tree.get_links()));
		assert_eq!(
			tree.get_links().borrow().len(),
			cloned_tree.get_links().borrow().len()
		);

		println!(
			"tree->..->links[\"example-link\"]        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_links().borrow().get("example-link").unwrap())
		);
		println!(
			"cloned_tree->..->links[\"example-link\"] | ptr: {:#?}\n",
			Weak::as_ptr(
				&cloned_tree
					.get_links()
					.borrow()
					.get("example-link")
					.unwrap()
			)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_links().borrow().get("example-link").unwrap(),
			&cloned_tree
				.get_links()
				.borrow()
				.get("example-link")
				.unwrap()
		));

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().borrow().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().borrow().get_joints()
		);
		assert_eq!(
			tree.get_root_link().borrow().get_joints().len(),
			cloned_tree.get_root_link().borrow().get_joints().len()
		);

		println!(
			"tree->..->joints        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_joints())
		);
		println!(
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Rc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().borrow().len(),
			cloned_tree.get_joints().borrow().len()
		);

		println!(
			"tree->..->newest_link        | ptr: {:#?}",
			Rc::as_ptr(&tree.get_newest_link())
		);
		println!(
			"cloned_tree->..->newest_link | ptr: {:#?}\n",
			Rc::as_ptr(&cloned_tree.get_newest_link())
		);
		assert!(!Rc::ptr_eq(
			&tree.get_newest_link(),
			&cloned_tree.get_newest_link()
		));
	}
}
