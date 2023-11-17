use std::sync::{Arc, RwLock, Weak};

use nalgebra::Matrix3;

use super::{BuildLink, CollisionBuilder, VisualBuilder};
use crate::{
	cluster_objects::{kinematic_data_tree::KinematicDataTree, KinematicTree},
	identifiers::GroupIDChanger,
	joint::{BuildJointChain, Joint, JointBuilder},
	link::{link_data, Link, LinkParent, LinkShapeData},
	transform::Mirror,
	utils::{ArcLock, WeakLock},
};

/// The builder for the `Link` type.
///
/// The `LinkBuilder` is used to construct a [`Link`].
/// It can be attached to a pre-existing [`Link`] in a [`KinematicTree`] or a [`Robot`](crate::cluster_objects::Robot).
/// Or a new [`KinematicTree`] can be started with this `LinkBuilder` as the blueprint for the `root_link`.
///
/// This will configure most of the link data:
/// - **`name`**: The [_string identifier_](crate::identifiers) (or name) of this [`Link`]. For practical purposes, it is recommended to use unique identifiers/names.
/// - **[`visuals`](super::VisualBuilder)** (0+): The builders for the [`Visual`](crate::link::Visual) elements associated with this [`Link`].
/// - **[`colliders`](super::CollisionBuilder)** (0+): The builders for the [`Collision`](crate::link::Collision) elements associated with this [`Link`].
/// - **[`joints`](crate::joint::JointBuilder)** (0+): The buiders for the child [`Joints`](crate::joint::Joint) of this [`Link`].
/// (This field should be/is assumed as empty on a bare `LinkBuilder`, but can optionally be non-empty in a [`Chained<LinkBuilder>`](crate::chained::Chained))<br/>
/// This field only be filled when reconstructing or yanking a pre-existing [`Link`]/[`Joint`].
/// - **[`inertial`](crate::link::inertial::Inertial)** (Optional): The [`Inertial`](crate::link::inertial::Inertial) data for this [`Link`].
// TODO: Check if something is missing?
#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkBuilder {
	// All fields are pub(crate) so I can struct initialize in rebuild
	/// The [_string identifier_](crate::identifiers) or name of this `Link`.
	///
	/// For practical purposes, it is recommended to use unique identifiers/names.
	pub(crate) name: String,
	// TODO: Figure out if we make this immutable on a `Link` and only allow editting throug the builder.
	pub(crate) visuals: Vec<VisualBuilder>,
	pub(crate) colliders: Vec<CollisionBuilder>,
	// TODO: Calulate Inertial?
	pub(crate) intertial: Option<link_data::Inertial>,
	pub(crate) joints: Vec<JointBuilder>,
}

impl LinkBuilder {
	/// Create a new [`LinkBuilder`] with the specified `name`.
	pub fn new(name: impl Into<String>) -> LinkBuilder {
		Self {
			name: name.into(),
			..Default::default()
		}
	}

	/// Adds a [`VisualBuilder`] to this `LinkBuilder`.
	pub fn add_visual(mut self, visual: VisualBuilder) -> Self {
		self.visuals.push(visual);
		self
	}

	// TODO: Not really sure if this is the way... but it is how clap does it.
	/// Adds a [`CollisionBuilder`] to this `LinkBuilder`.
	pub fn add_collider(mut self, collider: CollisionBuilder) -> Self {
		self.colliders.push(collider);
		self
	}

	// TODO: Naming not inline with convention
	// Not happy with the added `add_` but otherwise name colliding with getter
	/// Sets the [`Inertial`](link_data::Inertial) (`inertial`) of this `LinkBuilder`.
	pub fn add_intertial(mut self, inertial: link_data::Inertial) -> Self {
		self.intertial = Some(inertial);
		self
	}

	/// Creates a [`KinematicTree`] by building this `LinkBuilder`.
	pub fn build_tree(self) -> KinematicTree {
		BuildLink::build_tree(self)
	}
}

/// Non-builder methods
impl LinkBuilder {
	/// Gets a reference to the `name` of this `LinkBuilder`.
	pub fn name(&self) -> &String {
		&self.name
	}

	// TODO: Maybe Change to Iterator
	/// Gets a reference to the `visuals` of this `LinkBuilder`.
	pub fn visuals(&self) -> &Vec<VisualBuilder> {
		&self.visuals
	}

	// TODO: Maybe Change to Iterator
	/// Gets a mutable reference to the `visuals` of this `LinkBuilder`.
	pub fn visuals_mut(&mut self) -> &mut Vec<VisualBuilder> {
		&mut self.visuals
	}

	// TODO: Maybe Change to Iterator
	/// Gets a reference to the `colliders` of this `LinkBuilder`.
	pub fn colliders(&self) -> &Vec<CollisionBuilder> {
		&self.colliders
	}

	// TODO: Maybe Change to Iterator
	/// Gets a mutable reference to the `colliders` of this `LinkBuilder`.
	pub fn colliders_mut(&mut self) -> &mut Vec<CollisionBuilder> {
		&mut self.colliders
	}

