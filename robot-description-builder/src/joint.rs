mod joint_tranform_mode;
mod jointbuilder;

#[cfg(not(feature = "smart-joint-extension"))]
mod smartjointbuilder;
#[cfg(feature = "smart-joint-extension")]
pub mod smartjointbuilder;

/// TODO: Pub(crate) for now
/// pub(crate)
/// PUB FOR NOW FOR DOC TESTS????
pub mod joint_data;

pub use joint_tranform_mode::JointTransformMode;
pub(crate) use jointbuilder::BuildJointChain;
pub use jointbuilder::{BuildJoint, JointBuilder};
pub use smartjointbuilder::SmartJointBuilder;

#[cfg(feature = "xml")]
use std::borrow::Cow;
use std::sync::{Arc, Weak};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::{
	chained::Chained,
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	identifiers::GroupID,
	link::Link,
	transform::Transform,
	utils::{ArcLock, ArcRW, WeakLock},
	yank_errors::{RebuildBranchError, YankJointError},
};

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug)]
pub struct Joint {
	/// The name of the `Joint`
	pub(crate) name: String,
	/// A Reference to the parent Kinematic Tree
	pub(crate) tree: Weak<KinematicDataTree>,
	/// A Reference to the parent `Link`
	pub(crate) parent_link: WeakLock<Link>,
	pub(crate) child_link: ArcLock<Link>, //temp pub TODO: THIS PROBABLY ISN'T THE NICEST WAY TO DO THIS.
	/// The information specific to the JointType: TODO: DECIDE IF THIS SHOULD BE PUBLIC
	pub(crate) joint_type: JointType,
	/// The transform from the origin of the parent `Link` to this `Joint`'s origin.
	///
	/// In URDF this field is refered to as `<origin>`
	transform: Transform,
	axis: Option<(f32, f32, f32)>,
	calibration: joint_data::CalibrationData,
	dynamics: joint_data::DynamicsData,
	limit: Option<joint_data::LimitData>,
	/// TODO: Should be editable
	mimic: Option<joint_data::MimicData>,
	safety_controller: Option<joint_data::SafetyControllerData>,

	me: WeakLock<Joint>,
}

impl Joint {
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn joint_type(&self) -> JointType {
		self.joint_type
	}

	/// Returns a reference to the parent `Link`
	///
	/// TODO: ADD EXAMPLE
	///
	/// For now pub crate, this should maybe go to joint trait
	pub fn parent_link(&self) -> ArcLock<Link> {
		// If this panics, the Joint is not initialized propperly.
		self.parent_link
			.upgrade()
			.expect("Joint's parent link should be set")
	}

