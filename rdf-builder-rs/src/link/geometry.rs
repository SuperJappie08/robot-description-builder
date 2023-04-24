mod box_geometry;
mod cylinder_geometry;
mod geometry_shape_data;
mod sphere_geometry;

pub use box_geometry::BoxGeometry;
pub use cylinder_geometry::CylinderGeometry;
pub use sphere_geometry::SphereGeometry;

pub use geometry_shape_data::GeometryShapeData;

// #[cfg(feature = "xml")]
// use quick_xml::{name::QName, events::attributes::Attribute};

use std::fmt::Debug;

// #[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

use self::geometry_shape_data::GeometryShapeContainer;

pub trait GeometryInterface: Debug + ToURDF {
	fn volume(&self) -> f32;
	fn surface_area(&self) -> f32;
	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send>;

	/// Get's the untransformed boundingbox size of the geometry from it's center. (X, Y, Z)
	fn bounding_box(&self) -> (f32, f32, f32);

	/// Attemps to get a `GeometryShapeConatainer`
	///
	/// This fails when the data is not representable as a shape container.
	fn try_get_shape(&self) -> Result<GeometryShapeContainer, ()>;
}

// impl PartialEq for Box<dyn GeometryInterface + Sync + Send> {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.volume() == other.volume() && self.surface_area() == other.surface_area()
// 	}
// }

impl PartialEq for (dyn GeometryInterface + Sync + Send) {
	fn eq(&self, other: &Self) -> bool {
		// Should probably just get shape data
		self.volume() == other.volume()
			&& self.surface_area() == other.surface_area()
			&& self.bounding_box() == other.bounding_box()
	}
}

// TODO: Is this ever used?
impl From<&(dyn GeometryInterface + Sync + Send)> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: &(dyn GeometryInterface + Sync + Send)) -> Self {
		value.boxed_clone()
	}
}

// impl Clone for Box<dyn GeometryInterface> {
// 	fn clone(&self) -> Self {
// 		self.boxed_clone()
// 	}
// }

// impl ToURDF for Box<dyn GeometryInterface + Sync + Send> {
//     fn to_urdf(
// 		&self,
// 		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
// 		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
// 	) -> Result<(), quick_xml::Error> {
//         match self.get_data() {
//             GeometryData::Box(geometry) => writer
// 			.create_element("box")
// 			.with_attribute(Attribute {
// 				key: QName(b"size"),
// 				value: format!("{} {} {}", geometry.side1, geometry.side2, geometry.side3)
// 					.as_bytes()
// 					.into(),
// 			})
// 			.write_empty()?,
//             GeometryData::Cylinder(geometry) => todo!(),
//             GeometryData::Sphere(geometry) => todo!(),
//         };
// 		Ok(())
//     }
// }

#[cfg(not(feature = "urdf"))]
mod not_urdf {

	use super::{BoxGeometry, CylinderGeometry, SphereGeometry, ToURDF};

	impl ToURDF for BoxGeometry {}
	impl ToURDF for CylinderGeometry {}
	impl ToURDF for SphereGeometry {}
}