	// TODO: Maybe Change to Iterator
	/// Gets a reference to the `joints` of this `LinkBuilder`.
	pub fn joints(&self) -> &Vec<JointBuilder> {
		&self.joints
	}

	/// Gets an optional reference to the [`Inertial`](link_data::Inertial) of this `LinkBuilder`.
	pub fn inertial(&self) -> Option<&link_data::Inertial> {
		self.intertial.as_ref()
	}
}

impl Mirror for LinkBuilder {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		Self {
			name: self.name.clone(), // TODO: rename mirrored
			visuals: self
				.visuals
				.iter()
				.map(|visual_builder| visual_builder.mirrored(mirror_matrix))
				.collect(),
			colliders: self
				.colliders
				.iter()
				.map(|collider_builder| collider_builder.mirrored(mirror_matrix))
				.collect(),
			intertial: self
				.intertial
				.as_ref()
				.map(|intertial_data| intertial_data.mirrored(mirror_matrix)),
			joints: self
				.joints
				.iter()
				.map(|joint_builder| joint_builder.mirrored(mirror_matrix))
				.collect(),
		}
	}
}

impl BuildLink for LinkBuilder {
	fn build(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		#[cfg(any(feature = "logging", test))]
		log::info!("Making a Link[name = \"{}\"]", self.name);

		Arc::new_cyclic(|me| {
			RwLock::new(Link {
				name: self.name,
				tree: Weak::clone(tree),
				direct_parent: LinkParent::KinematicTree(Weak::clone(tree)),
				child_joints: Vec::new(),
				inertial: self.intertial,
				visuals: self.visuals.into_iter().map(VisualBuilder::build).collect(),
				colliders: self
					.colliders
					.into_iter()
					.map(CollisionBuilder::build)
					.collect(),
				me: Weak::clone(me),
			})
		})
	}

	fn start_building_chain(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		let joint_builders = self.joints.clone();
		let root = self.build(tree);

		// This unwrap is Ok since the Link has just been build
		let shape_data = root.read().unwrap().get_shape_data();

		// This unwrap is Ok since the Link has just been build
		root.write().unwrap().child_joints = joint_builders
			.into_iter()
			.map(|joint_builder| {
				joint_builder.build_chain(tree, &Arc::downgrade(&root), shape_data.clone())
			})
			.collect();
		root
	}

	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link> {
		let shape_data = self.get_shape_data();

		Arc::new_cyclic(|me| {
			RwLock::new(Link {
				name: self.name,
				tree: Weak::clone(tree),
				direct_parent: LinkParent::Joint(Weak::clone(parent_joint)),
				child_joints: self
					.joints
					.into_iter()
					.map(|joint_builder| joint_builder.build_chain(tree, me, shape_data.clone()))
					.collect(),
				inertial: self.intertial,
				visuals: self.visuals.into_iter().map(VisualBuilder::build).collect(),
				colliders: self
					.colliders
					.into_iter()
					.map(CollisionBuilder::build)
					.collect(),
				me: Weak::clone(me),
			})
		})
	}

	fn get_shape_data(&self) -> LinkShapeData {
		LinkShapeData::new(
			self.visuals()
				.iter()
				.map(|visual| visual.get_geometry_data()),
		)
	}
}

impl GroupIDChanger for LinkBuilder {
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
		self.name.change_group_id_unchecked(new_group_id);

		self.visuals_mut()
			.iter_mut()
			.for_each(|visual_builder| visual_builder.change_group_id_unchecked(new_group_id));
		self.colliders_mut()
			.iter_mut()
			.for_each(|collision_builder| {
				collision_builder.change_group_id_unchecked(new_group_id)
			});

		self.joints
			.iter_mut()
			.for_each(|joint_builder| joint_builder.change_group_id_unchecked(new_group_id));
	}

	fn apply_group_id(&mut self) {
		self.name.apply_group_id();

		self.visuals_mut()
			.iter_mut()
			.for_each(|visual_builder| visual_builder.apply_group_id());
		self.colliders_mut()
			.iter_mut()
			.for_each(|collision_builder| collision_builder.apply_group_id());

		self.joints
			.iter_mut()
			.for_each(|joint_builder| joint_builder.apply_group_id());
	}
}

#[cfg(test)]
mod tests {
	use super::{BuildLink, LinkBuilder};
	use crate::{
		link::{
			builder::{CollisionBuilder, VisualBuilder},
			geometry::{BoxGeometry, CylinderGeometry, GeometryShapeData, SphereGeometry},
			link_shape_data::LinkShapeData,
		},
		transform::Transform,
	};
	use test_log::test;
	//TODO: Write test

