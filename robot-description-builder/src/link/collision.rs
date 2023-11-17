#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use super::{builder::CollisionBuilder, geometry::GeometryInterface};
use crate::{identifiers::GroupID, transform::Transform};

/// A `Collision` geometry for a `Link`.
///
/// This struct holds one of (the many) Colliders for the associated [`Link`].
///  It can be constructed via the [`CollisionBuilder`] (accessable via the [`builder`](Self::builder) method) and [added while building the `Link`](crate::link::builder::LinkBuilder::add_collider).
/// It contains the following data:
/// - **[`geometry`](crate::link_data::geometry)**: The geometry used for collision checking[^mesh-warning].
/// - **[`transform`](crate::Transform)** (Optional): The transform from the [`Link`] frame to the `geometry`.
/// - **`name`** (Optional): The identifiers/names of this collision element. For practical purposes, it is recommended to use unique identifiers/names.
///
/// [^mesh-warning]: **WARNING:** It is not recommended to use high-detail meshes for collision geometries, since this will slow down the collision checking process.
/// Also, keep in mind, that some simulators only support the use of convex meshes for collisions, if at all.
///
/// [`Link`]: crate::link::Link
#[derive(Debug)]
pub struct Collision {
	// TODO: Add export option which generates name.
	/// The [_string identifier_](crate::identifiers) or name of this collision element.
	///
	/// For practical purposes, it is recommended to use unique identifiers/names.
	pub(crate) name: Option<String>,
	/// The transform from the origin of the parent `Link` to the origin of this `Collision`.
	///
	/// This is the reference for the placement of the `geometry`.
	///
	/// In URDF this field is refered to as `<origin>`.
	pub(crate) transform: Option<Transform>,
	/// The geometry of this Collision element.
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl Collision {
	/// Create a new [`CollisionBuilder`] with the specified [`Geometry`](crate::link_data::geometry).
	pub fn builder(
		geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>,
	) -> CollisionBuilder {
		CollisionBuilder::new(geometry)
	}

	/// Gets an optional reference to the `name` of this `Collision`.
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// Gets an optional reference to the `transform` of this `Collision`.
	pub fn transform(&self) -> Option<&Transform> {
		self.transform.as_ref()
	}

	/// Gets a reference to the `geometry` of this `Collision`.
	pub fn geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}

	/// Recreates the [`CollisionBuilder`], which was used to create this `Collision`.
	pub fn rebuild(&self) -> CollisionBuilder {
		CollisionBuilder {
			name: self.name.clone(),
			transform: self.transform,
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

		element.write_inner_content(|writer| -> quick_xml::Result<()> {
			if let Some(transform) = self.transform() {
				transform.to_urdf(writer, urdf_config)?
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
		self.name == other.name
			&& self.transform == other.transform
			&& *self.geometry == *other.geometry
	}
}

impl Clone for Collision {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			transform: self.transform,
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
					.transformed(Transform::new((4., 6.78, 1.), (PI, 2. * PI, 0.))),
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
					.transformed(Transform::new_translation(5.4, 9.1, 7.8)),
				String::from(
					r#"<collision name="some_col"><origin xyz="5.4 9.1 7.8"/><geometry><cylinder radius="4.5" length="75.35"/></geometry></collision>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
