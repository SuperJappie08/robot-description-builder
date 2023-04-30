use itertools::Itertools;
use nalgebra::{vector, Matrix3, Rotation3, Vector3};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Transform {
	pub translation: Option<(f32, f32, f32)>,
	pub rotation: Option<(f32, f32, f32)>,
}

impl Transform {
	pub fn new_translation(x: f32, y: f32, z: f32) -> Self {
		Self {
			translation: Some((x, y, z)),
			..Default::default()
		}
	}

	pub fn new_rotation(r: f32, p: f32, y: f32) -> Self {
		Self {
			rotation: Some((r, p, y)),
			..Default::default()
		}
	}

	pub fn new(xyz: (f32, f32, f32), rpy: (f32, f32, f32)) -> Self {
		Self {
			translation: Some(xyz),
			rotation: Some(rpy),
		}
	}

	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// # Example
	/// ```rust
	/// # use rdf_builder_rs::Transform;
	/// assert!(Transform {
	///     translation: Some((1., 2., 3.)),
	///     rotation: Some((4., 5., 6.))
	/// }
	/// .contains_some());
	///
	/// assert!(Transform {
	///     translation: Some((1., 2., 3.)),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(Transform {
	///     rotation: Some((4., 5., 6.)),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!Transform::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.translation.is_some() || self.rotation.is_some()
	}

	pub(crate) fn mirror(&self, mirror_matrix: &Matrix3<f32>) -> (Self, Matrix3<f32>) {
		(
			Transform {
				translation: self.translation.as_ref().map(|(x, y, z)| {
					let old_translation = vector![*x, *y, *z];
					(mirror_matrix * old_translation)
						.component_mul(&Vector3::from_iterator(old_translation.iter().map(|val| {
							if val.is_normal() {
								1.
							} else {
								0.
							}
						}))) // TODO: Perfomance enhancements are probably possible.
						.iter()
						.copied()
						.collect_tuple()
						.unwrap() // Unwrapping here to ensure that we collect to a Tuple3 | TODO: Change to expect? or remove
				}),
				rotation: self.rotation,
			},
			match self.rotation.as_ref() {
				Some(rpy) => {
					Rotation3::from_euler_angles(rpy.0, rpy.1, rpy.2)
						* mirror_matrix * Rotation3::from_euler_angles(rpy.0, rpy.1, rpy.2).inverse()
				}
				None => *mirror_matrix,
			},
		)
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MirrorAxis {
	X,
	Y,
	Z,
}

impl From<MirrorAxis> for Matrix3<f32> {
	fn from(value: MirrorAxis) -> Self {
		let diag = match value {
			MirrorAxis::X => (-1., 1., 1.),
			MirrorAxis::Y => (1., -1., 1.),
			MirrorAxis::Z => (1., 1., -1.),
		};

		Matrix3::from_diagonal(&Vector3::new(diag.0, diag.1, diag.2))
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Transform {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("origin");
		if let Some(translation) = self.translation {
			element = element.with_attribute(Attribute {
				key: QName(b"xyz"),
				value: format!("{} {} {}", translation.0, translation.1, translation.2)
					.as_bytes()
					.into(),
			})
		}

		if let Some(rotation) = self.rotation {
			element = element.with_attribute(Attribute {
				key: QName(b"rpy"),
				value: format!("{} {} {}", rotation.0, rotation.1, rotation.2)
					.as_bytes()
					.into(),
			});
		}

		element.write_empty()?;
		Ok(())
	}
}

impl From<Transform> for crate::joint::JointTransformMode {
	fn from(value: Transform) -> Self {
		Self::Direct(value)
	}
}

#[cfg(test)]
mod tests {
	use crate::transform_data::Transform;
	use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
	use test_log::test;

	mod mirror {
		use super::{test, *};
		use crate::transform_data::MirrorAxis;
		use nalgebra::{matrix, vector, Matrix3};

		fn test_mirror(
			transform: Transform,
			mirror_axis: MirrorAxis,
			result: (Transform, Matrix3<f32>),
		) {
			assert_eq!(transform.mirror(&mirror_axis.into()), result)
		}

		fn test_all_mirrors(transform: Transform, results: [(Transform, Matrix3<f32>); 3]) {
			results
				.into_iter()
				.enumerate()
				.map(|(index, result)| {
					(
						match index {
							0 => MirrorAxis::X,
							1 => MirrorAxis::Y,
							2 => MirrorAxis::Z,
							_ => unreachable!(),
						},
						result,
					)
				})
				.for_each(|(mirror_axis, result)| test_mirror(transform, mirror_axis, result))
		}

		fn test_all_mirrors_angle_var(
			transform: Transform,
			angle: f32,
			results: [(Transform, [Matrix3<f32>; 3]); 3],
		) {
			for i in 0..2 {
				let rotation = match i {
					0 => (angle, 0., 0.),
					1 => (0., angle, 0.),
					2 => (0., 0., angle),
					_ => unreachable!(),
				};

				test_all_mirrors(
					Transform {
						rotation: Some(rotation),
						..transform.clone()
					},
					[
						(
							Transform {
								rotation: Some(rotation),
								..results[0].0
							},
							results[0].1[i],
						),
						(
							Transform {
								rotation: Some(rotation),
								..results[1].0
							},
							results[1].1[i],
						),
						(
							Transform {
								rotation: Some(rotation),
								..results[2].0
							},
							results[2].1[i],
						),
					],
				)
			}
		}

		#[test]
		fn uniaxial_no_rotation() {
			// X
			test_all_mirrors(
				Transform::new_translation(2., 0., 0.),
				[
					(
						Transform {
							translation: Some((-2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			// Y
			test_all_mirrors(
				Transform::new_translation(0., 0.5, 0.),
				[
					(
						Transform {
							translation: Some((0., 0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((0., -0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			// Z
			test_all_mirrors(
				Transform::new_translation(0., 0., -900.),
				[
					(
						Transform {
							translation: Some((0., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0., 900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);
		}

		#[test]
		#[ignore = "Is this necessary?"]
		fn uniaxial_unirotation() {
			test_all_mirrors_angle_var(
				Transform::new_translation(2., 0., 0.),
				FRAC_PI_2,
				[
					(
						Transform::new_translation(-2., 0., 0.),
						[
							matrix![
								-1., 0., 0.;
								0., 1., 0.;
								0., 0., 1.;
							],
							Matrix3::from_diagonal(&vector![-1., 1., 1.]),
							Matrix3::from_diagonal(&vector![-1., 1., 1.]),
						],
					),
					(
						Transform::new_translation(2., 0., 0.),
						[
							matrix![
								1., 0. ,0.;
								0., 1., 0.;
								0. ,0. ,-1.;
							],
							Matrix3::from_diagonal(&vector![1., -1., 1.]),
							Matrix3::from_diagonal(&vector![1., -1., 1.]),
						],
					),
					(
						Transform::new_translation(2., 0., 0.),
						[
							Matrix3::from_diagonal(&vector![1., 1., -1.]),
							Matrix3::from_diagonal(&vector![1., 1., -1.]),
							Matrix3::from_diagonal(&vector![1., 1., -1.]),
						],
					),
				],
			);

			test_all_mirrors(
				Transform::new_translation(2., 0., 0.),
				[
					(
						Transform {
							translation: Some((-2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((2., 0., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			// Y
			test_all_mirrors(
				Transform::new((0., 0.5, 0.), (FRAC_PI_2, 0., FRAC_PI_4)),
				[
					(
						Transform {
							translation: Some((0., 0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((0., -0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0.5, 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			// Z
			test_all_mirrors(
				Transform::new_translation(0., 0., -900.),
				[
					(
						Transform {
							translation: Some((0., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0., 900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);
		}

		#[test]
		fn multiaxial_no_rotation() {
			test_all_mirrors(
				Transform::new_translation(2., 3., 0.),
				[
					(
						Transform {
							translation: Some((-2., 3., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((2., -3., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((2., 3., 0.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			test_all_mirrors(
				Transform::new_translation(0., 0.5, 7.),
				[
					(
						Transform {
							translation: Some((0., 0.5, 7.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((0., -0.5, 7.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((0., 0.5, -7.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			test_all_mirrors(
				Transform::new_translation(120., 0., -900.),
				[
					(
						Transform {
							translation: Some((-120., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((120., 0., -900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((120., 0., 900.)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			);

			test_all_mirrors(
				Transform::new_translation(3., 4., 5.0005),
				[
					(
						Transform {
							translation: Some((-3., 4., 5.0005)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![-1., 1., 1.]),
					),
					(
						Transform {
							translation: Some((3., -4., 5.0005)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., -1., 1.]),
					),
					(
						Transform {
							translation: Some((3., 4., -5.0005)),
							rotation: None,
						},
						Matrix3::from_diagonal(&vector![1., 1., -1.]),
					),
				],
			)
		}

		#[test]
		#[ignore = "Is this necessary?"]
		fn multiaxial_rotation() {
			todo!()
		}
	}

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, Transform};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		fn test_to_urdf_transform(transform: Transform, result: String, urdf_config: &URDFConfig) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(transform.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();
			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		fn translation_only() {
			test_to_urdf_transform(
				Transform {
					translation: Some((1.2, 2.3, 3.4)),
					..Default::default()
				},
				String::from(r#"<origin xyz="1.2 2.3 3.4"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn rotation_only() {
			test_to_urdf_transform(
				Transform {
					rotation: Some((1.2, 2.3, 3.4)),
					..Default::default()
				},
				String::from(r#"<origin rpy="1.2 2.3 3.4"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn translation_rotatation() {
			test_to_urdf_transform(
				Transform {
					translation: Some((1.23, 2.34, 3.45)),
					rotation: Some((4.56, 5.67, 6.78)),
				},
				String::from(r#"<origin xyz="1.23 2.34 3.45" rpy="4.56 5.67 6.78"/>"#),
				&URDFConfig::default(),
			);
		}
	}
}
