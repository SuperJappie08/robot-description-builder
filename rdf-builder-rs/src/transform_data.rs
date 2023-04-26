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

	/// TODO: There are multiple ways to mirror. Pick one that works and makes sense, or options
	///  - Mirror full thing and thereby invert joint axis
	///  - Mirror Only first joint
	pub fn mirrored(&self, axis: MirrorAxis) -> Self {
		// I doubt this check makes sense
		if self.contains_some() {
			match axis {
				MirrorAxis::X => todo!(),
				MirrorAxis::Y => todo!(),
				MirrorAxis::Z => todo!(),
			}
		} else {
			// Coping self
			*self
		}
	}
}

// FIXME: TODO: MAYBE UUSE ndarray instead?
// Or euclid or euler

// impl std::ops::Add for Transform {
// 	type Output = Transform;

// 	fn add(self, rhs: Self) -> Self::Output {
// 		match (self.contains_some(), rhs.contains_some()) {
// 			(true, true) => Self {
// 				translation: match (self.translation, rhs.translation) {
// 					(Some(lhs), Some(rhs)) => Some((lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)),
// 					(own_translation, None) => own_translation,
// 					(None, rhs_translation) => rhs_translation,
// 				},
// 				rotation: match (self.rotation, rhs.rotation) {
// 					(Some(lhs), Some(rhs)) => Some((lhs.0 + rhs.0, lhs.1 + rhs.1, lhs.2 + rhs.2)),
// 					(own_rot, None) => own_rot,
// 					(None, rhs_rot) => rhs_rot,
// 				},
// 			},
// 			(false, true) => rhs,
// 			(_, false) => self,
// 		}
// 	}
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MirrorAxis {
	X,
	Y,
	Z,
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
	use test_log::test;

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
