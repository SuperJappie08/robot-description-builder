use super::{
	mesh_geometry::MeshGeometry, BoxGeometry, CylinderGeometry, GeometryInterface, SphereGeometry,
};
use crate::transform::Transform;

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

#[derive(Debug, PartialEq, Clone)]
pub struct GeometryShapeData {
	/// The transform from the frame/origin of the parent `Link` to the frame/origin of this `Geometry`.
	///
	/// This is the reference for the placement of the `geometry`.
	///
	// TODO: Maybe remove the last line
	/// In URDF this field is refered to as `<origin>`.
	pub transform: Transform,
	pub geometry: GeometryShapeContainer,
}

impl GeometryShapeData {
	/// X Y Z Bounding box sizes from center of the origin of the shape.
	pub fn bounding_box(&self) -> (f32, f32, f32) {
		match &self.geometry {
			GeometryShapeContainer::Box(g) => g.bounding_box(),
			GeometryShapeContainer::Cylinder(g) => g.bounding_box(),
			GeometryShapeContainer::Sphere(g) => g.bounding_box(),
			GeometryShapeContainer::Mesh(g) => g.bounding_box(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum GeometryShapeContainer {
	Box(BoxGeometry),
	Cylinder(CylinderGeometry),
	Sphere(SphereGeometry),
	// Capsule(String),
	Mesh(MeshGeometry),
}

#[cfg(feature = "urdf")]
impl ToURDF for GeometryShapeContainer {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		match self {
			GeometryShapeContainer::Box(box_geometry) => box_geometry.to_urdf(writer, urdf_config),
			GeometryShapeContainer::Cylinder(cylinder_geometry) => {
				cylinder_geometry.to_urdf(writer, urdf_config)
			}
			GeometryShapeContainer::Sphere(sphere_geometry) => {
				sphere_geometry.to_urdf(writer, urdf_config)
			}
			GeometryShapeContainer::Mesh(mesh_geometry) => {
				mesh_geometry.to_urdf(writer, urdf_config)
			}
		}
	}
}

impl From<BoxGeometry> for GeometryShapeContainer {
	fn from(value: BoxGeometry) -> Self {
		Self::Box(value)
	}
}

impl From<CylinderGeometry> for GeometryShapeContainer {
	fn from(value: CylinderGeometry) -> Self {
		Self::Cylinder(value)
	}
}

impl From<SphereGeometry> for GeometryShapeContainer {
	fn from(value: SphereGeometry) -> Self {
		Self::Sphere(value)
	}
}

impl From<MeshGeometry> for GeometryShapeContainer {
	fn from(value: MeshGeometry) -> Self {
		Self::Mesh(value)
	}
}
