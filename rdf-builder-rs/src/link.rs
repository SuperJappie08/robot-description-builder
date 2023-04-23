pub mod builder;
mod collision;
mod geometry;
pub mod helper_functions;
mod inertial;
mod link_parent;
mod link_shape_data;
mod visual;
// mod linkbuilder;

pub(crate) use link_shape_data::LinkShapeData;

#[cfg(feature = "xml")]
use itertools::process_results;

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

pub mod link_data {
	pub use crate::link::collision::Collision;
	pub use crate::link::inertial::InertialData;
	pub use crate::link::link_parent::LinkParent;
	pub use crate::link::visual::Visual;
	pub mod geometry {
		pub use crate::link::geometry::*;
	}

	#[derive(Debug, PartialEq, Eq, Clone)]
	pub enum ConnectionPoint {
		/// Point at Link connection point (Link Origin without translation)
		Begin,
		CenterOfVolume,
		CenterOfMass,
		End,
	}
}

use thiserror::Error;

use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::{
	chained::Chained,
	cluster_objects::{
		kinematic_data_errors::{AddJointError, AddLinkError, AddMaterialError},
		kinematic_data_tree::KinematicDataTree,
		KinematicInterface,
	},
	joint::{BuildJoint, BuildJointChain, Joint},
	link::collision::Collision,
	link::inertial::InertialData,
	link::link_parent::LinkParent,
	link::visual::Visual,
	ArcLock, JointBuilder, TransformData, WeakLock,
};

use self::builder::{BuildLink, LinkBuilder};

/// TODO: Make Builder For Link
#[derive(Debug)]
pub struct Link {
	pub(crate) name: String,
	pub(crate) tree: Weak<KinematicDataTree>,
	direct_parent: link_data::LinkParent,
	child_joints: Vec<ArcLock<Joint>>,
	inertial: Option<InertialData>,
	visuals: Vec<link_data::Visual>,
	colliders: Vec<link_data::Collision>,
	/// TODO: Maybe array, or thing
	end_point: Option<(f32, f32, f32)>,
	me: Weak<RwLock<Self>>,
}

impl Link {
	pub fn builder<Name: Into<String>>(name: Name) -> LinkBuilder {
		LinkBuilder::new(name)
	}

	/// Gets a (strong) refence to the current `Link`. (An `Arc<RwLock<Link>>`)
	pub fn get_self(&self) -> ArcLock<Link> {
		// Unwrapping is Ok here, because if the Link exists, its self refence should exist.
		Weak::upgrade(&self.me).unwrap()
	}

	/// Gets a weak refence to the current `Link`. (An `Weak<RwLock<Link>>`)
	pub fn get_weak_self(&self) -> WeakLock<Link> {
		Weak::clone(&self.me)
	}

	pub fn get_parent(&self) -> &LinkParent {
		&self.direct_parent
	}

	/// Gets the reference to the name of the `Link`
	///
	/// # Example
	/// ```rust
	/// # use rdf_builder_rs::{KinematicInterface, linkbuilding::LinkBuilder};
	/// let tree = LinkBuilder::new("my-link").build_tree();
	///
	/// assert_eq!(tree.get_root_link().try_read().unwrap().get_name(), "my-link")
	/// ```
	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_joints(&self) -> &Vec<ArcLock<Joint>> {
		&self.child_joints
		// self.child_joints.iter().map(Arc::clone).collect()
	}

	pub(crate) fn get_joints_mut(&mut self) -> &mut Vec<ArcLock<Joint>> {
		&mut self.child_joints
	}

	/// TODO: DOC
	///
	/// # DEFINED BEHAVIOR:
	///  - The newest link get transfered from the child tree.
	///
	/// TODO: Consider implemeting in reverse, by adding a making a Chained<JointBuilder> and then attaching that
	pub fn try_attach_child_old(
		&mut self,
		old_tree: Box<dyn KinematicInterface>,
		joint_builder: impl BuildJoint,
	) -> Result<(), AttachChildError> {
		// Yanking and rebuilding is less effiecent than what was done before, however this is easier to maintain

		let root_name = old_tree.get_root_link().read().unwrap().get_name().clone(); // FIXME: UNWRAP
		let link_builder = old_tree.yank_link(&root_name).unwrap(); // FIXME: UNWRAP

		let tree = Weak::clone(&self.tree);

		let child_link = link_builder.start_building_chain(&tree);
		let weak_self = self.get_weak_self();

		let shape_data = self.get_shape_data();

		self.get_joints_mut()
			.push(joint_builder.build(tree, weak_self, child_link, shape_data));

		self.tree
			.upgrade()
			.unwrap()
			.try_add_joint(self.get_joints().last().unwrap())?; // FIXME: unwrap not OK

		self.tree
			.upgrade()
			.expect("KinematicDataTree should be initialized")
			.newest_link
			.read()
			.unwrap() // FIXME: is unwrap OK?
			.upgrade()
			.unwrap() // FIXME: is unwrap OK?
			.write()
			.unwrap() // FIXME: is unwrap OK?
			.direct_parent = LinkParent::Joint(Arc::downgrade(self.get_joints().last().unwrap()));

		Ok(())
	}

