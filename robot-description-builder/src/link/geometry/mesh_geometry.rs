use super::{GeometryInterface, GeometryShapeContainer};
use crate::{identifiers::GroupID, transform::Mirror};
use itertools::Itertools;
use nalgebra::{vector, Matrix3};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

// TODO: DOCS
//
// The fields are public for the Python wrapper. It doesn't change much for the Rust side, since most of the time these will be `Box<dyn GeometryInterface + Sync + Send>`.
// DOC COPY
// A trimesh element specified by a filename, and an optional scale that scales the mesh's axis-aligned-bounding-box. Any geometry format is acceptable but specific application compatibility is dependent on implementation. The recommended format for best texture and color support is Collada .dae files. The mesh file is not transferred between machines referencing the same model. It must be a local file. Prefix the filename with package://\<packagename\>/\<path\> to make the path to the mesh file relative to the package \<packagename\>.
#[derive(Debug, PartialEq, Clone)]
pub struct MeshGeometry {
	/// This should be a valid package path to a mesh file (e.g. `"package://robot_description/mesh/{mesh}"`).
	/// This is unchecked.
	/// You are on your own here.
	pub path: String,
	/// This is the size of the bounding box of the mesh at the current [`scale`](MeshGeometry::scale).
	///
	/// The bounding box is expected to be measured such that the center of the bounding box is at the origin.
	pub bounding_box: (f32, f32, f32),
	/// The desired scale off the mesh.
	///
	/// # Important
	/// If this is non-zero you need to pre-calculate the scaled [`bounding_box`](MeshGeometry::bounding_box).
	pub scale: (f32, f32, f32),
}

impl MeshGeometry {
	/// Creates a new `MeshGeometry`.
	///
	/// # Important
	/// - [`path`](MeshGeometry::path) should be a valid package path (e.g. `"package://robot_description/mesh/{mesh}"`). You are on your own here.
	/// - [`bounding_box`](MeshGeometry::bounding_box) should be the bounding box at the current `scale`.
	/// - [`scale`](MeshGeometry::scale) is either specified or defaults to `(1., 1., 1.)`.
	pub fn new(
		path: impl Into<String>,
		bounding_box: (f32, f32, f32),
		scale: Option<(f32, f32, f32)>,
	) -> Self {
		Self {
			path: path.into(),
			bounding_box,
			scale: scale.unwrap_or((1., 1., 1.)),
		}
	}
}

impl GeometryInterface for MeshGeometry {
	/// The volume of a mesh is approximated by its boundingbox
	fn volume(&self) -> f32 {
		self.bounding_box.0 * self.bounding_box.1 * self.bounding_box.2
	}

	/// The surface area of a mesh is approximated by its boundingbox
	fn surface_area(&self) -> f32 {
		2. * (self.bounding_box.0 * self.bounding_box.1
			+ self.bounding_box.1 * self.bounding_box.2
			+ self.bounding_box.0 * self.bounding_box.2)
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}

	fn bounding_box(&self) -> (f32, f32, f32) {
		self.bounding_box
	}

	fn shape_container(&self) -> GeometryShapeContainer {
		self.clone().into()
	}
}

// TODO: ADD MIRROR TEST
impl Mirror for MeshGeometry {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		// TODO: Add Mirrorable Specifier
		// if let Some(group_id @ ("L" | "R" | "N"))  = self.path.get_group_id() {

		// }
		Self {
			path: self.path.clone(), // TODO: MIRRORable PATH ID
			bounding_box: self.bounding_box,
			scale: (mirror_matrix * vector![self.scale.0, self.scale.1, self.scale.2])
				.iter()
				.copied()
				.collect_tuple()
				.unwrap(),
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for MeshGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| -> quick_xml::Result<()> {
			writer
				.create_element("mesh")
				.with_attribute(Attribute {
					key: QName(b"filename"),
					// Apply GroupID escaping to allow for mirror from `mesh_L` -> `mesh_R`
					value: self.path.display().as_bytes().into(),
				})
				.with_attribute(Attribute {
					key: QName(b"scale"),
					value: format!("{} {} {}", self.scale.0, self.scale.1, self.scale.2)
						.as_bytes()
						.into(),
				})
				.write_empty()?;
			Ok(())
		})?;
		Ok(())
	}
}

impl From<MeshGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: MeshGeometry) -> Self {
		Box::new(value)
	}
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "xml")]
	use std::io::Seek;
	use test_log::test;

	use super::{GeometryInterface, GeometryShapeContainer, MeshGeometry};
	#[cfg(feature = "urdf")]
	use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

	#[test]
	fn volume() {
		assert_eq!(
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.volume(),
			5.
		);
		assert_eq!(
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.volume(),
			27.
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.volume(),
			1.
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.volume(),
			14661.
		);
	}

	#[test]
	fn surface_area() {
		assert_eq!(
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.surface_area(),
			22.
		);
		assert_eq!(
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.surface_area(),
			54.
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.surface_area(),
			8.5
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.surface_area(),
			8378.5
		);
	}

	#[test]
	fn boxed_clone() {
		assert_eq!(
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.boxed_clone(),
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.into()
		);
		assert_eq!(
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.boxed_clone(),
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.into()
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.boxed_clone(),
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.into()
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.boxed_clone(),
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.into()
		);
	}

	#[test]
	fn bounding_box() {
		assert_eq!(
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.bounding_box(),
			(1., 5., 1.)
		);
		assert_eq!(
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.bounding_box(),
			(3., 3., 3.)
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.bounding_box(),
			(0.5, 0.5, 4.)
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.bounding_box(),
			(40.5, 90.5, 4.)
		);
	}

	#[test]
	fn get_shape() {
		assert_eq!(
			MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.shape_container(),
			GeometryShapeContainer::Mesh(MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			))
		);
		assert_eq!(
			MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.shape_container(),
			GeometryShapeContainer::Mesh(MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			))
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.shape_container(),
			GeometryShapeContainer::Mesh(MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			))
		);
		assert_eq!(
			MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.shape_container(),
			GeometryShapeContainer::Mesh(MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			))
		);
	}

	#[cfg(feature = "urdf")]
	#[test]
	fn to_urdf() {
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(MeshGeometry::new(
				"package://my-package/description/meshes/mesh_[[L]].dae",
				(1., 5., 1.),
				None
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(
					r#"<geometry><mesh filename="package://my-package/description/meshes/mesh_L.dae" scale="1 1 1"/></geometry>"#
				)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(MeshGeometry::new(
				"package://my-other-package/description/meshes/symmetrical-mesh.dae",
				(3., 3., 3.),
				None
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(
					r#"<geometry><mesh filename="package://my-other-package/description/meshes/symmetrical-mesh.dae" scale="1 1 1"/></geometry>"#
				)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(MeshGeometry::new(
				"package://a-package/description/meshes_[[L]]/arm.dae",
				(0.5, 0.5, 4.),
				Some((0.9, 0.9, 0.9))
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(
					r#"<geometry><mesh filename="package://a-package/description/meshes_L/arm.dae" scale="0.9 0.9 0.9"/></geometry>"#
				)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(MeshGeometry::new(
				"package://a-package/description/meshes/somethingweird.dae",
				(40.5, 90.5, 4.),
				Some((1., -1., 1.))
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(
					r#"<geometry><mesh filename="package://a-package/description/meshes/somethingweird.dae" scale="1 -1 1"/></geometry>"#
				)
			);
		}
	}
}
