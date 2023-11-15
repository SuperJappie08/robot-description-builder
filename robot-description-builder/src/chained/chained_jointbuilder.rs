use std::sync::Weak;

use nalgebra::Matrix3;

use super::{ChainableBuilder, Chained};
use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{BuildJointChain, Joint, JointBuilder},
	link::{builder::LinkBuilder, Link, LinkShapeData},
	transform::{Mirror, MirrorAxis},
	utils::{ArcLock, WeakLock},
};

impl Chained<JointBuilder> {
	/// TODO: TEST
	/// TODO: DOC
	pub fn mirror(&self, axis: MirrorAxis) -> Chained<JointBuilder> {
		let mirror_matrix: Matrix3<_> = axis.into();
		Chained(self.0.mirrored(&mirror_matrix))
	}
}

impl ChainableBuilder for JointBuilder {
	fn has_chain(&self) -> bool {
		self.child.is_some()
	}
}

impl BuildJointChain for Chained<JointBuilder> {
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_link: &WeakLock<Link>,
		parent_link_size_data: LinkShapeData,
	) -> ArcLock<Joint> {
		self.0.build_chain(tree, parent_link, parent_link_size_data)
	}
}

impl<JointB, LinkB> From<(JointB, LinkB)> for Chained<JointBuilder>
where
	JointB: Into<JointBuilder>,
	LinkB: Into<LinkBuilder>,
{
	fn from(value: (JointB, LinkB)) -> Self {
		Chained(JointBuilder {
			child: Some(value.1.into()),
			..value.0.into()
		})
	}
}

#[cfg(test)]
mod tests {
	use super::{Chained, JointBuilder};
	use std::f32::consts::FRAC_PI_2;
	use test_log::test;

	use crate::{
		cluster_objects::KinematicInterface,
		joint::{joint_data::LimitData, JointTransformMode, JointType, SmartJointBuilder},
		link::{
			link_data::{
				geometry::{BoxGeometry, CylinderGeometry},
				Collision, Visual,
			},
			Link,
		},
		linkbuilding::{CollisionBuilder, LinkBuilder, VisualBuilder},
		material::MaterialDescriptor,
		transform::{MirrorAxis, Transform},
	};

	#[test]
	fn mirror_simple_1() {
		let material_l1 = MaterialDescriptor::new_rgb(1., 0., 0.).named("Leg_l1");
		let material_l2 = MaterialDescriptor::new_rgb(0., 1., 0.).named("Leg_l2");
		let geom_leg_l1 = BoxGeometry::new(2., 3., 1.);
		let geom_leg_l2 = CylinderGeometry::new(1., 10.);

		let left_leg_1_tree = Link::builder("root").build_tree();

		left_leg_1_tree
			.get_root_link()
			.write()
			.unwrap()
			.try_attach_child(
				SmartJointBuilder::new_revolute("Leg_[L1]_j0")
					.add_transform(Transform::new_translation(1., 2., 3.))
					.with_axis((0., 1., 0.))
					.with_limit(0.1, 100.)
					.set_lower_limit(-0.5)
					.set_upper_limit(900.),
				{
					let tree = Link::builder("Leg_[L1]_l1")
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

					tree.get_root_link()
						.try_write()
						.unwrap()
						.try_attach_child(
							SmartJointBuilder::new_fixed("Leg_[L1]_j1")
								.add_transform(Transform::new((0., 3., 0.), (0., 0., FRAC_PI_2))),
							Link::builder("Leg_[L1]_l2")
								.add_visual(
									Visual::builder(geom_leg_l2.clone())
										.transformed(Transform::new(
											(0., 5., 0.),
											(FRAC_PI_2, 0., 0.),
										))
										.named("Leg_[L1]_l2_vis_1")
										.materialized(material_l2.clone()),
								)
								.add_collider(
									Collision::builder(geom_leg_l2.clone())
										.transformed(Transform::new(
											(0., 5., 0.),
											(FRAC_PI_2, 0., 0.),
										))
										.named("Leg_[L1]_l2_col_1"),
								),
						)
						.unwrap();
					tree.yank_link("Leg_[L1]_l1").unwrap()
				},
			)
			.unwrap();

		let left_leg_builder = left_leg_1_tree.yank_joint("Leg_[L1]_j0").unwrap();
		let right_leg_builder_x = left_leg_builder.mirror(MirrorAxis::X);

		assert_eq!(
			right_leg_builder_x,
			Chained(JointBuilder {
				name: "Leg_[L1]_j0".into(),
				joint_type: JointType::Revolute,
				transform: JointTransformMode::Direct(Transform::new_translation(-1., 2., 3.)),
				limit: Some(LimitData {
					lower: Some(-0.5),
					upper: Some(900.),
					effort: 0.1,
					velocity: 100.
				}),
				axis: Some((0., -1., 0.)),
				child: Some(LinkBuilder {
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
				}),
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_x.mirror(MirrorAxis::X));

		let right_leg_builder_y = left_leg_builder.mirror(MirrorAxis::Y);

		assert_eq!(
			right_leg_builder_y,
			Chained(JointBuilder {
				name: "Leg_[L1]_j0".into(),
				joint_type: JointType::Revolute,
				transform: JointTransformMode::Direct(Transform::new_translation(1., -2., 3.)),
				limit: Some(LimitData {
					lower: Some(-0.5),
					upper: Some(900.),
					effort: 0.1,
					velocity: 100.
				}),
				axis: Some((0., 1., 0.)),
				child: Some(LinkBuilder {
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
				}),
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_y.mirror(MirrorAxis::Y));

		let right_leg_builder_z = left_leg_builder.mirror(MirrorAxis::Z);

		assert_eq!(
			right_leg_builder_z,
			Chained(JointBuilder {
				name: "Leg_[L1]_j0".into(),
				joint_type: JointType::Revolute,
				transform: JointTransformMode::Direct(Transform::new_translation(1., 2., -3.)),
				limit: Some(LimitData {
					lower: Some(-0.5),
					upper: Some(900.),
					effort: 0.1,
					velocity: 100.
				}),
				axis: Some((0., -1., 0.)),
				child: Some(LinkBuilder {
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
				}),
				..Default::default()
			})
		);

		assert_eq!(left_leg_builder, right_leg_builder_z.mirror(MirrorAxis::Z));
	}

	// #[test]
	// fn chained_escaping() {
	// 	let tree = Link::builder("root-link").build_tree();

	// 	tree.get_root_link()
	// 		.try_write()
	// 		.unwrap()
	// 		.try_attach_child(
	// 			Link::builder("child-link")
	// 				.add_visual(Visual::builder(BoxGeometry::new(3., 4., 5.))),
	// 			SmartJointBuilder::new_continuous("jointy"),
	// 		)
	// 		.unwrap();

	// 	let mut builder = tree.yank_root();

	// 	builder.add_visual(Visual::builder(BoxGeometry::new(2.,3.,4.)));
	// }
}