	///
	/// ## TODO:
	///  - DOC
	///  - Test
	///  - Doctest
	pub fn try_attach_child<LinkChain>(
		&mut self,
		link_chain: LinkChain,
		joint_builder: impl BuildJoint,
	) -> Result<(), AttachChildError>
	where
		LinkChain: Into<Chained<LinkBuilder>>,
	{
		self.attach_joint_chain(Into::<Chained<JointBuilder>>::into((
			joint_builder,
			link_chain.into(),
		)))
	}

	/// TODO: This is not finalized yet
	///
	/// ## TODO:
	///  - DOC
	///  - Test
	///  - Doctest
	pub fn attach_joint_chain_at(
		&mut self,
		mut joint_chain: Chained<JointBuilder>,
		transform: TransformData,
	) -> Result<(), AttachChildError> {
		joint_chain.0.with_origin(transform);

		self.attach_joint_chain(joint_chain)
	}

	/// Attach a `Chained<JointBuilder>` to the position set in the root `JointBuilder`
	///
	/// ## TODO:
	///  - Implement
	///  - Test
	///  - Doctest
	pub fn attach_joint_chain(
		&mut self,
		joint_chain: Chained<JointBuilder>,
	) -> Result<(), AttachChildError> {
		let joint =
			joint_chain.build_chain(&self.tree, &self.get_weak_self(), self.get_shape_data());

		self.get_joints_mut().push(joint);

		self.tree
			.upgrade()
			.expect("KinematicDataTree should be initialized")
			.try_add_joint(self.get_joints().last().unwrap())?;

		Ok(())
	}

	pub fn add_visual(&mut self, visual: Visual) -> &mut Self {
		self.try_add_visual(visual).unwrap()
	}

	pub fn try_add_visual(&mut self, mut visual: Visual) -> Result<&mut Self, AddVisualError> {
		if visual.material.is_some() {
			let binding = self.tree.upgrade().unwrap();
			let result = binding.try_add_material(visual.get_material_mut().unwrap()); // FIXME: UNWRAP?
			if let Err(material_error) = result {
				match material_error {
					AddMaterialError::NoName =>
					{
						#[cfg(any(feature = "logging", test))]
						if log::log_enabled!(log::Level::Info) {
							log::info!("An attempt was made to add a material, without a `name`. So Moving on!")
						}
					}
					_ => Err(material_error)?,
				}
			}
		}

		self.visuals.push(visual);
		Ok(self)
	}

	/// TODO:NOTE: Originally returned self for chaining, dont now if that is neccessary? so removed for now
	pub fn add_collider(&mut self, collider: Collision) -> &mut Self {
		self.colliders.push(collider);
		self
	}

	pub fn get_inertial(&self) -> Option<&InertialData> {
		self.inertial.as_ref()
	}

	pub fn get_end_point(&self) -> Option<(f32, f32, f32)> {
		self.end_point
	}

	pub fn get_visuals(&self) -> &Vec<Visual> {
		&self.visuals
	}

	pub(crate) fn get_visuals_mut(&mut self) -> &mut Vec<Visual> {
		&mut self.visuals
	}

	pub fn get_colliders(&self) -> &Vec<Collision> {
		&self.colliders
	}

	/// TODO: EXPAND
	pub fn rebuild(&self) -> LinkBuilder {
		LinkBuilder {
			name: self.name.clone(),
			visual_builders: self.visuals.iter().map(|visual| visual.rebuild()).collect(),
			colliders: self.colliders.to_vec(),
			..Default::default()
		}
	}

	/// Rebuilds everything below this aswell
	///
	/// TODO: FINISH
	pub fn rebuild_branch(&self) -> LinkBuilder {
		LinkBuilder {
			joints: self
				.child_joints
				.iter()
				.map(|joint| joint.read().unwrap().rebuild_branch()) // FIXME: Figure out if unwrap is Ok here?
				.collect(),
			..self.rebuild()
		}
	}

