use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLock, Weak},
};

use itertools::Itertools;

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode, URDFMaterialReferences};
use crate::{
	joint::Joint,
	link::{builder::BuildLink, Link},
	link_data::Visual,
	material::{data::MaterialData, Material},
	transmission::{
		transmission_builder_state::{WithActuator, WithJoints},
		Transmission, TransmissionBuilder,
	},
	utils::{ArcLock, ArcRW, ErroredWrite, WeakLock},
};

use super::{kinematic_data_errors::*, PoisonWriteIndexError};

#[derive(Debug)]
pub struct KinematicDataTree {
	/// The root `Link` of the kinematic tree.
	pub(super) root_link: ArcLock<Link>,
	//TODO: In this implementation the Keys, are not linked to the objects and could be changed.
	// These index maps are ArcLock in order to be exposable to the outside world via the ref of this thing
	// TODO:? Maybe make materials immutable after creation.
	pub(crate) material_index: ArcLock<HashMap<String, ArcLock<MaterialData>>>,
	pub(crate) links: ArcLock<HashMap<String, WeakLock<Link>>>,
	pub(crate) joints: ArcLock<HashMap<String, WeakLock<Joint>>>,
	// Do Transmission have to wrapped into ArcLock? Maybe we can get a way with raw stuff?
	// Don't now it would unpack on the Python side...
	pub(crate) transmissions: ArcLock<HashMap<String, ArcLock<Transmission>>>,
	/// The most recently updated `Link`.
	pub(crate) newest_link: RwLock<WeakLock<Link>>,
	// is_rigid: bool // ? For gazebo -> TO AdvancedSimulationData [ASD]
	me: Weak<Self>,
}

impl KinematicDataTree {
	pub(crate) fn new(root_link_builder: impl BuildLink) -> Arc<Self> {
		let data = Arc::new_cyclic(|tree| Self {
			root_link: root_link_builder.start_building_chain(tree),
			material_index: Arc::new(RwLock::new(HashMap::new())),
			links: Arc::new(RwLock::new(HashMap::new())),
			joints: Arc::new(RwLock::new(HashMap::new())),
			transmissions: Arc::new(RwLock::new(HashMap::new())),
			newest_link: RwLock::new(Weak::new()),
			me: Weak::clone(tree),
		});

		{
			#[cfg(any(feature = "logging", test))]
			log::trace!("Attempting to register tree data to index");

			//FIXME: This unwrap is not Ok, the Link could contain conflicting materials
			data.try_add_link(&data.root_link).unwrap();
		}
		data
	}

	pub(crate) fn try_add_material(&self, material: &mut Material) -> Result<(), AddMaterialError> {
		material.initialize(self)
	}

