use nalgebra::Matrix3;

use crate::transform::{Mirror, Transform};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

// TODO: Maybe rename to Inertial?
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Inertial {
	/// The transform from the parent [`Link`](super::Link)'s frame to the frame of the `InertialData`.
	///
	/// This is the reference for the placement of the center of mass and the moments of inertia.
	///
	/// In URDF this field is refered to as `<origin>`.
	pub transform: Option<Transform>,
	/// The mass of the current [`Link`](super::Link).
	pub mass: f32,
	/// The Moments of ineria around the x axis.
	pub ixx: f32,
	/// Product of inertia element xy.
	pub ixy: f32,
	/// Product of inertia element xz.
	pub ixz: f32,
	/// The Moments of ineria around the y axis.
	pub iyy: f32,
	/// Product of inertia element yz.
	pub iyz: f32,
	/// The Moments of ineria around the z axis.
	pub izz: f32,
}

impl Mirror for Inertial {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		Self {
			transform: self
				.transform
				.as_ref()
				.map(|transform| transform.mirrored(mirror_matrix)),
			..*self
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Inertial {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("inertial");
		element.write_inner_content(|writer| -> quick_xml::Result<()> {
			if let Some(transform) = &self.transform {
				transform.to_urdf(writer, urdf_config)?;
			}

			writer
				.create_element("mass")
				.with_attribute(Attribute {
					key: QName(b"value"),
					value: format!("{}", self.mass).as_bytes().into(),
				})
				.write_empty()?;

			writer
				.create_element("inertia")
				.with_attributes([
					("ixx", self.ixx.to_string().as_str()),
					("ixy", self.ixy.to_string().as_str()),
					("ixz", self.ixz.to_string().as_str()),
					("iyy", self.iyy.to_string().as_str()),
					("iyz", self.iyz.to_string().as_str()),
					("izz", self.izz.to_string().as_str()),
				])
				.write_empty()?;

			Ok(())
		})?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::Inertial;
	use test_log::test;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, *};

		use crate::{
			to_rdf::to_urdf::{ToURDF, URDFConfig},
			transform::Transform,
		};

		use std::io::Seek;

		fn test_to_urdf_inertial(
			inertial_data: Inertial,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(inertial_data.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		fn no_transform() {
			test_to_urdf_inertial(
				Inertial {
					mass: 0.12,
					ixx: 1.23,
					ixy: 2.34,
					ixz: 3.45,
					iyy: 4.56,
					iyz: 5.67,
					izz: 6.78,
					..Default::default()
				},
				String::from(
					r#"<inertial><mass value="0.12"/><inertia ixx="1.23" ixy="2.34" ixz="3.45" iyy="4.56" iyz="5.67" izz="6.78"/></inertial>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn with_transform() {
			test_to_urdf_inertial(
				Inertial {
					transform: Some(Transform {
						translation: Some((10.1, 20.2, 30.3)),
						..Default::default()
					}),
					mass: 100.,
					ixx: 123.,
					iyy: 456.,
					izz: 789.,
					..Default::default()
				},
				String::from(
					r#"<inertial><origin xyz="10.1 20.2 30.3"/><mass value="100"/><inertia ixx="123" ixy="0" ixz="0" iyy="456" iyz="0" izz="789"/></inertial>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