	/// TODO: DOCS:
	/// TODO: TEST
	pub(crate) fn yank(&self) -> LinkBuilder {
		let builder = self.rebuild_branch();

		match self.get_parent() {
			LinkParent::Joint(joint) => {
				let joint = joint.upgrade().unwrap();
				joint
					.try_read()
					.unwrap() // FIXME: Is unwrap Ok here?
					.get_parent_link()
					.try_write()
					.unwrap() //FIXME: Is unwrap Ok here?
					.get_joints_mut()
					.retain(|other_joint| Arc::ptr_eq(&joint, other_joint));
			}
			LinkParent::KinematicTree(_) => {
				#[cfg(any(feature = "logging", test))]
				log::trace!("The tree should be dropped, but how?")
			}
		}

		builder
	}

	fn get_shape_data(&self) -> LinkShapeData {
		// FIXME: FINISH THIS
		// THIS SHOULD CONTAIN A BOUNDING BOX AND STUFF
		LinkShapeData::Box
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Link {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("link").with_attribute(Attribute {
			key: QName(b"name"),
			value: self.get_name().as_bytes().into(),
		});
		element.write_inner_content(|writer| -> Result<(), quick_xml::Error> {
			if let Some(inertial_data) = self.get_inertial() {
				inertial_data.to_urdf(writer, urdf_config)?;
			}

			process_results(
				self.visuals
					.iter()
					.map(|visual| visual.to_urdf(writer, urdf_config)),
				|iter| iter.collect(),
			)?;

			process_results(
				self.colliders
					.iter()
					.map(|collider| collider.to_urdf(writer, urdf_config)),
				|iter| iter.collect(),
			)?;

			Ok(())
		})?;

		// Write joints
		process_results(
			self.get_joints()
				.iter()
				.map(|joint| joint.read().unwrap().to_urdf(writer, urdf_config)),
			|iter| iter.collect(),
		)?;

		Ok(())
	}
}

impl PartialEq for Link {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.me, &other.me)
			&& self.name == other.name
			&& self.direct_parent == other.direct_parent
			&& self.tree.ptr_eq(&other.tree)
			&& self.inertial == other.inertial
			&& self.visuals.len() == other.visuals.len()
			&& self.colliders.len() == other.colliders.len()
			&& self.child_joints.len() == other.child_joints.len()
			&& self
				.visuals
				.iter()
				.zip(other.visuals.iter())
				.all(|(own_visual, other_visual)| own_visual == other_visual)
			&& self
				.colliders
				.iter()
				.zip(other.colliders.iter())
				.all(|(own_collider, other_collider)| own_collider == other_collider)
			&& self
				.child_joints
				.iter()
				.zip(other.child_joints.iter())
				.all(|(own_joint, other_joint)| Arc::ptr_eq(own_joint, other_joint))
	}
}

#[derive(Debug, PartialEq, Error)]
pub enum AttachChildError {
	#[error(transparent)]
	AddLink(#[from] AddLinkError),
	#[error(transparent)]
	AddJoint(#[from] AddJointError),
	#[error("Read Tree Error")]
	ReadTree, //(PoisonError<RwLockReadGuard<'a, KinematicTreeData>>),
	#[error("Read LinkIndex Error")]
	ReadLinkIndex, //(PoisonError<RwLockReadGuard<'a, HashMap<String, WeakLock<Link>>>>),
	#[error("Write Link Error")]
	WriteLink,
	#[error("Write Tree Error")]
	WriteTree,
}

impl From<PoisonError<RwLockReadGuard<'_, KinematicDataTree>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, KinematicDataTree>>) -> Self {
		Self::ReadTree //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, WeakLock<Link>>>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, HashMap<String, WeakLock<Link>>>>) -> Self {
		Self::ReadLinkIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, Link>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, Link>>) -> Self {
		Self::WriteLink
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, KinematicDataTree>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, KinematicDataTree>>) -> Self {
		Self::WriteTree
	}
}