	/// For now pub crate, this should maybe go to joint trait
	/// Is this even necessary?
	pub fn child_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.child_link)
	}

	/// FIXME: pub(crate) for now
	pub(crate) fn child_link_ref(&self) -> &ArcLock<Link> {
		&self.child_link
	}

	pub fn transform(&self) -> &Transform {
		&self.transform
	}

	pub fn axis(&self) -> Option<(f32, f32, f32)> {
		// This is fine since it implements Copy
		self.axis
	}

	/// Make a `JointBuilder` to build a 'Clone' of the `Joint`.
	///
	/// This method does not clone the child of the [`Joint`], only the `Joint` is self.
	///
	/// If the whole branch needs to be copied use [`rebuild_branch`](Self::rebuild_branch()).
	pub fn rebuild(&self) -> JointBuilder {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Rebuilding: {}", self.name());
		JointBuilder {
			name: self.name.clone(),
			joint_type: self.joint_type,
			// child is None, since this method only rebuilds the current Joint
			child: None,
			transform: self.transform.into(),
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic.clone().map(|mimic| mimic.into()),
			safety_controller: self.safety_controller,
		}
	}

	pub(crate) fn rebuild_branch_continued(&self) -> Result<JointBuilder, RebuildBranchError> {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Rebuilding: {}", self.name());
		Ok(JointBuilder {
			child: Some(self.child_link.mread()?.rebuild_branch_continued()?),
			..self.rebuild()
		})
	}

	// TODO: DOCS
	pub fn rebuild_branch(&self) -> Result<Chained<JointBuilder>, RebuildBranchError> {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Starting Branch Rebuilding: {}", self.name());
		Ok(Chained(self.rebuild_branch_continued()?))
	}

	/// TODO:Find a way to make these builders special?
	/// - Fix Documentation
	///
	/// Still need to purge
	///
	/// NOTE: you must get the link from the rep by cloning.
	/// TODO: Maybe add a `first` argument to only set the `newest_link` if it is the first in the call stack
	pub(crate) fn yank(&self) -> Result<JointBuilder, YankJointError> {
		let builder = self.rebuild_branch_continued()?;

		#[cfg(any(feature = "logging", test))]
		log::info!("Yanked Joint \"{}\"", self.name());

		self.parent_link()
			.mwrite()?
			.joints_mut()
			.retain(|joint| !Arc::ptr_eq(&self.get_self(), joint));

		// TODO: This is most-likely Ok, however it could error on the write.
		*self.tree.upgrade().unwrap().newest_link.write().unwrap() = Weak::clone(&self.parent_link);

		Ok(builder)
	}

	/// Gets a (strong) refence to the current `Joint`. (An `Arc<RwLock<Joint>>`)
	pub fn get_self(&self) -> ArcLock<Joint> {
		// Unwrapping is Ok here, because if the Joint exists, its self refence should exist.
		Weak::upgrade(&self.me).unwrap()
	}

	/// Gets a weak refence to the current `Joint`. (An `Weak<RwLock<Joint>>`)
	pub fn get_weak_self(&self) -> WeakLock<Joint> {
		Weak::clone(&self.me)
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Joint {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer
			.create_element("joint")
			.with_attribute(Attribute {
				key: QName(b"name"),
				value: self.name().display().as_bytes().into(),
			})
			.with_attribute(Attribute {
				key: QName(b"type"),
				value: self.joint_type().into(),
			});

		element.write_inner_content(|writer| -> quick_xml::Result<()> {
			let transform = self.transform();
			if transform.contains_some() {
				transform.to_urdf(writer, urdf_config)?;
			}

			writer
				.create_element("parent")
				.with_attribute(Attribute {
					key: QName(b"link"),
					value: self
						.parent_link()
						.read()
						.unwrap() // FIXME: Is unwrap Ok HEre?
						.name()
						.display()
						.as_bytes()
						.into(),
				})
				.write_empty()?;

			writer
				.create_element("child")
				.with_attribute(Attribute {
					key: QName(b"link"),
					value: self
						.child_link()
						.read()
						.unwrap() // FIXME: Is unwrap Ok HEre?
						.name()
						.display()
						.as_bytes()
						.into(),
				})
				.write_empty()?;

			if let Some((x, y, z)) = &self.axis {
				// TODO: Fix '<axis xyz="-0 -0 -1"/>' after mirror.
				writer
					.create_element("axis")
					.with_attribute(Attribute {
						key: QName(b"xyz"),
						value: format!("{} {} {}", x, y, z).as_bytes().into(),
					})
					.write_empty()?;
			}

			self.calibration.to_urdf(writer, urdf_config)?;
			self.dynamics.to_urdf(writer, urdf_config)?;

			if let Some(limit) = &self.limit {
				limit.to_urdf(writer, urdf_config)?;
			}

			// TODO: TEST INTEGRATION OF THESE
			if let Some(mimic) = &self.mimic {
				mimic.to_urdf(writer, urdf_config)?;
			}

			if let Some(safety_contoller) = &self.safety_controller {
				safety_contoller.to_urdf(writer, urdf_config)?;
			}

			Ok(())
		})?;

		self.child_link()
			.read()
			.unwrap() // FIXME: Is unwrap Ok HEre?
			.to_urdf(writer, urdf_config)?;
		Ok(())
	}
}

/// TODO: Maybe remove some fields from check, since it will always match if name and tree are true
impl PartialEq for Joint {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.me, &other.me)
			&& self.name == other.name
			&& Weak::ptr_eq(&self.tree, &other.tree)
			&& Weak::ptr_eq(&self.parent_link, &other.parent_link)
			&& Arc::ptr_eq(&self.child_link, &other.child_link)
			&& self.joint_type == other.joint_type
			&& self.transform == other.transform
			&& self.axis() == other.axis()
			&& self.calibration == other.calibration
			&& self.dynamics == other.dynamics
			&& self.limit == other.limit
			&& self.mimic == other.mimic
			&& self.safety_controller == other.safety_controller
		// `self.me` field is not covered however if they are on the same tree it is impossible to not match `self.me` while matching all the other fields
	}
}

