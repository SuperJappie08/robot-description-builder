#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::{link::geometry::GeometryInterface, transform_data::TransformData};

#[derive(Debug)]
pub struct Collision {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	origin: Option<TransformData>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl Collision {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		origin: Option<TransformData>,
		geometry: Box<dyn GeometryInterface + Sync + Send>,
	) -> Self {
		Self {
			name,
			origin,
			geometry,
		}
	}

	pub fn get_name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn get_origin(&self) -> Option<&TransformData> {
		self.origin.as_ref()
	}

	pub fn get_geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Collision {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("collision");
		if let Some(name) = self.get_name() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.as_bytes().into(),
			});
		}

		element.write_inner_content(|writer| {
			if let Some(origin) = self.get_origin().clone() {
				origin.to_urdf(writer, urdf_config)?
			}

			self.get_geometry().to_urdf(writer, urdf_config)?;
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Collision {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& (&self.geometry == &other.geometry)
	}
}

impl Clone for Collision {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin.clone(),
			geometry: self.geometry.boxed_clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;

	use crate::{
		link::{
			collision::Collision,
			geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
		},
		transform_data::TransformData,
	};

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::*;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		#[test]
		fn no_name_no_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(
				Collision::new(None, None, BoxGeometry::new(1.0, 2.0, 3.0).into())
					.to_urdf(&mut writer, &URDFConfig::default())
					.is_ok()
			);

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<collision><geometry><box size="1 2 3"/></geometry></collision>"#)
			)
		}

		#[test]
		fn name_no_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Collision::new(
				Some("myLink_col".to_owned()),
				None,
				CylinderGeometry::new(9., 6.258).into()
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<collision name="myLink_col"><geometry><cylinder radius="9" length="6.258"/></geometry></collision>"#
				)
			)
		}

		#[test]
		fn no_name_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Collision::new(
				None,
				Some(TransformData {
					translation: Some((4., 6.78, 1.)),
					rotation: Some((PI, 2. * PI, 0.))
				}),
				SphereGeometry::new(3.).into()
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<collision><origin xyz="4 6.78 1" rpy="3.1415927 6.2831855 0"/><geometry><sphere radius="3"/></geometry></collision>"#
				)
			)
		}

		#[test]
		fn name_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Collision::new(
				Some("some_col".into()),
				Some(TransformData {
					translation: Some((5.4, 9.1, 7.8)),
					..Default::default()
				}),
				CylinderGeometry::new(4.5, 75.35).into()
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<collision name="some_col"><origin xyz="5.4 9.1 7.8"/><geometry><cylinder radius="4.5" length="75.35"/></geometry></collision>"#
				)
			)
		}
	}
}
