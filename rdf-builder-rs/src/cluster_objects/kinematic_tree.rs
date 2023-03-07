use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{Joint, JointInterface},
	link::Link,
	material::Material,
	Transmission,
};

use super::{kinematic_data_errors::AddTransmissionError, KinematicInterface};

#[derive(Debug)]
pub struct KinematicTree(Arc<RwLock<KinematicTreeData>>);

impl KinematicTree {
	pub fn new(data: Arc<RwLock<KinematicTreeData>>) -> KinematicTree {
		KinematicTree(data)
	}
}

impl KinematicInterface for KinematicTree {
	fn get_root_link(&self) -> Arc<RwLock<Link>> {
		Arc::clone(&self.0.read().unwrap().root_link) // FIXME: Unwrapping might not be ok
	}

	fn get_newest_link(&self) -> Arc<RwLock<Link>> {
		self.0.read().unwrap().newest_link.upgrade().unwrap() // FIXME: Unwrapping might not be ok
	}

	fn get_kinematic_data(&self) -> Arc<RwLock<KinematicTreeData>> {
		Arc::clone(&self.0)
	}

	fn get_links(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>> {
		Arc::clone(&self.0.read().unwrap().links) // FIXME: Unwrapping might not be ok
	}

	fn get_joints(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>> {
		Arc::clone(&self.0.read().unwrap().joints) // FIXME: Unwrapping might not be ok
	}

	fn get_materials(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Material>>>>> {
		Arc::clone(&self.0.read().unwrap().material_index) // FIXME: Unwrapping might not be ok
	}

	fn get_transmissions(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Transmission>>>>> {
		Arc::clone(&self.0.read().unwrap().transmissions) // FIXME: Unwrapping might not be ok
	}

	fn get_link(&self, name: &str) -> Option<Arc<RwLock<Link>>> {
		self.0
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.links
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<Arc<RwLock<Joint>>> {
		self.0
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.joints
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Arc<RwLock<Material>>> {
		self.0
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.material_index
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn get_transmission(&self, name: &str) -> Option<Arc<RwLock<Transmission>>> {
		self.0
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.transmissions
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn try_add_transmission(
		&self,
		transmission: Arc<RwLock<Transmission>>,
	) -> Result<(), AddTransmissionError> {
		self.0
			.write()
			.unwrap() // FIXME: Unwrapping might not be ok
			.try_add_transmission(transmission)
	}
}

impl Clone for KinematicTree {
	fn clone(&self) -> Self {
		// TODO: Maybe update identifier?
		let tree = Link::new(self.get_root_link().read().unwrap().name.clone()); // FIXME: Unwrapping might not be ok

		let mut change = true;
		while change {
			let keys: Vec<String> = tree.get_links().read().unwrap().keys().cloned().collect(); // FIXME: Unwrapping might not be ok
			let key_count = keys.len();

			for key in keys {
				let binding = tree.get_link(&key).unwrap();
				let mut current_link = binding.write().unwrap(); // FIXME: Unwrapping might not be ok
				if current_link.get_joints().len()
					== self
						.get_link(&key)
						.unwrap()
						.read()
						.unwrap()
						.get_joints()
						.len()
				// FIXME: Unwrapping might not be ok
				{
					// FIXME: Clone other internal data
					continue;
				} else {
					for joint in self
						.get_link(&key)
						.unwrap()
						.read()
						.unwrap() // FIXME: Unwrapping might not be ok
						.get_joints()
						.iter()
						.map(|joint| joint.read().unwrap())
					// FIXME: Unwrapping might not be ok
					{
						current_link
							.try_attach_child(
								Link::new(joint.get_child_link().read().unwrap().name.clone())
									.into(), // FIXME: Unwrapping might not be ok
								joint.rebuild(),
							)
							.unwrap()
					}
				}
			}

			change = key_count != tree.get_links().read().unwrap().len(); // FIXME: Unwrapping might not be ok
		}
		tree
	}
}

impl From<KinematicTree> for Box<dyn KinematicInterface> {
	fn from(value: KinematicTree) -> Self {
		Box::new(value)
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Weak};

	use crate::{
		link::{Link, LinkParent},
		Joint, JointType, KinematicInterface,
	};

	#[test]
	fn clone_single() {
		let tree = Link::new("example-link".into());
		let cloned_tree = tree.clone();

		println!("tree->data        | ptr: {:#?}", Arc::as_ptr(&tree.0));
		println!(
			"cloned_tree->data | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.0)
		);
		assert!(!Arc::ptr_eq(&tree.0, &cloned_tree.0));

		println!(
			"tree->..->root_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_root_link())
		);
		println!(
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		println!(
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().try_read().unwrap().name.as_ptr()
		);
		println!(
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.name
				.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().try_read().unwrap().get_name(),
			&cloned_tree.get_root_link().try_read().unwrap().get_name()
		);

		println!(
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().try_read().unwrap().tree)
		);
		println!(
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().try_read().unwrap().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().try_read().unwrap().tree,
			&cloned_tree.get_root_link().try_read().unwrap().tree
		));

		println!(
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(
				match &tree
					.get_root_link()
					.try_read()
					.unwrap()
					.get_parent()
					.unwrap()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		println!(
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree
					.get_root_link()
					.try_read()
					.unwrap()
					.get_parent()
					.unwrap()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().try_read().unwrap().get_parent(),
			&cloned_tree.get_root_link().try_read().unwrap().get_parent()
		);

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().get_joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().get_joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len()
		);

		println!(
			"tree->..->links        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_links())
		);
		println!(
			"cloned_tree->..->links | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_links())
		);
		assert!(!Arc::ptr_eq(&tree.get_links(), &cloned_tree.get_links()));
		assert_eq!(
			tree.get_links().try_read().unwrap().len(),
			cloned_tree.get_links().try_read().unwrap().len()
		);

		println!(
			"tree->..->links[\"example-link\"]        | ptr: {:#?}",
			Weak::as_ptr(
				&tree
					.get_links()
					.try_read()
					.unwrap()
					.get("example-link")
					.unwrap()
			)
		);
		println!(
			"cloned_tree->..->links[\"example-link\"] | ptr: {:#?}\n",
			Weak::as_ptr(
				&cloned_tree
					.get_links()
					.try_read()
					.unwrap()
					.get("example-link")
					.unwrap()
			)
		);
		assert!(!Weak::ptr_eq(
			&tree
				.get_links()
				.try_read()
				.unwrap()
				.get("example-link")
				.unwrap(),
			&cloned_tree
				.get_links()
				.try_read()
				.unwrap()
				.get("example-link")
				.unwrap()
		));

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().get_joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().get_joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len()
		);

		println!(
			"tree->..->joints        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_joints())
		);
		println!(
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Arc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().try_read().unwrap().len(),
			cloned_tree.get_joints().try_read().unwrap().len()
		);

		println!(
			"tree->..->newest_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_newest_link())
		);
		println!(
			"cloned_tree->..->newest_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_newest_link())
		);
		assert!(!Arc::ptr_eq(
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
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::new("other-child".into()).into(),
				Joint::new("other-child-joint".into(), JointType::Fixed),
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				other_tree.into(),
				Joint::new("other-joint".into(), JointType::Fixed),
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::new("3".into()).into(),
				Joint::new("three".into(), JointType::Fixed),
			)
			.unwrap();

		let cloned_tree = tree.clone();

		println!("tree->data        | ptr: {:#?}", Arc::as_ptr(&tree.0));
		println!(
			"cloned_tree->data | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.0)
		);
		assert!(!Arc::ptr_eq(&tree.0, &cloned_tree.0));

		println!(
			"tree->..->root_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_root_link())
		);
		println!(
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		println!(
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().try_read().unwrap().name.as_ptr()
		);
		println!(
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.name
				.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().try_read().unwrap().get_name(),
			&cloned_tree.get_root_link().try_read().unwrap().get_name()
		);

		println!(
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().try_read().unwrap().tree)
		);
		println!(
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().try_read().unwrap().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().try_read().unwrap().tree,
			&cloned_tree.get_root_link().try_read().unwrap().tree
		));

		println!(
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(
				match &tree
					.get_root_link()
					.try_read()
					.unwrap()
					.get_parent()
					.unwrap()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		println!(
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree
					.get_root_link()
					.try_read()
					.unwrap()
					.get_parent()
					.unwrap()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().try_read().unwrap().get_parent(),
			&cloned_tree.get_root_link().try_read().unwrap().get_parent()
		);

		println!(
			"tree->..->root_link->child_joints:        {:?}",
			&tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.iter()
				.map(|joint| joint.read().unwrap().name.clone())
				.collect::<Vec<String>>()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.iter()
				.map(|joint| joint.read().unwrap().name.clone())
				.collect::<Vec<String>>()
		);
		assert_eq!(
			tree.get_root_link().read().unwrap().get_joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len()
		);

		println!(
			"tree->..->links        | ptr: {:#?} | keys: {:?}",
			Arc::as_ptr(&tree.get_links()),
			&tree
				.get_links()
				.try_read()
				.unwrap()
				.keys()
				.map(|key| key.clone())
				.collect::<Vec<String>>()
		);
		println!(
			"cloned_tree->..->links | ptr: {:#?} | keys: {:?}\n",
			Arc::as_ptr(&cloned_tree.get_links()),
			&cloned_tree
				.get_links()
				.try_read()
				.unwrap()
				.keys()
				.map(|key| key.clone())
				.collect::<Vec<String>>()
		);
		assert!(!Arc::ptr_eq(&tree.get_links(), &cloned_tree.get_links()));
		assert_eq!(
			tree.get_links().try_read().unwrap().len(),
			cloned_tree.get_links().try_read().unwrap().len()
		);

		println!(
			"tree->..->links[\"example-link\"]        | ptr: {:#?}",
			Weak::as_ptr(
				&tree
					.get_links()
					.try_read()
					.unwrap()
					.get("example-link")
					.unwrap()
			)
		);
		println!(
			"cloned_tree->..->links[\"example-link\"] | ptr: {:#?}\n",
			Weak::as_ptr(
				&cloned_tree
					.get_links()
					.try_read()
					.unwrap()
					.get("example-link")
					.unwrap()
			)
		);
		assert!(!Weak::ptr_eq(
			&tree
				.get_links()
				.try_read()
				.unwrap()
				.get("example-link")
				.unwrap(),
			&cloned_tree
				.get_links()
				.try_read()
				.unwrap()
				.get("example-link")
				.unwrap()
		));

		println!(
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().get_joints()
		);
		println!(
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().get_joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().get_joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len()
		);

		println!(
			"tree->..->joints        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_joints())
		);
		println!(
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Arc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().try_read().unwrap().len(),
			cloned_tree.get_joints().try_read().unwrap().len()
		);

		println!(
			"tree->..->newest_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_newest_link())
		);
		println!(
			"cloned_tree->..->newest_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_newest_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_newest_link(),
			&cloned_tree.get_newest_link()
		));
	}
}