// TODO: Might add data of specif joint type to Struct Spaces.
// TODO: Expand Jointtype specifics
/// An enum to represent the the types of `Joint`.
///
/// Currently, only URDF types are listed.
///
/// It is important to note that multi-axis jointtypes, like [`Floating`](JointType::Floating) and [`Planar`](JointType::Planar), are often incompatible with lots of different tooling for ROS/URDF, like `sensor_msgs/JointState` messages from for example [`joint_state_publisher`](https://github.com/ros/joint_state_publisher/blob/7cb7069d2d78ebe4b8b80adc6bd859df0c8ccfc9/joint_state_publisher/src/joint_state_publisher/__init__.py#L83-L85C29) which has chosen to ignore it.
/// This is a result of most programs (like [`kdl_parser`](https://github.com/ros/kdl_parser/blob/74d4ee3bc6938de8ae40a700997baef06114ea1b/kdl_parser/src/kdl_parser.cpp#L103) and `joint_state_publisher`) only supporting single axis joints.
/// Gazebo/[SDFormat](http://sdformat.org/spec?ver=1.10&elem=joint#joint_type) supports multi-access joints, but do not have an `Floating` or `Planar` equivalent.
/// However, these `JointType`s (Mostly [`Planar`](JointType::Planar)) can be approximated by a combination of other joints.  
///
/// The sections cited from the URDF specification are accurate as of [2023-08-21](https://wiki.ros.org/urdf/XML/joint#Attributes).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum JointType {
	/// A Fixed joint.
	///
	/// The URDF Specification says the following:
	/// > fixed — this is not really a joint because it cannot move. All degrees of freedom are locked. This type of joint does not require the `<axis>`, `<calibration>`, `<dynamics>`, `<limits>` or `<safety_controller>`.
	#[default]
	Fixed,
	/// A Revolute joint. (A limited rotational joint, like a weld)
	///
	/// This joint rotates a limited range around a specified (or default) axis.
	///
	/// The URDF Specification says the following:
	/// > revolute — a hinge joint that rotates along the axis and has a limited range specified by the upper and lower limits.
	Revolute,
	/// A Continuous rotational joint. (A bearing or motor connection, like a wheel)
	///
	/// This joint rotates around a specified (or default) axis.
	///
	/// Since this Jointtype is unlimited in its movement, the [Limit](joint_data::LimitData) should not be specified (??TODO: [`effort`](joint_data::LimitData::effort) and [`velocity`](joint_data::LimitData::velocity) migth be useable??).
	///
	/// The URDF Specification says the following:
	/// > continuous — a continuous hinge joint that rotates around the axis and has no upper and lower limits.
	Continuous,
	/// A Prismitic joint. (A limited sliding joint, like a drawer rail or a linear actuator)
	///
	/// This joint slides a limited range along a specified (or default) axis.
	///
	/// The URDF Specification says the following:
	/// > prismatic — a sliding joint that slides along the axis, and has a limited range specified by the upper and lower limits.
	Prismatic,
	/// A Floating joint. (Or a non-connection)
	///
	/// This is a joint to represent an unconnected link.
	/// Most parsers do not handle this `JointType`, since it's not an actual joint and it causes problems with lots of different tooling for ROS/URDF, like `sensor_msgs/JointState` messages from for example [`joint_state_publisher`](https://github.com/ros/robot_model/issues/188) which has chosen to ignore it.
	///
	/// The URDF Specification says the following:
	/// > floating — this joint allows motion for all 6 degrees of freedom.
	Floating,
	/// A Planar joint. (A plane contact, like a magnet on a metal sheet)
	///
	/// This joint slides on the plane perpendicular the specified (or default) axis.
	/// This `JointType` might cause problems with lots of different tooling for ROS/URDF, like `sensor_msgs/JointState` messages from for example [`joint_state_publisher`](https://github.com/ros/joint_state_publisher/blob/7cb7069d2d78ebe4b8b80adc6bd859df0c8ccfc9/joint_state_publisher/src/joint_state_publisher/__init__.py#L83-L85C29) which has chosen to ignore it.
	///
	/// The URDF Specification says the following:
	/// > planar — this joint allows motion in a plane perpendicular to the axis.
	Planar,
}

impl ToString for JointType {
	fn to_string(&self) -> String {
		match self {
			JointType::Fixed => String::from("fixed"),
			JointType::Revolute => String::from("revolute"),
			JointType::Continuous => String::from("continuous"),
			JointType::Prismatic => String::from("prismatic"),
			JointType::Floating => String::from("floating"),
			JointType::Planar => String::from("planar"),
		}
	}
}

