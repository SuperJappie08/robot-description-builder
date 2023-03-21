mod collision;
mod geometry;
pub mod helper_functions;
mod inertial;
mod link_parent;
mod visual;

#[cfg(feature = "xml")]
use itertools::process_results;
#[cfg(feature = "logging")]
use log::{info, log_enabled, Level};
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
	cluster_objects::{
		kinematic_data_errors::{AddJointError, AddLinkError, AddMaterialError},
		kinematic_tree_data::KinematicTreeData,
		KinematicInterface, KinematicTree,
	},
	joint::{BuildJoint, JointInterface},
	link::collision::Collision,
	link::inertial::InertialData,
	link::link_parent::LinkParent,
	link::visual::Visual,
	ArcLock, WeakLock,
};

// pub trait LinkTrait: Debug {
// 	/// Returns the parent of the `Link` wrapped in a optional.
// 	fn get_parent(&self) -> Option<LinkParent>;
// 	fn set_parent(&mut self, parent: LinkParent);

// 	/// Returns the name of the `Link`
// 	fn get_name(&self) -> String; // TODO: This might be temp because I want dynamic names.

// 	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>>; // TODO: Not final?
// 	fn try_attach_child(
// 		&mut self,
// 		tree: Box<dyn KinematicInterface>,
// 		joint_name: String,
// 		_joint_type: JointType,
// 	) -> Result<(), String>;

// 	// fn get_visual(&self) -> Vec<()>;
// 	// fn get_colliders(&self) -> Vec<()>;

// 	fn add_visual(&mut self, visual: Visual) -> Self;
// 	fn add_collider(&mut self, Collider: Collision) -> Self;
// }


#[derive(Debug)]
pub struct Link {
	pub(crate) name: String,
	pub(crate) tree: WeakLock<KinematicTreeData>,
	direct_parent: Option<link_data::LinkParent>,
	child_joints: Vec<ArcLock<Box<dyn JointInterface + Sync + Send>>>,
	inertial: Option<InertialData>,
	visuals: Vec<link_data::Visual>,
	colliders: Vec<link_data::Collision>,
	/// TODO: Maybe array, or thing
	end_point: Option<(f32, f32, f32)>,
}

impl Link {
	/// NOTE: Maybe change name to `Impl Into<String>` or a `&str`, for ease of use?
	pub fn new(name: String) -> KinematicTree {
		#[cfg(feature = "logging")]
		info!("Making a Link[name = \"{}\"", name);

		let link = Link {
			name,
			tree: Weak::new(),
			direct_parent: None,
			child_joints: Vec::new(),
			inertial: None,
			visuals: Vec::new(),
			colliders: Vec::new(),
			end_point: None
		};

		let tree = KinematicTreeData::new_link(link);

		KinematicTree::new(tree)
	}
	// }
	//
	// impl LinkTrait for Link {
	pub fn get_parent(&self) -> Option<LinkParent> {
		self.direct_parent.clone()
	}

	pub(crate) fn set_parent(&mut self, parent: LinkParent) {
		self.direct_parent = Some(parent);
		// NO-FIXME: Add yourself to registry.
		// Maybe that has already happend tho? -> You can't because of the Rc Pointer thing
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}

	pub fn get_joints(&self) -> Vec<ArcLock<Box<dyn JointInterface + Sync + Send>>> {
		self.child_joints.iter().map(Arc::clone).collect()
	}

	/// Maybe rename to try attach child
	/// DEFINED BEHAVIOR:
	///  - The newest link get transfered from the child tree.
	pub fn try_attach_child(
		&mut self,
		tree: Box<dyn KinematicInterface>,
		joint_builder: impl BuildJoint,
	) -> Result<(), AttachChildError> {
		// Generics dont workt that well Rc<RefCell<T>>  where T: KinematicInterface
		//Box<Rc<RefCell<dyn KinematicInterface>>>
		// TODO: NEEDS TO DO SOMETHING WITH JOINT TYPE
		let parent_link = self
			.tree
			.upgrade()
			.unwrap()
			.read()?
			.links
			.read()?
			.get(self.name.as_str())
			.map(Weak::clone)
			.unwrap();

		let joint = Arc::new(RwLock::new(joint_builder.build(
			Weak::clone(&self.tree),
			parent_link,
			tree.get_root_link(),
		)));

		self.child_joints.push(Arc::clone(&joint));

		{
			tree.get_root_link().write()?.direct_parent =
				Some(LinkParent::Joint(Arc::downgrade(&joint)))
		}

		// Maybe I can just go down the tree and add everything by hand for now? It sounds like a terrible Idea, let's do it!

		let parent_tree = self.tree.upgrade().unwrap();
		{
			let mut parent_tree = parent_tree.write()?; // FIXME: Probably shouldn't unwrap
			parent_tree.try_add_link(tree.get_root_link())?;
			parent_tree.try_add_joint(joint)?;
		}
		{
			tree.get_root_link().write()?.add_to_tree(&parent_tree);
		}

		Ok(())
		// Ok(self.tree.upgrade().unwrap())
	}

