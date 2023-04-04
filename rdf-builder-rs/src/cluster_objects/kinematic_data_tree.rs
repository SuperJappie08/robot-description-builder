use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLock, RwLockWriteGuard, Weak},
};

use itertools::{process_results, Itertools};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode, URDFMaterialReferences};
use crate::{
	joint::Joint,
	link::{builder::BuildLink, Link},
	material_mod::{Material, MaterialData},
	transmission::Transmission,
	ArcLock, WeakLock,
};

use crate::cluster_objects::kinematic_data_errors::*;

#[derive(Debug)]
pub struct KinematicDataTree {
	pub(crate) root_link: ArcLock<Link>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	// These index maps are ArcLock in order to be exposable to the outside world via the ref of this thing
	/// TODO:? Maybe make materials immutable after creation.
	pub(crate) material_index: ArcLock<HashMap<String, ArcLock<MaterialData>>>,
	pub(crate) links: ArcLock<HashMap<String, WeakLock<Link>>>,
	pub(crate) joints: ArcLock<HashMap<String, WeakLock<Joint>>>,
	/// Do Transmission have to wrapped into ArcLock? Maybe we can get a way with raw stuff?
	/// Don't now it would unpack on the Python side...
	pub(crate) transmissions: ArcLock<HashMap<String, ArcLock<Transmission>>>,
	/// The most recently updated `Link`
	pub(crate) newest_link: RwLock<WeakLock<Link>>,
	// is_rigid: bool // ? For gazebo -> TO AdvancedSimulationData [ASD]
}

impl KinematicDataTree {
	pub(crate) fn newer_link(root_link_builder: impl BuildLink) -> Arc<Self> {
		let data = Arc::new_cyclic(|tree| Self {
			root_link: root_link_builder.start_building_chain(tree),
			material_index: Arc::new(RwLock::new(HashMap::new())),
			links: Arc::new(RwLock::new(HashMap::new())),
			joints: Arc::new(RwLock::new(HashMap::new())),
			transmissions: Arc::new(RwLock::new(HashMap::new())),
			newest_link: RwLock::new(Weak::new()),
		});

		{
			#[cfg(any(feature = "logging", test))]
			log::trace!("Attempting to register tree data to index");

			// Unwrapping is Ok here, since we just made the KinematicDataTree
			let root_link = Arc::clone(&data.root_link);

			// 1st Unwrapping is Ok here, since we just made the KinematicDataTree
			data.try_add_link(root_link).unwrap(); //FIXME: 2nd Unwrap Ok?
		}
		data
	}

	pub(crate) fn try_add_material(&self, material: &mut Material) -> Result<(), AddMaterialError> {
		material.initialize(self).unwrap(); // FIXME: Is unwrap Ok here?

		Ok(())
	}

	/// This might replace try_add_link at some point, when i figure out of this contual building works?
	/// But it loops throug everything which could be a lot...
	///
	/// Never mind it only will loop over things down stream.
	/// It might actually be worth doing it
	///
	/// I have done it.
	pub(crate) fn try_add_link(&self, link: ArcLock<Link>) -> Result<(), AddLinkError> {
		let name = link.read()?.get_name().clone();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Link: {}", name);

		let other = { self.links.read()?.get(&name) }.map(Weak::clone);
		if let Some(preexisting_link) = other.and_then(|weak_link| weak_link.upgrade()) {
			if !Arc::ptr_eq(&preexisting_link, &link) {
				return Err(AddLinkError::Conflict(name));
			}
		} else {
			assert!(self
				.links
				.write()?
				.insert(name, Arc::downgrade(&link))
				.is_none());
			*self.newest_link.write().unwrap() = Arc::downgrade(&link); //FIXME: Unwrap Ok?
		}

		process_results(
			link.try_write()
				.unwrap() // FIXME: Figure out if unwrap Ok?
				.get_visuals_mut()
				.iter_mut()
				.filter_map(|visual| visual.get_material_mut())
				.map(|material| match self.try_add_material(material) {
					Err(AddMaterialError::NoName) => Ok(()), // TODO: SHOULD LOG HERE or earlier???
					o => o,
				}),
			|iter| iter.collect_vec(),
		)
		.unwrap(); // FIXME: THIS IS TEMP

		process_results(
			link.read()
				.unwrap() // FIXME: Figureout if unwrap Ok?
				.get_joints()
				.iter()
				.map(|joint| self.try_add_joint(joint)),
			|iter| iter.collect_vec(),
		)
		.unwrap(); // FIXME: THIS IS TEMP

		Ok(())
	}

