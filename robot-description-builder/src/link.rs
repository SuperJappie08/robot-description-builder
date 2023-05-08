pub mod builder;
mod collision;
mod geometry;
pub mod helper_functions;
mod inertial;
mod link_parent;
mod link_shape_data;
mod visual;

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

	/// TODO: Depricate or Implement?
	#[derive(Debug, PartialEq, Eq, Clone)]
	pub enum ConnectionPoint {
		/// Point at Link connection point (Link Origin without translation)
		Begin,
		CenterOfVolume,
		CenterOfMass,
		End,
	}
}

use std::sync::{Arc, Weak};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::{
	chained::Chained,
	cluster_objects::{
		kinematic_data_errors::AddJointError, kinematic_data_tree::KinematicDataTree,
	},
	identifiers::GroupID,
	joint::{BuildJoint, BuildJointChain, Joint, JointBuilder},
	link::{
		builder::LinkBuilder, collision::Collision, inertial::InertialData,
		link_parent::LinkParent, visual::Visual,
	},
	transform::Transform,
	ArcLock, WeakLock,
};

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
	/// Or calculate when necessary
	end_point: Option<(f32, f32, f32)>,
	me: WeakLock<Self>,
}

impl Link {
	pub fn builder(name: impl Into<String>) -> LinkBuilder {
		LinkBuilder::new(name)
	}

	/// Gets a (strong) refence to the current [`Link`]. (An `Arc<RwLock<Link>>`)
	pub fn get_self(&self) -> ArcLock<Link> {
		// Unwrapping is Ok here, because if the Link exists, its self refence should exist.
		Weak::upgrade(&self.me).unwrap()
	}

	/// Gets a weak refence to the current [`Link`]. (An `Weak<RwLock<Link>>`)
	pub fn get_weak_self(&self) -> WeakLock<Link> {
		Weak::clone(&self.me)
	}

	pub fn parent(&self) -> &LinkParent {
		&self.direct_parent
	}

	/// Gets the reference to the name of the `Link`
	///
	/// # Example
	/// ```rust
	/// # use robot_description_builder::{KinematicInterface, linkbuilding::LinkBuilder};
	/// let tree = LinkBuilder::new("my-link").build_tree();
	///
	/// assert_eq!(tree.get_root_link().try_read().unwrap().name(), "my-link")
	/// ```
	pub fn name(&self) -> &String {
		&self.name
	}

	/// Returns a reference to the joints of this [`Link`].
	///
	/// The vector contains all [`Joint`]s connected to this [`Link`], wrapped in a `Arc<RwLock<T>>`.
	pub fn joints(&self) -> &Vec<ArcLock<Joint>> {
		&self.child_joints
	}

	/// Returns a mutable reference to joints `Vec` of this [`Link`].
	///
	/// The vector contains all [`Joint`]s connected to this [`Link`], wrapped in a `Arc<RwLock<T>>`.
	pub(crate) fn joints_mut(&mut self) -> &mut Vec<ArcLock<Joint>> {
		&mut self.child_joints
	}