#[cfg(feature = "xml")]
impl From<JointType> for Cow<'_, [u8]> {
	fn from(value: JointType) -> Self {
		value.to_string().into_bytes().into()
	}
}

#[cfg(test)]
mod tests {

	use crate::{
		cluster_objects::KinematicInterface,
		joint::{joint_data, smartjointbuilder::SmartJointBuilder, JointBuilder, JointType},
		link::{
			builder::LinkBuilder,
			link_data::{
				geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
				Visual,
			},
		},
		linkbuilding::{CollisionBuilder, VisualBuilder},
		material::MaterialDescriptor,
		transform::Transform,
	};
	use test_log::test;

	#[test]
	fn rebuild() {
		let tree = LinkBuilder::new("root").build_tree();
		tree.get_newest_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new("Joint1")
					.fixed()
					.add_transform(Transform::new_translation(2.0, 3.0, 5.0)),
				LinkBuilder::new("child"),
			)
			.unwrap();

		let rebuilder = tree
			.get_joint("Joint1")
			.unwrap()
			.try_read()
			.unwrap()
			.rebuild();
		assert_eq!(
			rebuilder,
			JointBuilder::new("Joint1", crate::JointType::Fixed).add_origin_offset((2.0, 3.0, 5.0))
		)
	}

	#[test]
	fn yank_simple() {
		let material_red = MaterialDescriptor::new_color(1., 0., 0., 1.).named("Red");

		let tree = LinkBuilder::new("link-0")
			.add_collider(CollisionBuilder::new(BoxGeometry::new(1.0, 2.0, 3.0)))
			.add_visual(
				Visual::builder(BoxGeometry::new(1.0, 2.0, 3.0)).materialized(material_red.clone()),
			)
			.build_tree();

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new("joint-0")
					.add_transform(Transform::new_translation(1.0, 0., 0.))
					.fixed(),
				LinkBuilder::new("link-1")
					.add_collider(
						CollisionBuilder::new(SphereGeometry::new(4.))
							.transformed(Transform::new_translation(2., 0., 0.)),
					)
					.add_visual(
						Visual::builder(SphereGeometry::new(4.))
							.transformed(Transform::new_translation(2., 0., 0.))
							.materialized(material_red.clone()),
					),
			)
			.unwrap();

		assert_eq!(tree.get_links().try_read().unwrap().len(), 2);
		assert_eq!(tree.get_joints().try_read().unwrap().len(), 1);
		assert_eq!(tree.get_materials().try_read().unwrap().len(), 1);

		// let joint =Arc::clone(tree.get_root_link().try_read().unwrap().get_joints().last().unwrap());
		// let builder = joint.try_read().unwrap().yank();

		// let builder = tree
		// 	.get_joint("joint-0")
		// 	.unwrap()
		// 	.try_read()
		// 	.unwrap()
		// 	.yank();
		let builder = tree.yank_joint("joint-0").unwrap();

		assert_eq!(tree.get_links().try_read().unwrap().len(), 1);
		assert_eq!(tree.get_joints().try_read().unwrap().len(), 0);

		assert_eq!(
			builder.0,
			JointBuilder {
				name: "joint-0".into(),
				joint_type: JointType::Fixed,
				transform: Transform {
					translation: Some((1., 0., 0.)),
					rotation: None
				}
				.into(),
				child: Some(LinkBuilder {
					name: "link-1".into(),
					visuals: vec![VisualBuilder::new_full(
						None,
						Some(Transform {
							translation: Some((2., 0., 0.)),
							rotation: None
						}),
						SphereGeometry::new(4.),
						Some(material_red.clone())
					)],
					colliders: vec![CollisionBuilder::new_full(
						None,
						Some(Transform {
							translation: Some((2., 0., 0.)),
							rotation: None
						}),
						SphereGeometry::new(4.)
					)],
					..Default::default()
				}),
				..Default::default() // TODO: Decide if this is Ok to do in a test
			}
		);

		// TODO: Maybe the test is to simple
	}

	#[test]
	fn yank_less_simple() {
		let tree = {
			let material_red = MaterialDescriptor::new_color(1., 0., 0., 1.).named("Red");

			LinkBuilder::new("link-0")
				.add_collider(CollisionBuilder::new(BoxGeometry::new(1.0, 2.0, 3.0)))
				.add_visual(
					Visual::builder(BoxGeometry::new(1.0, 2.0, 3.0))
						.materialized(material_red.clone()),
				)
				.build_tree()
		};

		tree.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new("joint-0")
					.add_transform(Transform::new_translation(1.0, 0., 0.))
					.fixed(),
				{
					let tmp_tree = LinkBuilder::new("link-1")
						.add_collider(
							CollisionBuilder::new(SphereGeometry::new(4.))
								.transformed(Transform::new_translation(2., 0., 0.)),
						)
						.add_visual(
							Visual::builder(SphereGeometry::new(4.))
								.transformed(Transform::new_translation(2., 0., 0.))
								.materialized(
									MaterialDescriptor::new_color(0., 0., 1., 1.).named("Blue"),
								),
						)
						.build_tree();

					tmp_tree
						.get_root_link()
						.write()
						.unwrap()
						.try_attach_child(
							SmartJointBuilder::new("joint-1-1")
								.revolute()
								.add_transform(Transform::new_translation(4., 0., 0.))
								.with_axis((0., 0., 1.))
								.with_limit(100., 1000.)
								.set_upper_limit(std::f32::consts::FRAC_PI_6)
								.set_lower_limit(-std::f32::consts::FRAC_PI_6),
							LinkBuilder::new("link-1-1").add_visual(
								Visual::builder(CylinderGeometry::new(0.5, 18.))
									.named("link-1-1-vis")
									.transformed(Transform::new_translation(9., 0.5, 0.))
									.materialized(MaterialDescriptor::new_color(
										0.5, 0.5, 0.5, 0.75,
									)),
							),
						)
						.unwrap();

					tmp_tree
				},
			)
			.unwrap();

		tree.get_root_link()
			.write()
			.unwrap()
			.try_attach_child(
				JointBuilder::new("joint-2", JointType::Fixed).add_origin_offset((0., 0., 1.5)),
				LinkBuilder::new("link-2").build_tree(),
			)
			.unwrap();

		assert_eq!(tree.get_links().try_read().unwrap().len(), 4);
		assert_eq!(tree.get_joints().try_read().unwrap().len(), 3);
		assert_eq!(tree.get_materials().try_read().unwrap().len(), 2);

		assert_eq!(tree.get_root_link().try_read().unwrap().name(), "link-0");
		assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "link-2");

		{
			let tree = tree.clone();
			let yanked_branch = tree.yank_joint("joint-2");

			assert!(yanked_branch.is_some());

			assert_eq!(tree.get_links().try_read().unwrap().len(), 3);
			assert_eq!(tree.get_joints().try_read().unwrap().len(), 2);
			assert_eq!(tree.get_materials().try_read().unwrap().len(), 2);

			{
				let mut link_keys: Vec<String> = tree
					.get_links()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				link_keys.sort();

				assert_eq!(link_keys, vec!["link-0", "link-1", "link-1-1"]);
			}
			{
				let mut joint_keys: Vec<String> = tree
					.get_joints()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				joint_keys.sort();

				assert_eq!(joint_keys, vec!["joint-0", "joint-1-1"]);
			}
			{
				let mut material_keys: Vec<String> = tree
					.get_materials()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				material_keys.sort();

				assert_eq!(material_keys, vec!["Blue", "Red"]);
			}

			assert_eq!(tree.get_root_link().read().unwrap().name(), "link-0");
			assert_eq!(tree.get_newest_link().read().unwrap().name(), "link-0");

			assert_eq!(
				yanked_branch.unwrap().0,
				JointBuilder {
					name: "joint-2".into(),
					joint_type: JointType::Fixed,
					transform: Transform {
						translation: Some((0., 0., 1.5)),
						..Default::default()
					}
					.into(),
					child: Some(LinkBuilder::new("link-2")),
					..Default::default()
				}
			)
		}

		{
			let tree = tree.clone();
			let yanked_branch = tree.yank_joint("joint-1-1");

			assert!(yanked_branch.is_some());

			assert_eq!(tree.get_links().try_read().unwrap().len(), 3);
			assert_eq!(tree.get_joints().try_read().unwrap().len(), 2);
			assert_eq!(tree.get_materials().try_read().unwrap().len(), 2);

			{
				let mut link_keys: Vec<String> = tree
					.get_links()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				link_keys.sort();

				assert_eq!(link_keys, vec!["link-0", "link-1", "link-2"]);
			}
			{
				let mut joint_keys: Vec<String> = tree
					.get_joints()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				joint_keys.sort();

				assert_eq!(joint_keys, vec!["joint-0", "joint-2"]);
			}
			{
				let mut material_keys: Vec<String> = tree
					.get_materials()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				material_keys.sort();

				assert_eq!(material_keys, vec!["Blue", "Red"]);
			}

			assert_eq!(tree.get_root_link().read().unwrap().name(), "link-0");
			assert_eq!(tree.get_newest_link().read().unwrap().name(), "link-1");

			assert_eq!(
				yanked_branch.unwrap().0,
				JointBuilder {
					name: "joint-1-1".into(),
					joint_type: JointType::Revolute,
					transform: Transform {
						translation: Some((4., 0., 0.)),
						..Default::default()
					}
					.into(),
					child: Some(LinkBuilder {
						name: "link-1-1".into(),
						visuals: vec![VisualBuilder::new_full(
							Some("link-1-1-vis".into()),
							Some(Transform {
								translation: Some((9., 0.5, 0.)),
								..Default::default()
							}),
							CylinderGeometry::new(0.5, 18.),
							Some(MaterialDescriptor::new_color(0.5, 0.5, 0.5, 0.75))
						)],
						..Default::default()
					}),
					axis: Some((0., 0., 1.)),
					limit: Some(joint_data::LimitData {
						effort: 100.,
						velocity: 1000.,
						lower: Some(-std::f32::consts::FRAC_PI_6),
						upper: Some(std::f32::consts::FRAC_PI_6),
					}),
					..Default::default()
				}
			)
		}

		{
			let tree = tree.clone();
			let yanked_branch = tree.yank_joint("joint-0");

			assert!(yanked_branch.is_some());

			assert_eq!(tree.get_links().try_read().unwrap().len(), 2);
			assert_eq!(tree.get_joints().try_read().unwrap().len(), 1);
			assert_eq!(tree.get_materials().try_read().unwrap().len(), 2);

			{
				let mut link_keys: Vec<String> = tree
					.get_links()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				link_keys.sort();

				assert_eq!(link_keys, vec!["link-0", "link-2"]);
			}
			{
				let mut joint_keys: Vec<String> = tree
					.get_joints()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				joint_keys.sort();

				assert_eq!(joint_keys, vec!["joint-2"]);
			}
			{
				let mut material_keys: Vec<String> = tree
					.get_materials()
					.try_read()
					.unwrap()
					.keys()
					.cloned()
					.collect();
				material_keys.sort();

				assert_eq!(material_keys, vec!["Blue", "Red"]);
			}

			assert_eq!(tree.get_root_link().read().unwrap().name(), "link-0");
			assert_eq!(tree.get_newest_link().read().unwrap().name(), "link-0");

			assert_eq!(
				yanked_branch.unwrap().0,
				JointBuilder {
					name: "joint-0".into(),
					transform: Transform {
						translation: Some((1., 0., 0.)),
						..Default::default()
					}
					.into(),
					child: Some(LinkBuilder {
						name: "link-1".into(),
						visuals: vec![VisualBuilder::new_full(
							None,
							Some(Transform {
								translation: Some((2., 0., 0.)),
								..Default::default()
							}),
							SphereGeometry::new(4.),
							Some(MaterialDescriptor::new_color(0., 0., 1., 1.,).named("Blue"))
						)],
						colliders: vec![CollisionBuilder::new(SphereGeometry::new(4.))
							.transformed(Transform::new_translation(2., 0., 0.))],
						joints: vec![JointBuilder {
							name: "joint-1-1".into(),
							joint_type: JointType::Revolute,
							transform: Transform {
								translation: Some((4., 0., 0.)),
								..Default::default()
							}
							.into(),
							child: Some(LinkBuilder {
								name: "link-1-1".into(),
								visuals: vec![VisualBuilder::new_full(
									Some("link-1-1-vis".into()),
									Some(Transform {
										translation: Some((9., 0.5, 0.)),
										..Default::default()
									}),
									CylinderGeometry::new(0.5, 18.),
									Some(MaterialDescriptor::new_color(0.5, 0.5, 0.5, 0.75))
								)],
								..Default::default()
							}),
							axis: Some((0., 0., 1.)),
							limit: Some(joint_data::LimitData {
								effort: 100.,
								velocity: 1000.,
								lower: Some(-std::f32::consts::FRAC_PI_6),
								upper: Some(std::f32::consts::FRAC_PI_6),
							}),
							..Default::default()
						}],
						..Default::default()
					}),
					..Default::default()
				}
			)
		}
	}
}
