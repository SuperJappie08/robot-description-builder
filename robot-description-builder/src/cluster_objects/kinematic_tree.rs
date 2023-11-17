use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLockWriteGuard},
};

use crate::{
	cluster_objects::{
		kinematic_data_errors::AddTransmissionError, kinematic_data_tree::KinematicDataTree,
		robot::Robot, KinematicInterface,
	},
	joint::Joint,
	link::Link,
	material::{data::MaterialData, Material},
	transmission::{
		transmission_builder_state::{WithActuator, WithJoints},
		Transmission, TransmissionBuilder,
	},
	utils::{ArcLock, WeakLock},
};

/// A representation of a basic Kinematic structure.
///
/// `KinematicTree`'s can be made from a single [`LinkBuilder`](crate::link::builder::LinkBuilder) by its [`build_tree()`](crate::link::builder::LinkBuilder::build_tree) method.
///
/// A `KinematicTree` can be upgraded to a [`Robot`] by calling [`to_robot`](KinematicTree::to_robot).
/// This will allow the `Robot` to be exported to [all supported Robot Description Formats](crate::to_rdf).
#[derive(Debug)]
pub struct KinematicTree(Arc<KinematicDataTree>);

impl KinematicTree {
	pub(crate) fn new(data: Arc<KinematicDataTree>) -> KinematicTree {
		KinematicTree(data)
	}

	/// Converts this unnamed `KinematicTree` to a [`Robot`], which can be represents a more finalized robot structure.
	pub fn to_robot(self, name: impl Into<String>) -> Robot {
		Robot::new(name, self.0)
	}
}