#[derive(Debug, Error)]
pub enum AddVisualError {
	#[error(transparent)]
	AddMaterial(#[from] AddMaterialError),
	#[error("Write KinematicTreeData Error")]
	WriteKinemeticData,
}

impl From<PoisonError<RwLockWriteGuard<'_, KinematicDataTree>>> for AddVisualError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, KinematicDataTree>>) -> Self {
		Self::WriteKinemeticData
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Weak};
	use test_log::test;

	use crate::{
		cluster_objects::KinematicInterface,
		joint::{JointBuilder, JointType},
		link::{builder::LinkBuilder, link_parent::LinkParent, Link},
	};

	#[test]
	fn new() {
		let tree = LinkBuilder::new("Link-on-Park").build_tree();

		let binding = tree.get_root_link();
		let root_link = binding.try_read().unwrap();
		assert_eq!(root_link.name, "Link-on-Park".to_string());

		assert!(root_link.direct_parent.is_valid_reference());
		assert!({
			match root_link.direct_parent {
				LinkParent::KinematicTree(_) => true,
				_ => false,
			}
		});

		let newest_link = tree.get_newest_link();
		assert_eq!(
			newest_link.try_read().unwrap().get_name(),
			root_link.get_name()
		);
		assert!(Arc::ptr_eq(&newest_link, &binding));

		assert_eq!(tree.get_links().try_read().unwrap().len(), 1);
		assert_eq!(tree.get_joints().try_read().unwrap().len(), 0);
	}

	#[test]
	fn try_attach_single_child() {
		let tree = LinkBuilder::new("base_link").build_tree();

		assert_eq!(
			tree.get_newest_link()
				.try_write()
				.unwrap()
				.try_attach_child(
					LinkBuilder::new("child_link"),
					JointBuilder::new("steve", JointType::Fixed)
				),
			Ok(())
		);

		assert_eq!(
			tree.get_root_link().try_read().unwrap().get_name(),
			"base_link"
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().get_name(),
			"child_link"
		);

		assert!(tree
			.get_links()
			.try_read()
			.unwrap()
			.contains_key("base_link"));
		assert!(tree
			.get_links()
			.try_read()
			.unwrap()
			.contains_key("child_link"));
		assert!(tree.get_joints().try_read().unwrap().contains_key("steve"));

		assert_eq!(
			tree.get_joint("steve")
				.unwrap()
				.try_read()
				.unwrap()
				.get_parent_link()
				.try_read()
				.unwrap()
				.get_name(),
			"base_link"
		);
		assert_eq!(
			tree.get_joint("steve")
				.unwrap()
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_name(),
			"child_link"
		);

		let weak_joint =
			{ Weak::clone(tree.get_joints().try_read().unwrap().get("steve").unwrap()) };
		assert_eq!(
			tree.get_link("child_link")
				.unwrap()
				.try_read()
				.unwrap()
				.direct_parent,
			LinkParent::Joint(weak_joint)
		);
	}

	#[test]
	fn try_attach_multi_child() {
		let tree = Link::builder("root").build_tree();
		let other_tree = Link::builder("other_root").build_tree();
		let tree_three = Link::builder("3").build_tree();

		other_tree
			.get_newest_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				LinkBuilder::new("other_child_link"),
				JointBuilder::new("other_joint", JointType::Fixed),
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				other_tree,
				JointBuilder::new("initial_joint", JointType::Fixed),
			)
			.unwrap();

		//TODO: What should be the defined behavior?
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().get_name(),
			"other_child_link"
		);

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(tree_three, JointBuilder::new("joint-3", JointType::Fixed))
			.unwrap();

		assert_eq!(tree.get_root_link().try_read().unwrap().get_name(), "root");
		assert_eq!(tree.get_newest_link().try_read().unwrap().get_name(), "3");

		{
			let binding = tree.get_links();
			let links = binding.try_read().unwrap();
			assert_eq!(links.len(), 4);
			assert!(links.contains_key("root"));
			assert!(links.contains_key("other_root"));
			assert!(links.contains_key("other_child_link"));
			assert!(links.contains_key("3"));
		}

		{
			let binding = tree.get_joints();
			let joints = binding.try_read().unwrap();
			assert_eq!(joints.len(), 3);
			assert!(joints.contains_key("other_joint"));
			assert!(joints.contains_key("initial_joint"));
			assert!(joints.contains_key("joint-3"));
		}

		let binding = tree.get_root_link();
		let root_link = binding.try_read().unwrap();
		assert_eq!(
			root_link.direct_parent,
			LinkParent::KinematicTree(Weak::clone(&root_link.tree))
		);
		assert_eq!(root_link.child_joints.len(), 2);
		assert_eq!(
			root_link.child_joints[0].try_read().unwrap().get_name(),
			"initial_joint"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_name(),
			"other_root"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len(),
			1
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()[0]
				.try_read()
				.unwrap()
				.get_name(),
			"other_joint"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.read()
				.unwrap()
				.get_name(),
			"other_child_link"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()[0]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len(),
			0
		);

		assert_eq!(
			root_link.child_joints[1].try_read().unwrap().get_name(),
			"joint-3"
		);
		assert_eq!(
			root_link.child_joints[1]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_name(),
			"3"
		);
		assert_eq!(
			root_link.child_joints[1]
				.try_read()
				.unwrap()
				.get_child_link()
				.try_read()
				.unwrap()
				.get_joints()
				.len(),
			0
		);
	}
}
