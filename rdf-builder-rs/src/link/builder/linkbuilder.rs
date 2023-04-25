use std::sync::{Arc, RwLock, Weak};

use inertial::InertialData;
use itertools::process_results;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{BuildJointChain, Joint, JointBuilder},
	link::{
		builder::{visual_builder::VisualBuilder, BuildLink},
		Link, LinkParent, LinkShapeData,
	},
	link::{inertial, link_data},
	ArcLock, KinematicTree, WeakLock,
};

use super::CollisionBuilder;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkBuilder {
	// All fields are pub(crate) so I can struct initialize in rebuild
	pub(crate) name: String,
	/// TODO: Figure out if we make this immutable on a `Link` and only allow editting throug the builder.
	pub(crate) visual_builders: Vec<VisualBuilder>,
	pub(crate) colliders: Vec<CollisionBuilder>,
	/// TODO: Calulate InertialData?
	pub(crate) intertial: Option<link_data::InertialData>,
	pub(crate) joints: Vec<JointBuilder>,
}

impl LinkBuilder {
	/// TODO: depreaction Planned
	pub fn new(name: impl Into<String>) -> LinkBuilder {
		Self {
			name: name.into(),
			..Default::default()
		}
	}

	pub fn add_visual(mut self, visual: VisualBuilder) -> Self {
		self.visual_builders.push(visual);
		self
	}

	/// TODO: Not really sure if this is the way... but it is how clap does it.
	pub fn add_collider(mut self, collider: CollisionBuilder) -> Self {
		self.colliders.push(collider);
		self
	}

	pub fn intertial(mut self, inertial: InertialData) -> Self {
		self.intertial = Some(inertial);
		self
	}

	// ===== NON BUILDER METHODS =====

	pub fn get_visuals(&self) -> &Vec<VisualBuilder> {
		&self.visual_builders
	}

	pub fn get_visuals_mut(&mut self) -> &mut Vec<VisualBuilder> {
		&mut self.visual_builders
	}

	pub fn get_colliders(&self) -> &Vec<CollisionBuilder> {
		&self.colliders
	}

	pub fn get_colliders_mut(&mut self) -> &mut Vec<CollisionBuilder> {
		&mut self.colliders
	}

	pub fn get_joints(&self) -> &Vec<JointBuilder> {
		&self.joints
	}

	// pub(crate) fn build(self, tree: ArcLock<KinematicTreeData>) -> ArcLock<Link> {
	//     // Not sure How i wanna do this yet,
	//     // Maybe with colliders and visuals, stacking and calculating the always calculating the endpoint or not?
	// }

	/// FIXME: This is temporary, since BuildLink is now a private trait
	pub fn build_tree(self) -> KinematicTree {
		BuildLink::build_tree(self)
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
				visuals: process_results(
					self.visual_builders
						.into_iter()
						.map(|visual_builder| visual_builder.build()),
					|iter| iter.collect(),
				)
				.unwrap(),
				colliders: self
					.colliders
					.into_iter()
					.map(|collision_builder| collision_builder.build())
					.collect(),
				end_point: None, //TODO:
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
				visuals: itertools::process_results(
					self.visual_builders
						.into_iter()
						.map(|visual_builder| visual_builder.build()),
					|iter| iter.collect(),
				)
				.unwrap(), // UNWRAP NOT OK
				colliders: self
					.colliders
					.into_iter()
					.map(|collider_builder| collider_builder.build())
					.collect(),
				end_point: None, // FIXME: Fix this
				me: Weak::clone(me),
			})
		})
	}

	fn get_shape_data(&self) -> LinkShapeData {
		LinkShapeData::new(
			self.get_visuals()
				.iter()
				.map(|visual| visual.get_geometry_data()),
		)
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		link::{
			builder::{BuildLink, LinkBuilder, VisualBuilder},
			geometry::{BoxGeometry, GeometryShapeData, SphereGeometry},
			link_shape_data::LinkShapeData,
		},
		link_data::geometry::CylinderGeometry,
		linkbuilding::CollisionBuilder,
		transform_data::Transform,
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
						origin: Transform::default(),
						geometry: SphereGeometry::new(0.).into()
					},
					geometries: vec![GeometryShapeData {
						origin: Transform::default(),
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
						origin: Transform::default(),
						geometry: BoxGeometry::new(10., 20., 30.).into()
					},
					geometries: vec![GeometryShapeData {
						origin: Transform::default(),
						geometry: BoxGeometry::new(10., 20., 30.).into()
					}]
				}
			)
		}
		{
			let link_builder = LinkBuilder::new("a Link")
				.add_visual(
					VisualBuilder::new(CylinderGeometry::new(1., 2.))
						.tranformed(Transform::new_translation(5., 0., 16.)),
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
						origin: Transform::new_translation(5., 0., 16.),
						geometry: CylinderGeometry::new(1., 2.).into()
					},
					geometries: vec![
						GeometryShapeData {
							origin: Transform::new_translation(5., 0., 16.),
							geometry: CylinderGeometry::new(1., 2.).into()
						},
						GeometryShapeData {
							origin: Transform::default(),
							geometry: BoxGeometry::new(10., 20., 30.).into()
						}
					]
				}
			)
		}
	}
}