	#[test]
	fn get_shape_data() {
		{
			let link_builder = LinkBuilder::new("a Link");

			assert_eq!(
				link_builder.get_shape_data(),
				LinkShapeData {
					main_geometry: GeometryShapeData {
						transform: Transform::default(),
						geometry: SphereGeometry::new(0.).into()
					},
					geometries: vec![GeometryShapeData {
						transform: Transform::default(),
						geometry: SphereGeometry::new(0.).into()
					}]
				}
			)
		}
		{
			let link_builder = LinkBuilder::new("a Link")
				.add_visual(
					VisualBuilder::new(BoxGeometry::new(10., 20., 30.)).named("a link's visual"),
				)
				.add_collider(
					CollisionBuilder::new(SphereGeometry::new(3.)).named("this does not get used"),
				);

			assert_eq!(
				link_builder.get_shape_data(),
				LinkShapeData {
					main_geometry: GeometryShapeData {
						transform: Transform::default(),
						geometry: BoxGeometry::new(10., 20., 30.).into()
					},
					geometries: vec![GeometryShapeData {
						transform: Transform::default(),
						geometry: BoxGeometry::new(10., 20., 30.).into()
					}]
				}
			)
		}
		{
			let link_builder = LinkBuilder::new("a Link")
				.add_visual(
					VisualBuilder::new(CylinderGeometry::new(1., 2.))
						.transformed(Transform::new_translation(5., 0., 16.)),
				)
				.add_visual(
					VisualBuilder::new(BoxGeometry::new(10., 20., 30.)).named("a link's visual"),
				)
				.add_collider(
					CollisionBuilder::new(SphereGeometry::new(3.)).named("this does not get used"),
				);

			assert_eq!(
				link_builder.get_shape_data(),
				LinkShapeData {
					main_geometry: GeometryShapeData {
						transform: Transform::new_translation(5., 0., 16.),
						geometry: CylinderGeometry::new(1., 2.).into()
					},
					geometries: vec![
						GeometryShapeData {
							transform: Transform::new_translation(5., 0., 16.),
							geometry: CylinderGeometry::new(1., 2.).into()
						},
						GeometryShapeData {
							transform: Transform::default(),
							geometry: BoxGeometry::new(10., 20., 30.).into()
						}
					]
				}
			)
		}
	}

	mod group_id_changer {
		use super::{test, LinkBuilder};
		use crate::identifiers::{GroupIDChanger, GroupIDError};

		#[test]
		fn change_group_id_unchecked_simple() {
			#[inline]
			fn test(name: impl Into<String>, new_group_id: &str, result: &str) {
				let mut link_builder = LinkBuilder::new(name);
				unsafe {
					link_builder.change_group_id_unchecked(new_group_id);
				}
				assert_eq!(link_builder.name, result)
			}

			test("leg_[[M09da]]_link_1", "C10df", "leg_[[C10df]]_link_1");
			test("leg_[[M09da]]_link_1", "", "leg_[[]]_link_1");
			test("leg_[[M09da]]_link_1", "[[tsst", "leg_[[[[tsst]]_link_1");
			test("leg_[[M09da]]_link_1", "tsst]]", "leg_[[tsst]]]]_link_1");
		}

		#[test]
		#[ignore = "TODO"]
		fn change_group_id_unchecked_advanced() {
			todo!()
		}

		#[test]
		fn change_group_id_simple() {
			#[inline]
			fn test(
				name: impl Into<String>,
				new_group_id: &str,
				change_result: Result<(), GroupIDError>,
				result: &str,
			) {
				let mut link_builder = LinkBuilder::new(name);
				assert_eq!(link_builder.change_group_id(new_group_id), change_result);
				assert_eq!(link_builder.name, result)
			}

			test(
				"leg_[[M09da]]_link_1",
				"C10df",
				Ok(()),
				"leg_[[C10df]]_link_1",
			);
			test(
				"leg_[[M09da]]_link_1",
				"",
				Err(GroupIDError::new_empty()),
				"leg_[[M09da]]_link_1",
			);
			test(
				"leg_[[M09da]]_link_1",
				"[[tsst",
				Err(GroupIDError::new_open("[[tsst")),
				"leg_[[M09da]]_link_1",
			);
			test(
				"leg_[[M09da]]_link_1",
				"tsst]]",
				Err(GroupIDError::new_close("tsst]]")),
				"leg_[[M09da]]_link_1",
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn change_group_id_advanced() {
			todo!()
		}

		#[test]
		fn apply_group_id_simple() {
			#[inline]
			fn test(name: impl Into<String>, result: &str) {
				let mut link_builder = LinkBuilder::new(name);
				link_builder.apply_group_id();
				assert_eq!(link_builder.name, result)
			}

			test("leg_[[M09da]]_link_1", "leg_M09da_link_1");
			test("leg_[[M09daf_link_1", "leg_[[M09daf_link_1");
			test("leg_sM09da]]_link_1", "leg_sM09da]]_link_1");
			test("leg_[\\[M09da]\\]_link_1", "leg_[[M09da]]_link_1");
		}

		#[test]
		#[ignore = "TODO"]
		fn apply_group_id_advanced() {
			todo!()
		}
	}
}
