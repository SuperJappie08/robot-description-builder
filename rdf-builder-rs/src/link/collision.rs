#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use super::{builder::CollisionBuilder, geometry::GeometryInterface};
use crate::{identifiers::GroupID, transform::Transform};

#[derive(Debug)]
pub struct Collision {
	/// TODO: Figure out if I want to keep the name optional?.
	pub(crate) name: Option<String>,
	pub(crate) origin: Option<Transform>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl Collision {
	pub fn builder(
		geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>,
	) -> CollisionBuilder {
		CollisionBuilder::new(geometry)
	}

	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn origin(&self) -> Option<&Transform> {
		self.origin.as_ref()
	}

	pub fn geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}

	pub fn rebuild(&self) -> CollisionBuilder {
		CollisionBuilder {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
		}
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
		if let Some(name) = self.name() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.display().as_bytes().into(),
			});
		}

		element.write_inner_content(|writer| {
			if let Some(origin) = self.origin() {
				origin.to_urdf(writer, urdf_config)?
			}

			self.geometry()
				.shape_container()
				.to_urdf(writer, urdf_config)?;
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Collision {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.origin == other.origin && *self.geometry == *other.geometry
	}
}

impl Clone for Collision {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;
	use test_log::test;

	use crate::{
		link::{
			builder::CollisionBuilder,
			collision::Collision,
			geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
		},
		transform::Transform,
	};

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, *};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		fn test_to_urdf_collision(
			collision: CollisionBuilder,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(collision.build().to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		fn no_name_no_origin() {
			test_to_urdf_collision(
				Collision::builder(BoxGeometry::new(1.0, 2.0, 3.0)),
				String::from(r#"<collision><geometry><box size="1 2 3"/></geometry></collision>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn name_no_origin() {
			test_to_urdf_collision(
				Collision::builder(CylinderGeometry::new(9., 6.258)).named("myLink_col"),
				String::from(
					r#"<collision name="myLink_col"><geometry><cylinder radius="9" length="6.258"/></geometry></collision>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn no_name_origin() {
			test_to_urdf_collision(
				Collision::builder(SphereGeometry::new(3.))
					.tranformed(Transform::new((4., 6.78, 1.), (PI, 2. * PI, 0.))),
				String::from(
					r#"<collision><origin xyz="4 6.78 1" rpy="3.1415927 6.2831855 0"/><geometry><sphere radius="3"/></geometry></collision>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn name_origin() {
			test_to_urdf_collision(
				Collision::builder(CylinderGeometry::new(4.5, 75.35))
					.named("some_col")
					.tranformed(Transform::new_translation(5.4, 9.1, 7.8)),
				String::from(
					r#"<collision name="some_col"><origin xyz="5.4 9.1 7.8"/><geometry><cylinder radius="4.5" length="75.35"/></geometry></collision>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