	pub(crate) fn add_to_tree(&mut self, new_parent_tree: &ArcLock<KinematicTreeData>) {
		{
			let mut new_ptree = new_parent_tree.write().unwrap(); // FIXME: Probably shouldn't unwrap
			self.child_joints
				.iter()
				.for_each(|joint| new_ptree.try_add_joint(Arc::clone(joint)).unwrap());
			// TODO: Add materials, and other stuff
			// The Material Copying might get complex, because I depend on the Ref_Count for determining how to display it.
		}
		self.child_joints.iter().for_each(|joint| {
			joint.write().unwrap().add_to_tree(new_parent_tree); // FIXME: Probably shouldn't unwrap
		});
		self.tree = Arc::downgrade(new_parent_tree);
	}

	pub fn add_visual(&mut self, visual: Visual) -> &mut Self {
		self.try_add_visual(visual).unwrap()
	}

	pub fn try_add_visual(&mut self, visual: Visual) -> Result<&mut Self, AddVisualError> {
		if visual.material.is_some() {
			let binding = self.tree.upgrade().unwrap();
			let mut tree = binding.write()?;
			let result = tree.try_add_material(Arc::clone(visual.material.as_ref().unwrap()));
			if let Err(material_error) = result {
				match material_error {
					AddMaterialError::NoName =>
					{
						#[cfg(feature = "logging")]
						if log_enabled!(Level::Info) {
							info!("An attempt was made to add a material, without a `name`. So Moving on!")
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

	pub fn get_inertial(&self) -> &Option<InertialData> {
		&self.inertial
	}

	pub fn get_end_point(&self) -> Option<(f32, f32, f32)> {
		self.end_point
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
			value: self.name.clone().as_bytes().into(),
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
		self.name == other.name
			&& self.direct_parent == other.direct_parent
			// && self.child_joints == other.child_joints // FIXME: Fix this
			&& self.tree.ptr_eq(&other.tree)
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

impl From<PoisonError<RwLockReadGuard<'_, KinematicTreeData>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, KinematicTreeData>>) -> Self {
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

impl From<PoisonError<RwLockWriteGuard<'_, KinematicTreeData>>> for AttachChildError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, KinematicTreeData>>) -> Self {
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

impl From<PoisonError<RwLockWriteGuard<'_, KinematicTreeData>>> for AddVisualError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, KinematicTreeData>>) -> Self {
		Self::WriteKinemeticData
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{Arc, Weak};

	use crate::{
		cluster_objects::KinematicInterface,
		joint::{JointBuilder, JointType},
		link::{link_parent::LinkParent, Link},
	};

	#[test]
	fn new() {
		let tree = Link::new("Link-on-Park".into());

		let binding = tree.get_root_link();
		let root_link = binding.try_read().unwrap();
		assert_eq!(root_link.name, "Link-on-Park".to_string());

		assert!(root_link.direct_parent.is_some());
		assert!({
			match root_link.direct_parent {
				Some(LinkParent::KinematicTree(_)) => true,
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
		let tree = Link::new("base_link".into());

		assert_eq!(
			tree.get_newest_link()
				.try_write()
				.unwrap()
				.try_attach_child(
					Link::new("child_link".into()).into(),
					JointBuilder::new("steve".into(), JointType::Fixed)
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
			Some(LinkParent::Joint(weak_joint))
		);
	}

	#[test]
	fn try_attach_multi_child() {
		let tree = Link::new("root".into());
		let other_tree = Link::new("other_root".into());
		let tree_three = Link::new("3".into());

		other_tree
			.get_newest_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::new("other_child_link".into()).into(),
				JointBuilder::new("other_joint".into(), JointType::Fixed),
			)
			.unwrap();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				other_tree.into(),
				JointBuilder::new("initial_joint".into(), JointType::Fixed),
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
			.try_attach_child(
				tree_three.into(),
				JointBuilder::new("joint-3".into(), JointType::Fixed),
			)
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
			Some(LinkParent::KinematicTree(Weak::clone(&root_link.tree)))
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