	pub(crate) fn try_add_joint(&self, joint: &ArcLock<Joint>) -> Result<(), AddJointError> {
		let joint_binding = joint.read()?;
		let name = joint_binding.get_name();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Joint: {}", name);

		let other = { self.joints.read()?.get(name) }.map(Weak::clone);
		if let Some(preexisting_joint) = other.and_then(|weak_joint| weak_joint.upgrade()) {
			if !Arc::ptr_eq(&preexisting_joint, joint) {
				return Err(AddJointError::Conflict(name.into()));
			}
		} else {
			assert!(self
				.joints
				.write()?
				.insert(name.into(), Arc::downgrade(joint))
				.is_none());
		}

		self.try_add_link(joint.read().unwrap().get_child_link())
			.unwrap(); //FIXME: Unwrap is not OK here
		Ok(())
	}

	pub(crate) fn try_add_transmission(
		&self,
		transmission: ArcLock<Transmission>,
	) -> Result<(), AddTransmissionError> {
		let name = transmission.read()?.get_name().clone();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Transmission: {}", name);

		let other_transmission = { self.transmissions.read()?.get(&name) }.map(Arc::clone);
		if let Some(preexisting_transmission) = other_transmission {
			if !Arc::ptr_eq(&preexisting_transmission, &transmission) {
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

	/// Cleans up orphaned/broken `Joint` entries from the `joints` HashMap.
	pub(crate) fn purge_joints(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<'_, HashMap<String, WeakLock<Joint>>>>> {
		let mut joints = self.joints.write()?;
		joints.retain(|_, weak_joint| weak_joint.upgrade().is_some());
		joints.shrink_to_fit();
		Ok(())
	}

	/// Cleans up orphaned/broken `Link` entries from the `links` HashMap.
	pub(crate) fn purge_links(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<'_, HashMap<String, WeakLock<Link>>>>> {
		let mut links = self.links.write()?;
		links.retain(|_, weak_link| weak_link.upgrade().is_some());
		links.shrink_to_fit();
		Ok(())
	}

	/// Cleans up orphaned/unused `Material` entries from `material_index` HashMap
	///
	/// TODO: Check if this works
	/// FIXME: This doesn't work if you hace multiple robots using the same material.
	pub(crate) fn purge_materials(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<MaterialData>>>>> {
		let mut materials = self.material_index.write()?;
		materials.retain(|_, material| Arc::strong_count(material) > 1);
		materials.shrink_to_fit();
		Ok(())
	}

	/// Cleans up orphaned/broken `Transmission` entries from the `transmissions` HashMap
	pub(crate) fn purge_transmissions(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<Transmission>>>>> {
		// Ok(())
		todo!("Not Implemnted yet! First Implement `Transmission`")
	}

	/// TODO: DECIDE IF DEPRECATE?
	/// Cleans up broken `Joint` and `Link` entries from the `links` and `joints` HashMaps.
	///
	/// TODO: Rewrite DOC
	pub(crate) fn purge(&self) {
		self.purge_joints().unwrap(); //FIXME: UNWRAP?

		self.purge_links().unwrap(); //FIXME: UNWRAP?

		//TODO: UPDATE FOR MATERIALS
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for KinematicDataTree {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &URDFConfig,
	) -> Result<(), quick_xml::Error> {
		// Write Materials if use > 2 depending on Config
		// That might already be handled by the new material system
		// TODO: Config stuff
		process_results(
			self.material_index
				.read()
				.unwrap() // FIXME: Is unwrap ok here?
				.values()
				.filter(|material| {
					match urdf_config.material_references {
						URDFMaterialReferences::AllNamedMaterialOnTop => true,
						URDFMaterialReferences::OnlyMultiUseMaterials => {
							Arc::strong_count(material) > 2
						} // Weak::strong_count(material_ref)
					}
				})
				.map(|material| {
					material.read().unwrap().to_urdf(
						writer,
						&URDFConfig {
							direct_material_ref: URDFMaterialMode::FullMaterial,
							..urdf_config.clone()
						},
					)
				}),
			|iter| iter.collect(),
		)?;

		self.root_link
			.read()
			.unwrap()
			.to_urdf(writer, urdf_config)?; // FIXME: Is unwrap ok here?

		process_results(
			self.transmissions
				.read()
				.unwrap() // FIXME: Is unwrap ok here?
				.values()
				.map(|transmission| transmission.read().unwrap().to_urdf(writer, urdf_config)), // FIXME: Is unwrap ok here?
			|iter| iter.collect(),
		)?;

		Ok(())
	}
}

impl PartialEq for KinematicDataTree {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.root_link, &other.root_link)
			&& Weak::ptr_eq(
				&self.newest_link.read().unwrap(),
				&other.newest_link.read().unwrap(),
			)
		// TODO: Check other things
		// && self.material_index == other.material_index
		// && self.transmissions == other.transmissions
	}
}
// impl KinematicTreeTrait for KinematicTreeData {}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Weak};
	use test_log::test;

	use crate::{
		cluster_objects::kinematic_data_tree::KinematicDataTree,
		joint::{JointBuilder, JointType},
		link::builder::LinkBuilder,
		link_data::LinkParent,
	};

	// 	#[test]
	// 	fn new_link() {
	// 		let data_tree = KinematicTreeData::new_link(Link {name: "Linky", ..Default::default()});
	// 	}

	#[test]
	fn newer_link_singular_empty() {
		let tree = KinematicDataTree::newer_link(LinkBuilder::new("Linky"));

		assert_eq!(tree.links.try_read().unwrap().len(), 1);
		assert_eq!(tree.joints.try_read().unwrap().len(), 0);
		assert_eq!(tree.material_index.try_read().unwrap().len(), 0);
		assert_eq!(tree.transmissions.try_read().unwrap().len(), 0);

		assert!(tree.links.try_read().unwrap().contains_key("Linky"));
		assert_eq!(tree.root_link.try_read().unwrap().get_name(), "Linky");
		assert_eq!(
			tree.newest_link
				.read()
				.unwrap()
				.upgrade()
				.unwrap()
				.try_read()
				.unwrap()
				.get_name(),
			"Linky"
		);

		assert!(Arc::ptr_eq(
			&tree.root_link,
			&tree.newest_link.read().unwrap().upgrade().unwrap()
		));
		assert!(tree
			.root_link
			.try_read()
			.unwrap()
			.get_parent()
			.is_valid_reference());
		assert!(match tree.root_link.try_read().unwrap().get_parent() {
			LinkParent::KinematicTree(_) => true,
			_ => false,
		});
	}

	#[test]
	fn newer_link_multi_empty() {
		let tree = KinematicDataTree::newer_link(LinkBuilder {
			joints: vec![
				JointBuilder {
					child: Some(LinkBuilder {
						joints: vec![JointBuilder {
							child: Some(LinkBuilder::new("other-child")),
							..JointBuilder::new("other-child-joint", JointType::Fixed)
						}],
						..LinkBuilder::new("other-link")
					}),
					..JointBuilder::new("other-joint", JointType::Fixed)
				},
				JointBuilder {
					child: Some(LinkBuilder::new("3")),
					..JointBuilder::new("three", JointType::Fixed)
				},
			],
			..LinkBuilder::new("example-link")
		});

		assert_eq!(tree.links.try_read().unwrap().len(), 4);
		assert_eq!(tree.joints.try_read().unwrap().len(), 3);
		assert_eq!(tree.material_index.try_read().unwrap().len(), 0);
		assert_eq!(tree.transmissions.try_read().unwrap().len(), 0);

		let mut link_keys: Vec<String> = tree.links.try_read().unwrap().keys().cloned().collect();
		link_keys.sort();
		assert_eq!(
			link_keys,
			vec!["3", "example-link", "other-child", "other-link",]
		);

		let mut joint_keys: Vec<String> = tree.joints.try_read().unwrap().keys().cloned().collect();
		joint_keys.sort();
		assert_eq!(
			joint_keys,
			vec!["other-child-joint", "other-joint", "three",]
		);

		assert_eq!(
			tree.root_link.try_read().unwrap().get_name(),
			"example-link"
		);
		assert_eq!(
			tree.newest_link
				.read()
				.unwrap()
				.upgrade()
				.unwrap()
				.try_read()
				.unwrap()
				.get_name(),
			"3"
		);

		assert!(tree
			.root_link
			.try_read()
			.unwrap()
			.get_parent()
			.is_valid_reference());
		assert!(match tree.root_link.try_read().unwrap().get_parent() {
			LinkParent::KinematicTree(_) => true,
			_ => false,
		});

		assert_eq!(
			tree.root_link
				.try_read()
				.unwrap()
				.get_joints()
				.iter()
				.map(|joint| joint.try_read().unwrap().get_name().clone())
				.collect::<Vec<String>>(),
			vec!["other-joint", "three"]
		);

		// Start childs of 'example-link'

		{
			let joint = tree
				.joints
				.try_read()
				.unwrap()
				.get("other-joint")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(joint.try_read().unwrap().get_name(), "other-joint");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().get_parent_link(),
				&tree.root_link
			));
			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().get_child_link(),
				&tree
					.links
					.try_read()
					.unwrap()
					.get("other-link")
					.unwrap()
					.upgrade()
					.unwrap()
			));
		}