impl KinematicInterface for KinematicTree {
	fn get_root_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.0.root_link)
	}

	fn get_newest_link(&self) -> ArcLock<Link> {
		self.0.newest_link.read().unwrap().upgrade().unwrap() // FIXME: Unwrapping might not be ok
	}

	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>> {
		Arc::clone(&self.0.links)
	}

	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>> {
		Arc::clone(&self.0.joints)
	}

	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<MaterialData>>> {
		Arc::clone(&self.0.material_index)
	}

	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>> {
		Arc::clone(&self.0.transmissions)
	}

	fn get_link(&self, name: &str) -> Option<ArcLock<Link>> {
		self.0
			.links
			.read()
			/* In the future the lock could be saved by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.expect("The RwLock of the Link Index was poisoned. In the future this will be recoverable (mutex_unpoison).")
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>> {
		self.0
			.joints
			.read()
			/* In the future the lock could be saved by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.expect("The RwLock of the Joint Index was poisoned. In the future this will be recoverable (mutex_unpoison).")
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Material> {
		self.0
			.material_index
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
			.map(|data| Material::new_named_inited(name, data))
	}

	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>> {
		self.0
			.transmissions
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn try_add_transmission(
		&self,
		transmission: TransmissionBuilder<WithJoints, WithActuator>,
	) -> Result<(), AddTransmissionError> {
		self.0.try_add_transmission(transmission)
	}

	fn purge_links(&self) {
		self.0.purge_links()
	}

	fn purge_joints(&self) {
		self.0.purge_joints()
	}

	fn purge_materials(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<MaterialData>>>>> {
		self.0.purge_materials()
	}

	fn purge_transmissions(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<Transmission>>>>> {
		self.0.purge_transmissions()
	}
}

impl Clone for KinematicTree {
	fn clone(&self) -> Self {
		let root_link = self
			.get_root_link()
			.read()
			.unwrap() // FIXME: UNWRAP MIGHTN NOT BE OK HERE
			.rebuild_branch_continued()
			.unwrap(); // FIXME: UNWRAP MIGHTN NOT BE OK HERE

		root_link.build_tree()
	}
}

#[cfg(test)]
mod tests {
	use log::trace;
	use std::sync::{Arc, Weak};
	use test_log::test;

	use crate::{
		joint::{JointBuilder, JointType},
		link::{builder::LinkBuilder, link_data::LinkParent, Link},
		KinematicInterface,
	};

	#[test]
	fn clone_single() {
		let tree = Link::builder("example-link").build_tree();
		let cloned_tree = tree.clone();

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->data        | ptr: {:#?}",
			Arc::as_ptr(&tree.0)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->data | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.0)
		);
		assert!(!Arc::ptr_eq(&tree.0, &cloned_tree.0));

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_root_link())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().try_read().unwrap().name().as_ptr()
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.name()
				.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().try_read().unwrap().name(),
			&cloned_tree.get_root_link().try_read().unwrap().name()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().try_read().unwrap().tree)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().try_read().unwrap().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().try_read().unwrap().tree,
			&cloned_tree.get_root_link().try_read().unwrap().tree
		));

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(
				match &tree
					.get_root_link()
					.try_read()
					.unwrap()
					.parent()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree
					.get_root_link()
					.try_read()
					.unwrap()
					.parent()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().try_read().unwrap().parent(),
			&cloned_tree.get_root_link().try_read().unwrap().parent()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().joints()
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->links        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_links())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->links | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_links())
		);
		assert!(!Arc::ptr_eq(&tree.get_links(), &cloned_tree.get_links()));
		assert_eq!(
			tree.get_links().try_read().unwrap().len(),
			cloned_tree.get_links().try_read().unwrap().len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
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
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->links[\"example-link\"] | ptr: {:#?}\n",
			Weak::as_ptr(
				&cloned_tree
					.get_links()
					.try_read()
					.unwrap().get("example-link")
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

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().joints()
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->joints        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_joints())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Arc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().try_read().unwrap().len(),
			cloned_tree.get_joints().try_read().unwrap().len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
			"tree->..->newest_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_newest_link())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_single",
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
		let tree = LinkBuilder::new("example-link").build_tree();
		let other_tree = LinkBuilder::new("other-link").build_tree();
		other_tree
			.get_newest_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				JointBuilder::new("other-child-joint", JointType::Fixed),
				LinkBuilder::new("other-child").build_tree(),
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				JointBuilder::new("other-joint", JointType::Fixed),
				other_tree,
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				JointBuilder::new("three", JointType::Fixed),
				LinkBuilder::new("3"),
			)
			.unwrap();

		let cloned_tree = tree.clone();

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->data        | ptr: {:#?}",
			 Arc::as_ptr(&tree.0)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->data | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.0)
		);
		assert!(!Arc::ptr_eq(&tree.0, &cloned_tree.0));

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_root_link())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->root_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_root_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_root_link(),
			&cloned_tree.get_root_link()
		));

		// Note: This may not be permanent behavior
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link->name        | ptr: {:#?}",
			&tree.get_root_link().try_read().unwrap().name().as_ptr()
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->root_link->name | ptr: {:#?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.name()
				.as_ptr()
		);
		assert_eq!(
			&tree.get_root_link().try_read().unwrap().name(),
			&cloned_tree.get_root_link().try_read().unwrap().name()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link->tree        | ptr: {:#?}",
			Weak::as_ptr(&tree.get_root_link().try_read().unwrap().tree)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->root_link->tree | ptr: {:#?}\n",
			Weak::as_ptr(&cloned_tree.get_root_link().try_read().unwrap().tree)
		);
		assert!(!Weak::ptr_eq(
			&tree.get_root_link().try_read().unwrap().tree,
			&cloned_tree.get_root_link().try_read().unwrap().tree
		));

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link->direct_parent->0        | ptr: {:#?}",
			Weak::as_ptr(
				match &tree
					.get_root_link()
					.try_read()
					.unwrap()
					.parent()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->root_link->direct_parent->0 | ptr: {:#?}\n",
			Weak::as_ptr(
				match &cloned_tree
					.get_root_link()
					.try_read()
					.unwrap()
					.parent()
				{
					LinkParent::KinematicTree(weak_tree) => weak_tree,
					LinkParent::Joint(_) => panic!("This should not return a Joint Parent"),
				}
			)
		);
		assert_ne!(
			&tree.get_root_link().try_read().unwrap().parent(),
			&cloned_tree.get_root_link().try_read().unwrap().parent()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link->child_joints:        {:?}",
			&tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.iter()
				.map(|joint| joint.read().unwrap().name().clone())
				.collect::<Vec<String>>()
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->root_link->child_joints: {:?}\n",
			&cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.iter()
				.map(|joint| joint.read().unwrap().name().clone())
				.collect::<Vec<String>>()
		);
		assert_eq!(
			tree.get_root_link().read().unwrap().joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
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
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
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

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
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
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
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

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->root_link->child_joints:        {:#?}",
			&tree.get_root_link().try_read().unwrap().joints()
		);
		trace!(
			target: "RDB::test::KineTree::clone_multi",
			"cloned_tree->..->root_link->child_joints: {:#?}\n",
			&cloned_tree.get_root_link().try_read().unwrap().joints()
		);
		assert_eq!(
			tree.get_root_link().try_read().unwrap().joints().len(),
			cloned_tree
				.get_root_link()
				.try_read()
				.unwrap()
				.joints()
				.len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->joints        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_joints())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->joints | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_joints())
		);
		assert!(!Arc::ptr_eq(&tree.get_joints(), &cloned_tree.get_joints()));
		assert_eq!(
			tree.get_joints().try_read().unwrap().len(),
			cloned_tree.get_joints().try_read().unwrap().len()
		);

		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"tree->..->newest_link        | ptr: {:#?}",
			Arc::as_ptr(&tree.get_newest_link())
		);
		trace!(
			target: "RDB-RS::test::KineTree::clone_multi",
			"cloned_tree->..->newest_link | ptr: {:#?}\n",
			Arc::as_ptr(&cloned_tree.get_newest_link())
		);
		assert!(!Arc::ptr_eq(
			&tree.get_newest_link(),
			&cloned_tree.get_newest_link()
		));
	}
}