	/// TODO: DOC
	/// TODO: maybe flip arguments, because then it will be Link ( Joint, Link)
	///
	/// # DEFINED BEHAVIOR:
	///  - The newest link get transfered from the child tree. TODO: VERIFY
	///
	/// ## TODO:
	///  - DOC
	///  - Test
	///  - Doctest
	pub fn try_attach_child<LinkChain>(
		&mut self,
		link_chain: LinkChain,
		joint_builder: impl BuildJoint,
	) -> Result<(), AddJointError>
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
		transform: Transform,
	) -> Result<(), AddJointError> {
		joint_chain.0.with_origin(transform);

		self.attach_joint_chain(joint_chain)
	}

	/// Attach a `Chained<JointBuilder>` to the position set in the root `JointBuilder`
	///
	/// ## TODO:
	///  - Test
	///  - Doctest
	pub fn attach_joint_chain(
		&mut self,
		joint_chain: Chained<JointBuilder>,
	) -> Result<(), AddJointError> {
		let joint =
			joint_chain.build_chain(&self.tree, &self.get_weak_self(), self.get_shape_data());

		self.tree
			.upgrade()
			.expect("KinematicDataTree should be initialized")
			.try_add_joint(&joint)?;

		self.joints_mut().push(joint);
		Ok(())
	}

	pub fn inertial(&self) -> Option<&InertialData> {
		self.inertial.as_ref()
	}

	pub fn get_end_point(&self) -> Option<(f32, f32, f32)> {
		self.end_point
	}

	pub fn visuals(&self) -> &Vec<Visual> {
		&self.visuals
	}

	pub(crate) fn visuals_mut(&mut self) -> &mut Vec<Visual> {
		&mut self.visuals
	}

	pub fn colliders(&self) -> &Vec<Collision> {
		&self.colliders
	}

	/// TODO: EXPAND
	pub fn rebuild(&self) -> LinkBuilder {
		LinkBuilder {
			name: self.name.clone(),
			visual_builders: self.visuals.iter().map(|visual| visual.rebuild()).collect(),
			colliders: self
				.colliders
				.iter()
				.map(|collision| collision.rebuild())
				.collect(),
			intertial: self.inertial.clone(),
			..Default::default()
		}
	}

	/// Rebuilds everything below this aswell
	///
	/// TODO: FINISH
	pub(crate) fn rebuild_branch_continued(&self) -> LinkBuilder {
		LinkBuilder {
			joints: self
				.child_joints
				.iter()
				.map(|joint| joint.read().unwrap().rebuild_branch_continued()) // FIXME: Figure out if unwrap is Ok here?
				.collect(),
			..self.rebuild()
		}
	}

	/// TODO: DOCS:
	/// TODO: TEST
	pub fn rebuild_branch(&self) -> Chained<LinkBuilder> {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "LinkBuilder","Starting Branch Rebuilding: {}", self.name());
		Chained(self.rebuild_branch_continued())
	}

	/// TODO: DOCS:
	/// TODO: TEST
	pub(crate) fn yank(&self) -> LinkBuilder {
		let builder = self.rebuild_branch_continued();

		match self.parent() {
			LinkParent::Joint(joint) => {
				let joint = joint.upgrade().unwrap();
				let parent_link = &joint.try_read().unwrap().parent_link; // FIXME: Is unwrap Ok here?

				*self.tree.upgrade().unwrap().newest_link.write().unwrap() =
					Weak::clone(parent_link);

				parent_link
					.upgrade()
					.unwrap() // This unwrap is Ok since the parent_link on a Joint is initialized while adding to the tree.
					.try_write()
					.unwrap() //FIXME: Is unwrap Ok here?
					.joints_mut()
					.retain(|other_joint| !Arc::ptr_eq(&joint, other_joint));
			}
			LinkParent::KinematicTree(_) => {
				#[cfg(any(feature = "logging", test))]
				log::trace!("The tree should be dropped, but how?")
			}
		}

		builder
	}

	pub(crate) fn get_shape_data(&self) -> LinkShapeData {
		LinkShapeData::new(
			self.visuals()
				.iter()
				.map(|visual| visual.get_geometry_data()),
		)
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
			value: self.name().display().as_bytes().into(),
		});
		element.write_inner_content(|writer| -> Result<(), quick_xml::Error> {
			if let Some(inertial_data) = self.inertial() {
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
			self.joints()
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
		assert_eq!(newest_link.try_read().unwrap().name(), root_link.name());
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

		assert_eq!(tree.get_root_link().try_read().unwrap().name(), "base_link");
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().name(),
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
				.parent_link()
				.try_read()
				.unwrap()
				.name(),
			"base_link"
		);
		assert_eq!(
			tree.get_joint("steve")
				.unwrap()
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.name(),
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
			tree.get_newest_link().try_read().unwrap().name(),
			"other_child_link"
		);

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(tree_three, JointBuilder::new("joint-3", JointType::Fixed))
			.unwrap();

		assert_eq!(tree.get_root_link().try_read().unwrap().name(), "root");
		assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "3");

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
			root_link.child_joints[0].try_read().unwrap().name(),
			"initial_joint"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.name(),
			"other_root"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()
				.len(),
			1
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()[0]
				.try_read()
				.unwrap()
				.name(),
			"other_joint"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()[0]
				.try_read()
				.unwrap()
				.child_link()
				.read()
				.unwrap()
				.name(),
			"other_child_link"
		);
		assert_eq!(
			root_link.child_joints[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()[0]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()
				.len(),
			0
		);

		assert_eq!(
			root_link.child_joints[1].try_read().unwrap().name(),
			"joint-3"
		);
		assert_eq!(
			root_link.child_joints[1]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.name(),
			"3"
		);
		assert_eq!(
			root_link.child_joints[1]
				.try_read()
				.unwrap()
				.child_link()
				.try_read()
				.unwrap()
				.joints()
				.len(),
			0
		);
	}
}