		// Start childs of 'other-joint'

		{
			let link = tree
				.links
				.try_read()
				.unwrap()
				.get("other-link")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(link.try_read().unwrap().get_name(), "other-link");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().get_parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().get_parent() {
					LinkParent::Joint(joint) => joint,
					LinkParent::KinematicTree(_) => panic!("Not ok"),
				},
				&tree.joints.try_read().unwrap().get("other-joint").unwrap()
			));

			assert_eq!(
				link.try_read()
					.unwrap()
					.get_joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().get_name().clone())
					.collect::<Vec<String>>(),
				vec!["other-child-joint"]
			);
		}

		{
			let joint = tree
				.joints
				.try_read()
				.unwrap()
				.get("other-child-joint")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(joint.try_read().unwrap().get_name(), "other-child-joint");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Weak::ptr_eq(
				&joint.try_read().unwrap().parent_link,
				&tree.links.try_read().unwrap().get("other-link").unwrap()
			));

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().get_child_link(),
				&tree
					.links
					.try_read()
					.unwrap()
					.get("other-child")
					.unwrap()
					.upgrade()
					.unwrap()
			));
		}

		{
			let link = tree
				.links
				.try_read()
				.unwrap()
				.get("other-child")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(link.try_read().unwrap().get_name(), "other-child");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().get_parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().get_parent() {
					LinkParent::Joint(joint) => joint,
					LinkParent::KinematicTree(_) => panic!("Not ok"),
				},
				&tree
					.joints
					.try_read()
					.unwrap()
					.get("other-child-joint")
					.unwrap()
			));

			assert_eq!(
				link.try_read()
					.unwrap()
					.get_joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().get_name().clone())
					.count(),
				0
			);
		}

		// Start child 2 of 'example-link'
		{
			let joint = tree
				.joints
				.try_read()
				.unwrap()
				.get("three")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(joint.try_read().unwrap().get_name(), "three");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().get_parent_link(),
				&tree.root_link
			));
			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().get_child_link(),
				&tree
					.links
					.try_read()
					.unwrap()
					.get("3")
					.unwrap()
					.upgrade()
					.unwrap()
			));
		}

		{
			let link = tree
				.links
				.try_read()
				.unwrap()
				.get("3")
				.unwrap()
				.upgrade()
				.unwrap();

			assert_eq!(link.try_read().unwrap().get_name(), "3");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().get_parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().get_parent() {
					LinkParent::Joint(joint) => joint,
					LinkParent::KinematicTree(_) => panic!("Not ok"),
				},
				&tree.joints.try_read().unwrap().get("three").unwrap()
			));

			assert_eq!(
				link.try_read()
					.unwrap()
					.get_joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().get_name().clone())
					.count(),
				0
			);
		}
	}

	#[test]
	#[ignore]
	fn newer_link_singular_full() {
		todo!()
	}

	#[test]
	#[ignore]
	fn newer_link_multi_full() {
		todo!()
	}
}