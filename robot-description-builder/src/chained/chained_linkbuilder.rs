use std::sync::Weak;

use nalgebra::Matrix3;

use super::{ChainableBuilder, Chained};
use crate::{
	cluster_objects::{kinematic_data_tree::KinematicDataTree, KinematicInterface},
	joint::Joint,
	link::{
		builder::{BuildLink, LinkBuilder},
		Link,
	},
	transform::{Mirror, MirrorAxis},
	utils::{ArcLock, WeakLock},
};

impl Chained<LinkBuilder> {
	/// TODO: More tests
	/// TODO: DOC
	pub fn mirror(&self, axis: MirrorAxis) -> Chained<LinkBuilder> {
		let mirror_matrix: Matrix3<_> = axis.into();
		Chained(self.0.mirrored(&mirror_matrix))
	}
}

impl ChainableBuilder for LinkBuilder {
	fn has_chain(&self) -> bool {
		!self.joints.is_empty()
	}
}

impl BuildLink for Chained<LinkBuilder> {
	fn build(self, _tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		unimplemented!("build should not be able to be called?")
	}

	fn start_building_chain(self, tree: &Weak<KinematicDataTree>) -> ArcLock<Link> {
		self.0.start_building_chain(tree)
	}

	fn build_chain(
		self,
		_tree: &Weak<KinematicDataTree>,
		_parent_joint: &WeakLock<Joint>,
	) -> ArcLock<Link> {
		unimplemented!("build_chain should not be able to be called?")
	}

	fn get_shape_data(&self) -> crate::link::LinkShapeData {
		unimplemented!("get_shape_data should not be able to be called?")
	}
}

/// Since Link's can end a chain, a `LinkBuilder` can always be converted to a `Chained<LinkBuilder>`.
impl From<LinkBuilder> for Chained<LinkBuilder> {
	fn from(value: LinkBuilder) -> Self {
		Self(value)
	}
}

impl From<Chained<LinkBuilder>> for LinkBuilder {
	fn from(value: Chained<LinkBuilder>) -> Self {
		value.0
	}
}

impl<KI> From<KI> for Chained<LinkBuilder>
where
	KI: KinematicInterface,
{
	fn from(value: KI) -> Self {
		value.yank_root().unwrap() // This might not be Ok
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::FRAC_PI_2;

	use test_log::test;

	use crate::{
		cluster_objects::KinematicInterface,
		joint::JointTransformMode,
		link::{
			link_data::{
				geometry::{BoxGeometry, CylinderGeometry},
				Collision, Visual,
			},
			Link,
		},
		linkbuilding::{CollisionBuilder, LinkBuilder, VisualBuilder},
		material::MaterialDescriptor,
		transform::MirrorAxis,
		Chained, JointBuilder, JointType, SmartJointBuilder, Transform,
	};

	#[test]
	fn mirror_simple_1() {
		let material_l1 = MaterialDescriptor::new_rgb(1., 0., 0.).named("Leg_l1");
		let material_l2 = MaterialDescriptor::new_rgb(0., 1., 0.).named("Leg_l2");
		let geom_leg_l1 = BoxGeometry::new(2., 3., 1.);
		let geom_leg_l2 = CylinderGeometry::new(1., 10.);

		let left_leg_1_tree = Link::builder("Leg_[L1]_l1")
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
			)
			.build_tree();

		left_leg_1_tree
			.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new_fixed("Leg_[L1]_j1")
					.add_transform(Transform::new((0., 3., 0.), (0., 0., FRAC_PI_2))),
				Link::builder("Leg_[L1]_l2")
					.add_visual(
						Visual::builder(geom_leg_l2.clone())
							.transformed(Transform::new((0., 5., 0.), (FRAC_PI_2, 0., 0.)))
							.named("Leg_[L1]_l2_vis_1")
							.materialized(material_l2.clone()),
					)
					.add_collider(
						Collision::builder(geom_leg_l2.clone())
							.transformed(Transform::new((0., 5., 0.), (FRAC_PI_2, 0., 0.)))
							.named("Leg_[L1]_l2_col_1"),
					),
			)
			.unwrap();

		let left_leg_builder = left_leg_1_tree.yank_link("Leg_[L1]_l1").unwrap();
		let right_leg_builder_x = left_leg_builder.mirror(MirrorAxis::X);

		assert_eq!(
			right_leg_builder_x,
			Chained(LinkBuilder {
				name: "Leg_[L1]_l1".into(),
				visuals: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					transform: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					transform: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					transform: JointTransformMode::Direct(Transform {
						translation: Some((0., 3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visuals: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							transform: Some(Transform {
								translation: Some((0., -5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							transform: Some(Transform {
								translation: Some((0., -5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
						}],
						..Default::default()
					}),
					..Default::default()
				},],
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_x.mirror(MirrorAxis::X));

		let right_leg_builder_y = left_leg_builder.mirror(MirrorAxis::Y);

		assert_eq!(
			right_leg_builder_y,
			Chained(LinkBuilder {
				name: "Leg_[L1]_l1".into(),
				visuals: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					transform: Some(Transform {
						translation: Some((0., -1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					transform: Some(Transform {
						translation: Some((0., -1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					transform: JointTransformMode::Direct(Transform {
						translation: Some((0., -3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visuals: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							transform: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							transform: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
						}],
						..Default::default()
					}),
					..Default::default()
				},],
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_y.mirror(MirrorAxis::Y));

		let right_leg_builder_z = left_leg_builder.mirror(MirrorAxis::Z);

		assert_eq!(
			right_leg_builder_z,
			Chained(LinkBuilder {
				name: "Leg_[L1]_l1".into(),
				visuals: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					transform: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					transform: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					transform: JointTransformMode::Direct(Transform {
						translation: Some((0., 3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visuals: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							transform: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							transform: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
						}],
						..Default::default()
					}),
					..Default::default()
				},],
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_z.mirror(MirrorAxis::Z));
	}
}
