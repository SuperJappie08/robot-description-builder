use std::sync::Weak;

use nalgebra::Matrix3;

use crate::{
	chained::{ChainableBuilder, Chained},
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	linkbuilding::{BuildLink, LinkBuilder},
	transform_data::MirrorAxis,
	ArcLock, Joint, KinematicInterface, Link, WeakLock,
};

impl Chained<LinkBuilder> {
	/// TODO: More tests
	/// TODO: DOC
	pub fn mirror(&self, axis: MirrorAxis) -> Chained<LinkBuilder> {
		let mirror_matrix: Matrix3<_> = axis.into();
		Chained(self.0.mirror(&mirror_matrix))
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

/// Since Link's can end a chain, a `LinkBuilder` can always be converted to a `Chained<LinkBuilder>`
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
		// FIXME: Is unwrap Ok Here?
		// FIXME: Maybe use the non-blocking read, for production?
		Self(value.get_root_link().try_read().unwrap().yank())
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
		material_mod::MaterialBuilder,
		transform_data::MirrorAxis,
		Chained, JointBuilder, JointType, SmartJointBuilder, Transform,
	};

	#[test]
	fn mirror_simple_1() {
		let material_l1 = MaterialBuilder::new_rgb(1., 0., 0.).named("Leg_l1");
		let material_l2 = MaterialBuilder::new_rgb(0., 1., 0.).named("Leg_l2");
		let geom_leg_l1 = BoxGeometry::new(2., 3., 1.);
		let geom_leg_l2 = CylinderGeometry::new(1., 10.);

		let left_leg_1_tree = Link::builder("Leg_[L1]_l1")
			.add_visual(
				Visual::builder(geom_leg_l1.clone())
					.tranformed(Transform::new_translation(0., 1.5, 0.))
					.named("Leg_[L1]_l1_vis_1")
					.material(material_l1.clone()),
			)
			.add_collider(
				Collision::builder(geom_leg_l1.clone())
					.tranformed(Transform::new_translation(0., 1.5, 0.))
					.named("Leg_[L1]_l1_col_1"),
			)
			.build_tree();

		left_leg_1_tree
			.get_root_link()
			.try_write()
			.unwrap()
			.try_attach_child(
				Link::builder("Leg_[L1]_l2")
					.add_visual(
						Visual::builder(geom_leg_l2.clone())
							.tranformed(Transform::new((0., 5., 0.), (FRAC_PI_2, 0., 0.)))
							.named("Leg_[L1]_l2_vis_1")
							.material(material_l2.clone()),
					)
					.add_collider(
						Collision::builder(geom_leg_l2.clone())
							.tranformed(Transform::new((0., 5., 0.), (FRAC_PI_2, 0., 0.)))
							.named("Leg_[L1]_l2_col_1"),
					),
				SmartJointBuilder::new_fixed("Leg_[L1]_j1")
					.add_transform(Transform::new((0., 3., 0.), (0., 0., FRAC_PI_2))),
			)
			.unwrap();

		let left_leg_builder = left_leg_1_tree.yank_link("Leg_[L1]_l1").unwrap();
		let right_leg_builder_x = left_leg_builder.mirror(MirrorAxis::X);

		assert_eq!(
			right_leg_builder_x,
			Chained(LinkBuilder {
				name: "Leg_[L1]_l1".into(),
				visual_builders: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					origin: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					origin: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					origin: JointTransformMode::Direct(Transform {
						translation: Some((0., 3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visual_builders: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							origin: Some(Transform {
								translation: Some((0., -5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							origin: Some(Transform {
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
				visual_builders: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					origin: Some(Transform {
						translation: Some((0., -1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					origin: Some(Transform {
						translation: Some((0., -1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					origin: JointTransformMode::Direct(Transform {
						translation: Some((0., -3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visual_builders: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							origin: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							origin: Some(Transform {
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
				visual_builders: vec![VisualBuilder {
					name: Some("Leg_[L1]_l1_vis_1".into()),
					origin: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
					material_description: Some(material_l1.clone())
				}],
				colliders: vec![CollisionBuilder {
					name: Some("Leg_[L1]_l1_col_1".into()),
					origin: Some(Transform {
						translation: Some((0., 1.5, 0.)),
						rotation: None
					}),
					geometry: BoxGeometry::new(2., 3., 1.).into(),
				}],
				joints: vec![JointBuilder {
					name: "Leg_[L1]_j1".into(),
					joint_type: JointType::Fixed,
					origin: JointTransformMode::Direct(Transform {
						translation: Some((0., 3., 0.)),
						rotation: Some((0., 0., FRAC_PI_2))
					}),
					child: Some(LinkBuilder {
						name: "Leg_[L1]_l2".into(),
						visual_builders: vec![VisualBuilder {
							name: Some("Leg_[L1]_l2_vis_1".into()),
							origin: Some(Transform {
								translation: Some((0., 5., 0.)),
								rotation: Some((FRAC_PI_2, 0., 0.))
							}),
							geometry: CylinderGeometry::new(1., 10.).into(),
							material_description: Some(material_l2.clone())
						}],
						colliders: vec![CollisionBuilder {
							name: Some("Leg_[L1]_l2_col_1".into()),
							origin: Some(Transform {
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