	// This might replace try_add_link at some point, when i figure out of this contual building works?
	// But it loops throug everything which could be a lot...
	//
	// Never mind it only will loop over things down stream.
	// It might actually be worth doing it
	//
	// I have done it.
	pub(crate) fn try_add_link(&self, link: &ArcLock<Link>) -> Result<(), AttachChainError> {
		let name = link
			.mread()
			.map_err(AddLinkError::ReadNewLink)?
			.name()
			.clone();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Link: {}", name);

		let other = self
			.links
			.mread()
			/* In the future this lock might be saveable by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.map_err(AddLinkError::ReadIndex)?
			.get(&name)
			.map(Weak::clone);
		if let Some(preexisting_link) = other.and_then(|weak_link| weak_link.upgrade()) {
			if !Arc::ptr_eq(&preexisting_link, link) {
				return Err(AttachChainError::Link(AddLinkError::Conflict(name)));
			}
		} else {
			assert!(self
				.links
				.mwrite()
				/* In the future this lock might be saveable by overwriting with a newly generated index,
				however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
				.map_err(AddLinkError::WriteIndex)?
				.insert(name, Arc::downgrade(link))
				.is_none());
			*self
				.newest_link
				.write()
				/* In the future the lock could be saved but waiting for
				"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
				.map_err(|_| PoisonError::new(ErroredWrite(self.me.upgrade().unwrap())))? = Arc::downgrade(link);
		}

		link.mwrite()
			.map_err(AddLinkError::WriteNewLink)? // TODO: Don't think this is can occure.
			.visuals_mut()
			.iter_mut()
			.filter_map(Visual::material_mut)
			.map(|material| self.try_add_material(material))
			.process_results(|iter| iter.collect_vec())?;

		link.mread()
			.map_err(AddLinkError::ReadNewLink)? // TODO: Don't think this is can occure.
			.joints()
			.iter()
			.map(|joint| self.try_add_joint(joint))
			.process_results(|iter| iter.collect_vec())?;

		Ok(())
	}

	pub(crate) fn try_add_joint(&self, joint: &ArcLock<Joint>) -> Result<(), AttachChainError> {
		let name = joint
			.mread()
			.map_err(AddJointError::ReadNewJoint)?
			.name()
			.clone();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Joint: {}", name);

		let other = self
			.joints
			.mread()
			/* In the future this lock might be saveable by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.map_err(AddJointError::ReadIndex)?
			.get(&name)
			.map(Weak::clone);
		if let Some(preexisting_joint) = other.and_then(|weak_joint| weak_joint.upgrade()) {
			if !Arc::ptr_eq(&preexisting_joint, joint) {
				return Err(AttachChainError::Joint(AddJointError::Conflict(name)));
			}
		// Multi Adding should not occure
		} else {
			assert!(self
				.joints
				.mwrite()
				/* In the future this lock might be saveable by overwriting with a newly generated index,
				however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
				.map_err(AddJointError::WriteIndex)?
				.insert(name, Arc::downgrade(joint))
				.is_none());
		}

		self.try_add_link(
			joint
				.mread()
				.map_err(AddJointError::ReadNewJoint)?
				.child_link_ref(),
		)?;

		Ok(())
	}

	pub(crate) fn try_add_transmission(
		&self,
		transmission: TransmissionBuilder<WithJoints, WithActuator>,
	) -> Result<(), AddTransmissionError> {
		let name = transmission.name().clone();

		#[cfg(any(feature = "logging", test))]
		log::debug!(target: "KinematicTreeData","Trying to attach Transmission: {}", name);

		let other_transmission = self.transmissions.mread()?.get(&name).map(Arc::clone);
		if other_transmission.is_some() {
			Err(AddTransmissionError::Conflict(name))
		} else {
			assert!(self
				.transmissions
				.mwrite()?
				.insert(name, Arc::new(RwLock::new(transmission.build(&self.me)?)))
				.is_none());
			Ok(())
		}
	}

	/// Cleans up orphaned/broken `Joint` entries from the `joints` HashMap.
	pub(crate) fn purge_joints(&self) {
		/* In the future the lock could be saved by overwriting with a newly generated index,
		however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
		let mut joints = self.joints.write().expect("The RwLock of the Joint Index was poisoned. In the future this will be recoverable (mutex_unpoison).");
		joints.retain(|_, weak_joint| weak_joint.upgrade().is_some());
		joints.shrink_to_fit()
	}

	/// Cleans up orphaned/broken `Link` entries from the `links` HashMap.
	pub(crate) fn purge_links(&self) {
		/* In the future the lock could be saved by overwriting with a newly generated index,
		however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
		let mut links = self.links.write().expect("The RwLock of the Link Index was poisoned. In the future this will be recoverable (mutex_unpoison).");
		links.retain(|_, weak_link| weak_link.upgrade().is_some());
		links.shrink_to_fit()
	}

	/// Cleans up orphaned/unused `Material` entries from `material_index` HashMap.
	// TODO: Check if this works
	pub(crate) fn purge_materials(
		&self,
	) -> Result<(), PoisonWriteIndexError<String, ArcLock<MaterialData>>> {
		/* In the future the lock could be saved by overwriting with a newly generated index,
		however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
		let mut materials = self.material_index.write()?;
		materials.retain(|_, material| Arc::strong_count(material) > 1);
		materials.shrink_to_fit();
		Ok(())
	}

	/// Cleans up orphaned/broken `Transmission` entries from the `transmissions` HashMap.
	pub(crate) fn purge_transmissions(
		&self,
	) -> Result<(), PoisonWriteIndexError<String, ArcLock<Transmission>>> {
		// Ok(())
		// TODO:
		todo!("Not Implemnted yet! First Implement `Transmission`")
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

		self.material_index
			.read()
			.unwrap() // FIXME: Is unwrap ok here?
			.iter()
			.filter(|(_, material_data)| {
				// Maybe do this differently?
				match urdf_config.material_references {
					URDFMaterialReferences::AllNamedMaterialOnTop => true,
					URDFMaterialReferences::OnlyMultiUseMaterials => {
						Arc::strong_count(material_data) > 2
					}
					_ => false,
				}
			})
			.map(
				|(name, arc_material_data)| {
					Material::new_named_inited(name.clone(), Arc::clone(arc_material_data))
				}, // FIXME: This might be a bit weird to do it like this, a propper construction method would be nice
			)
			.sorted_by_cached_key(|material| material.name().unwrap().clone()) // TODO: Is it worth to make sorting optional?
			.map(|material: Material| {
				material.to_urdf(
					writer,
					&URDFConfig {
						direct_material_ref: URDFMaterialMode::FullMaterial,
						..urdf_config.clone()
					},
				)
			})
			.process_results(|iter| iter.collect())?;

		self.root_link
			.read()
			.unwrap() // FIXME: Is unwrap ok here?
			.to_urdf(
				writer,
				&URDFConfig {
					direct_material_ref: {
						// TODO: Added this for future ALL MATERIALS
						use URDFMaterialReferences::*;
						match urdf_config.material_references {
							AllNamedMaterialOnTop | OnlyMultiUseMaterials => {
								URDFMaterialMode::Referenced
							}
							AlwaysInline => URDFMaterialMode::FullMaterial,
						}
					},
					..urdf_config.clone()
				},
			)?;

		self.transmissions
			.read()
			.unwrap() // FIXME: Is unwrap ok here?
			.values()
			.map(|transmission| transmission.read().unwrap().to_urdf(writer, urdf_config)) // FIXME: Is unwrap ok here?
			.process_results(|iter| iter.collect())?;

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
		let tree = KinematicDataTree::new(LinkBuilder::new("Linky"));

		assert_eq!(tree.links.try_read().unwrap().len(), 1);
		assert_eq!(tree.joints.try_read().unwrap().len(), 0);
		assert_eq!(tree.material_index.try_read().unwrap().len(), 0);
		assert_eq!(tree.transmissions.try_read().unwrap().len(), 0);

		assert!(tree.links.try_read().unwrap().contains_key("Linky"));
		assert_eq!(tree.root_link.try_read().unwrap().name(), "Linky");
		assert_eq!(
			tree.newest_link
				.read()
				.unwrap()
				.upgrade()
				.unwrap()
				.try_read()
				.unwrap()
				.name(),
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
			.parent()
			.is_valid_reference());
		assert!(match tree.root_link.try_read().unwrap().parent() {
			LinkParent::KinematicTree(_) => true,
			_ => false,
		});
	}

	#[test]
	fn newer_link_multi_empty() {
		let tree = KinematicDataTree::new(LinkBuilder {
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

		assert_eq!(tree.root_link.try_read().unwrap().name(), "example-link");
		assert_eq!(
			tree.newest_link
				.read()
				.unwrap()
				.upgrade()
				.unwrap()
				.try_read()
				.unwrap()
				.name(),
			"3"
		);

		assert!(tree
			.root_link
			.try_read()
			.unwrap()
			.parent()
			.is_valid_reference());
		assert!(match tree.root_link.try_read().unwrap().parent() {
			LinkParent::KinematicTree(_) => true,
			_ => false,
		});

		assert_eq!(
			tree.root_link
				.try_read()
				.unwrap()
				.joints()
				.iter()
				.map(|joint| joint.try_read().unwrap().name().clone())
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

			assert_eq!(joint.try_read().unwrap().name(), "other-joint");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().parent_link(),
				&tree.root_link
			));
			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().child_link(),
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

			assert_eq!(link.try_read().unwrap().name(), "other-link");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().parent() {
					LinkParent::Joint(joint) => joint,
					LinkParent::KinematicTree(_) => panic!("Not ok"),
				},
				&tree.joints.try_read().unwrap().get("other-joint").unwrap()
			));

			assert_eq!(
				link.try_read()
					.unwrap()
					.joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().name().clone())
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

			assert_eq!(joint.try_read().unwrap().name(), "other-child-joint");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Weak::ptr_eq(
				&joint.try_read().unwrap().parent_link,
				&tree.links.try_read().unwrap().get("other-link").unwrap()
			));

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().child_link(),
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

			assert_eq!(link.try_read().unwrap().name(), "other-child");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().parent() {
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
					.joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().name().clone())
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

			assert_eq!(joint.try_read().unwrap().name(), "three");
			assert!(joint.try_read().unwrap().tree.upgrade().is_some());

			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().parent_link(),
				&tree.root_link
			));
			assert!(Arc::ptr_eq(
				&joint.try_read().unwrap().child_link(),
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

			assert_eq!(link.try_read().unwrap().name(), "3");
			assert!(link.try_read().unwrap().tree.upgrade().is_some());

			assert!(link.try_read().unwrap().parent().is_valid_reference());
			assert!(Weak::ptr_eq(
				match link.try_read().unwrap().parent() {
					LinkParent::Joint(joint) => joint,
					LinkParent::KinematicTree(_) => panic!("Not ok"),
				},
				&tree.joints.try_read().unwrap().get("three").unwrap()
			));

			assert_eq!(
				link.try_read()
					.unwrap()
					.joints()
					.iter()
					.map(|joint| joint.try_read().unwrap().name().clone())
					.count(),
				0
			);
		}
	}

	#[test]
	#[ignore = "TODO"]
	fn newer_link_singular_full() {
		todo!()
	}

	#[test]
	#[ignore = "TODO"]
	fn newer_link_multi_full() {
		todo!()
	}

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::{io::Seek, sync::Arc};

		use super::{test, KinematicDataTree};
		use crate::{
			link::link_data::{
				geometry::{BoxGeometry, CylinderGeometry},
				Collision, Visual,
			},
			link::Link,
			material::MaterialDescriptor,
			to_rdf::{
				to_urdf::{ToURDF, URDFConfig, URDFMaterialReferences},
				XMLMode,
			},
			SmartJointBuilder, Transform,
		};

		fn test_to_urdf_kinematic_data_tree(
			kinematic_data_tree: Arc<KinematicDataTree>,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = match urdf_config.xml_mode {
				XMLMode::NoIndent => quick_xml::Writer::new(std::io::Cursor::new(Vec::new())),
				XMLMode::Indent(c, depth) => quick_xml::Writer::new_with_indent(
					std::io::Cursor::new(Vec::new()),
					c as u8,
					depth,
				),
			};
			assert!(kinematic_data_tree
				.to_urdf(&mut writer, urdf_config)
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		/// FIXME: This test checks for weird behavior, since I ran into an free floating [`MaterialData`](crate::material_mod::MaterialData).
		///			It also show both root level materials and inline both is kind of Ok but it is uncessary.
		///
		/// tl;dr: We are checking for weird behavior, but atleast the tested behavior is compliant.
		fn material_as_matrials() {
			let material_l1 = MaterialDescriptor::new_rgb(1., 0., 0.).named("Leg_l1");
			let material_l2 = MaterialDescriptor::new_rgb(0., 1., 0.).named("Leg_l2");
			let geom_leg_l1 = BoxGeometry::new(2., 3., 1.);
			let geom_leg_l2 = CylinderGeometry::new(1., 10.);

			let kinematic_data_tree = KinematicDataTree::new(
				Link::builder("Leg_[L1]_l1")
					.add_visual(
						Visual::builder(geom_leg_l1.clone())
							.transformed(Transform::new_translation(0., 1.5, 0.))
							.named("Leg_[L1]_l1_vis_1")
							.materialized(material_l1.clone()),
					)
					.add_collider(
						Collision::builder(geom_leg_l1.clone())
							.transformed(Transform::new_translation(0., 1.5, 0.))
							.named("Leg_[L1]_l1_col_1"),
					),
			);

			kinematic_data_tree
				.root_link
				.try_write()
				.unwrap()
				.try_attach_child(
					SmartJointBuilder::new_fixed("Leg_[L1]_j1").add_transform(Transform::new(
						(0., 3., 0.),
						(0., 0., std::f32::consts::FRAC_PI_2),
					)),
					Link::builder("Leg_[L1]_l2")
						.add_visual(
							Visual::builder(geom_leg_l2.clone())
								.transformed(Transform::new(
									(0., 5., 0.),
									(std::f32::consts::FRAC_PI_2, 0., 0.),
								))
								.named("Leg_[L1]_l2_vis_1")
								.materialized(material_l2.clone()),
						)
						.add_collider(
							Collision::builder(geom_leg_l2.clone())
								.transformed(Transform::new(
									(0., 5., 0.),
									(std::f32::consts::FRAC_PI_2, 0., 0.),
								))
								.named("Leg_[L1]_l2_col_1"),
						),
				)
				.unwrap();

			test_to_urdf_kinematic_data_tree(
				Arc::clone(&kinematic_data_tree),
				format!(
					r#"<material name="Leg_l1">
	<color rgba="1 0 0 1"/>
</material>
<material name="Leg_l2">
	<color rgba="0 1 0 1"/>
</material>
<link name="Leg_[L1]_l1">
	<visual name="Leg_[L1]_l1_vis_1">
		<origin xyz="0 1.5 0"/>
		<geometry>
			<box size="2 3 1"/>
		</geometry>
		<material name="Leg_l1"/>
	</visual>
	<collision name="Leg_[L1]_l1_col_1">
		<origin xyz="0 1.5 0"/>
		<geometry>
			<box size="2 3 1"/>
		</geometry>
	</collision>
</link>
<joint name="Leg_[L1]_j1" type="fixed">
	<origin xyz="0 3 0" rpy="0 0 {}"/>
	<parent link="Leg_[L1]_l1"/>
	<child link="Leg_[L1]_l2"/>
</joint>
<link name="Leg_[L1]_l2">
	<visual name="Leg_[L1]_l2_vis_1">
		<origin xyz="0 5 0" rpy="{} 0 0"/>
		<geometry>
			<cylinder radius="1" length="10"/>
		</geometry>
		<material name="Leg_l2"/>
	</visual>
	<collision name="Leg_[L1]_l2_col_1">
		<origin xyz="0 5 0" rpy="{} 0 0"/>
		<geometry>
			<cylinder radius="1" length="10"/>
		</geometry>
	</collision>
</link>"#,
					std::f32::consts::FRAC_PI_2,
					std::f32::consts::FRAC_PI_2,
					std::f32::consts::FRAC_PI_2,
				),
				&URDFConfig {
					xml_mode: XMLMode::Indent('\t', 1),
					..Default::default()
				},
			);

			// Test full material
			test_to_urdf_kinematic_data_tree(
				kinematic_data_tree,
				format!(
					r#"<link name="Leg_[L1]_l1">
	<visual name="Leg_[L1]_l1_vis_1">
		<origin xyz="0 1.5 0"/>
		<geometry>
			<box size="2 3 1"/>
		</geometry>
		<material name="Leg_l1">
			<color rgba="1 0 0 1"/>
		</material>
	</visual>
	<collision name="Leg_[L1]_l1_col_1">
		<origin xyz="0 1.5 0"/>
		<geometry>
			<box size="2 3 1"/>
		</geometry>
	</collision>
</link>
<joint name="Leg_[L1]_j1" type="fixed">
	<origin xyz="0 3 0" rpy="0 0 {}"/>
	<parent link="Leg_[L1]_l1"/>
	<child link="Leg_[L1]_l2"/>
</joint>
<link name="Leg_[L1]_l2">
	<visual name="Leg_[L1]_l2_vis_1">
		<origin xyz="0 5 0" rpy="{} 0 0"/>
		<geometry>
			<cylinder radius="1" length="10"/>
		</geometry>
		<material name="Leg_l2">
			<color rgba="0 1 0 1"/>
		</material>
	</visual>
	<collision name="Leg_[L1]_l2_col_1">
		<origin xyz="0 5 0" rpy="{} 0 0"/>
		<geometry>
			<cylinder radius="1" length="10"/>
		</geometry>
	</collision>
</link>"#,
					std::f32::consts::FRAC_PI_2,
					std::f32::consts::FRAC_PI_2,
					std::f32::consts::FRAC_PI_2,
				),
				&URDFConfig {
					xml_mode: XMLMode::Indent('\t', 1),
					material_references: URDFMaterialReferences::AlwaysInline,
					..Default::default()
				},
			)
		}

		#[test]
		#[ignore = "TODO"]
		fn to_urdf_2() {
			todo!("MORE TEST")
		}
	}
}
