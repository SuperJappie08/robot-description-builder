mod joint_tranform_mode;
mod jointbuilder;
mod smartjointbuilder;

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
	chained::Chained, cluster_objects::kinematic_data_tree::KinematicDataTree,
	identifiers::GroupID, link::Link, transform::Transform, ArcLock, WeakLock,
};

#[cfg(any(feature = "xml"))]
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
	origin: Transform,
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

	pub fn jointtype(&self) -> JointType {
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

	pub fn origin(&self) -> &Transform {
		&self.origin
	}

	/// Make a `JointBuilder` to build a 'Clone' of the `Joint`
	pub fn rebuild(&self) -> JointBuilder {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Rebuilding: {}", self.name());
		JointBuilder {
			name: self.name.clone(),
			joint_type: self.joint_type,
			origin: self.origin.into(),
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic.clone().map(|mimic| mimic.into()),
			safety_controller: self.safety_controller,
			..Default::default()
		}
	}

	/// TODO: MAKE A PUBLIC VERSION WHICH RETURNS WRAPPED IN CHAINED
	pub(crate) fn rebuild_branch_continued(&self) -> JointBuilder {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Rebuilding: {}", self.name());
		JointBuilder {
			child: Some(self.child_link.read().unwrap().rebuild_branch_continued()), // FIXME: Figure out if unwrap is Ok here?
			..self.rebuild()
		}
	}

	pub fn rebuild_branch(&self) -> Chained<JointBuilder> {
		#[cfg(any(feature = "logging", test))]
		log::info!(target: "JointBuilder","Starting Branch Rebuilding: {}", self.name());
		Chained(self.rebuild_branch_continued())
	}

	/// TODO:Find a way to make these builders special?
	/// - Fix Documentation
	///
	/// Still need to purge
	///
	/// NOTE: you must get the link from the rep by cloning.
	/// TODO: Maybe add a `first` argument to only set the `newest_link` if it is the first in the call stack
	pub(crate) fn yank(&self) -> JointBuilder {
		let builder = self.rebuild_branch_continued();

		#[cfg(any(feature = "logging", test))]
		log::info!("Yanked Joint \"{}\"", self.name());

		self.parent_link()
			.try_write()
			.unwrap() // FIXME: UNWRAP NOT OK
			.joints_mut()
			.retain(|joint| !Arc::ptr_eq(&self.get_self(), joint));

		*self.tree.upgrade().unwrap().newest_link.write().unwrap() = Weak::clone(&self.parent_link);

		builder
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
				value: self.jointtype().into(),
			});

		element.write_inner_content(|writer| {
			let origin = self.origin();
			if origin.contains_some() {
				origin.to_urdf(writer, urdf_config)?;
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

/// TODO: Are all fields covered?
impl PartialEq for Joint {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.me, &other.me)
			&& self.name == other.name
			&& Weak::ptr_eq(&self.tree, &other.tree)
			&& Weak::ptr_eq(&self.parent_link, &other.parent_link)
			&& Arc::ptr_eq(&self.child_link, &other.child_link)
			&& self.joint_type == other.joint_type
			&& self.origin == other.origin
	}
}

/// TODO: Might add data of specif joint type to Struct Spaces.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum JointType {
	/// TODO: TEMP DEFAULT
	#[default]
	Fixed, // — this is not really a joint because it cannot move. All degrees of freedom are locked. This type of joint does not require the <axis>, <calibration>, <dynamics>, <limits> or <safety_controller>.
	Revolute, // — a hinge joint that rotates along the axis and has a limited range specified by the upper and lower limits.
	Continuous, // — a continuous hinge joint that rotates around the axis and has no upper and lower limits.
	Prismatic, // — a sliding joint that slides along the axis, and has a limited range specified by the upper and lower limits.
	Floating,  // — this joint allows motion for all 6 degrees of freedom.
	Planar,    // — this joint allows motion in a plane perpendicular to the axis.
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

#[cfg(any(feature = "xml"))]
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
				LinkBuilder::new("child"),
				SmartJointBuilder::new("Joint1")
					.fixed()
					.add_transform(Transform::new_translation(2.0, 3.0, 5.0)),
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
				LinkBuilder::new("link-1")
					.add_collider(
						CollisionBuilder::new(SphereGeometry::new(4.))
							.tranformed(Transform::new_translation(2., 0., 0.)),
					)
					.add_visual(
						Visual::builder(SphereGeometry::new(4.))
							.tranformed(Transform::new_translation(2., 0., 0.))
							.materialized(material_red.clone()),
					),
				SmartJointBuilder::new("joint-0")
					.add_transform(Transform::new_translation(1.0, 0., 0.))
					.fixed(),
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
				origin: Transform {
					translation: Some((1., 0., 0.)),
					rotation: None
				}
				.into(),
				child: Some(LinkBuilder {
					name: "link-1".into(),
					visual_builders: vec![VisualBuilder::new_full(
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

		// todo!("FINISH TEST 'lib::joint::test::yank_simple'")
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
				{
					let tmp_tree = LinkBuilder::new("link-1")
						.add_collider(
							CollisionBuilder::new(SphereGeometry::new(4.))
								.tranformed(Transform::new_translation(2., 0., 0.)),
						)
						.add_visual(
							Visual::builder(SphereGeometry::new(4.))
								.tranformed(Transform::new_translation(2., 0., 0.))
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
							LinkBuilder::new("link-1-1").add_visual(
								Visual::builder(CylinderGeometry::new(0.5, 18.))
									.named("link-1-1-vis")
									.tranformed(Transform::new_translation(9., 0.5, 0.))
									.materialized(MaterialDescriptor::new_color(
										0.5, 0.5, 0.5, 0.75,
									)),
							),
							SmartJointBuilder::new("joint-1-1")
								.revolute()
								.add_transform(Transform::new_translation(4., 0., 0.))
								.with_axis((0., 0., 1.))
								.with_limit(100., 1000.)
								.set_upper_limit(std::f32::consts::FRAC_PI_6)
								.set_lower_limit(-std::f32::consts::FRAC_PI_6),
						)
						.unwrap();

					tmp_tree
				},
				SmartJointBuilder::new("joint-0")
					.add_transform(Transform::new_translation(1.0, 0., 0.))
					.fixed(),
			)
			.unwrap();

		tree.get_root_link()
			.write()
			.unwrap()
			.try_attach_child(
				LinkBuilder::new("link-2").build_tree(),
				JointBuilder::new("joint-2", JointType::Fixed).add_origin_offset((0., 0., 1.5)),
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
					origin: Transform {
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
					origin: Transform {
						translation: Some((4., 0., 0.)),
						..Default::default()
					}
					.into(),
					child: Some(LinkBuilder {
						name: "link-1-1".into(),
						visual_builders: vec![VisualBuilder::new_full(
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
					origin: Transform {
						translation: Some((1., 0., 0.)),
						..Default::default()
					}
					.into(),
					child: Some(LinkBuilder {
						name: "link-1".into(),
						visual_builders: vec![VisualBuilder::new_full(
							None,
							Some(Transform {
								translation: Some((2., 0., 0.)),
								..Default::default()
							}),
							SphereGeometry::new(4.),
							Some(MaterialDescriptor::new_color(0., 0., 1., 1.,).named("Blue"))
						)],
						colliders: vec![CollisionBuilder::new(SphereGeometry::new(4.))
							.tranformed(Transform::new_translation(2., 0., 0.))],
						joints: vec![JointBuilder {
							name: "joint-1-1".into(),
							joint_type: JointType::Revolute,
							origin: Transform {
								translation: Some((4., 0., 0.)),
								..Default::default()
							}
							.into(),
							child: Some(LinkBuilder {
								name: "link-1-1".into(),
								visual_builders: vec![VisualBuilder::new_full(
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
