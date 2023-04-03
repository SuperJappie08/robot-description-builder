use std::sync::Arc;

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode, URDFMaterialReferences};
use crate::{
	link::geometry::GeometryInterface, material::Material, transform_data::TransformData, ArcLock,
};
// use crate::material::MaterialDescriptor;
// use crate::linkbuilding::VisualBuilder;

#[derive(Debug)]
pub struct Visual {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	pub(crate) origin: Option<TransformData>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub material: Option<ArcLock<Material>>,
}

impl Visual {
	// #[deprecated]
	// pub fn builder<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
	// 	name: Option<String>,
	// 	origin: Option<TransformData>,
	// 	geometry: Geometry,
	// 	material_description: Option<MaterialDescriptor>,
	// ) -> VisualBuilder {
	// 	VisualBuilder::new(name, origin, geometry, material_description)
	// }

	/// Maybe temp
	pub fn new<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		name: Option<String>,
		origin: Option<TransformData>,
		geometry: Geometry,
		material: Option<ArcLock<Material>>,
	) -> Self {
		Self {
			name,
			origin,
			geometry: geometry.into(),
			material,
		}
	}

	pub fn get_name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// TODO: Maybe make optional reference?
	pub fn get_origin(&self) -> Option<&TransformData> {
		self.origin.as_ref()
	}

	pub fn get_geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}

	pub fn get_material(&self) -> Option<&ArcLock<Material>> {
		self.material.as_ref()
	}

	// #[deprecated]
	// pub fn rebuild(&self) -> VisualBuilder {
	// 	VisualBuilder {
	// 		name: self.name.clone(),
	// 		origin: self.origin,
	// 		geometry: self.geometry.boxed_clone(),
	// 		material_description: self
	// 			.material
	// 			.as_ref()
	// 			.map(|material| material.read().unwrap().describe()), // UNWRAP???
	// 	}
	// }
}

#[cfg(feature = "urdf")]
impl ToURDF for Visual {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("visual");
		if let Some(name) = self.get_name() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.as_bytes().into(),
			});
		}
		element.write_inner_content(|writer| {
			if let Some(origin) = self.get_origin() {
				origin.to_urdf(writer, urdf_config)?
			}

			self.get_geometry().to_urdf(writer, urdf_config)?;
			if let Some(material) = self.get_material() {
				let has_name =  material.read().unwrap().get_name().is_some(); // FIXME: Check if unwrap is ok here?
				let material_config = URDFConfig {
					direct_material_ref: match urdf_config.material_references {
						URDFMaterialReferences::AllNamedMaterialOnTop => {
							// TODO: Figure out if this check is useless (for name)
							if has_name
							{
								URDFMaterialMode::Referenced
							} else {
								URDFMaterialMode::FullMaterial
							}
						}
						URDFMaterialReferences::OnlyMultiUseMaterials => {
							#[cfg(any(feature = "logging", test))]
							log::info!(target: "ToURDF::Visual","The Material {} has a strong count of {}", material.read().unwrap().get_name().unwrap(), Arc::strong_count(material));
							// TODO: Figure out if this check is useless (for name)
							if has_name && Arc::strong_count(material) > 2 {
								URDFMaterialMode::Referenced
							} else {
								URDFMaterialMode::FullMaterial
							}
						}
					},
					..urdf_config.clone()
				};
				material
					.read()
					.unwrap() // FIXME: Don't know if unwrap is ok here?
					.to_urdf(writer, &material_config)?
			}
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Visual {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& (&self.geometry == &other.geometry)
			&& match (&self.material, &other.material) {
				(None, None) => true,
				(Some(own_material), Some(other_material)) => {
					// FIXME: The Or is for testing pursposes, It might need to be incorparted into the Lib, but then we need a differnt way
					// Needed for unnamed materials, which do not share a reference.
					// TODO: Redo materials
					Arc::ptr_eq(own_material, other_material)
						|| own_material
							.read()
							.unwrap()
							.eq(&other_material.read().unwrap())
				}
				_ => false,
			}
	}
}

impl Clone for Visual {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
			material: self.material.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;
	use test_log::test;

	use crate::{
		link::{
			geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
			visual::Visual,
		},
		material::Material,
		transform_data::TransformData,
	};

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, *};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialReferences};
		use std::io::Seek;

		#[test]
		fn no_name_no_origin_no_material() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(
				Visual::new(None, None, BoxGeometry::new(1.0, 2.0, 3.0), None)
					.to_urdf(&mut writer, &URDFConfig::default())
					.is_ok()
			);

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<visual><geometry><box size="1 2 3"/></geometry></visual>"#)
			)
		}

		#[test]
		fn name_no_origin_no_material() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Visual::new(
				Some("myLink_vis".to_owned()),
				None,
				CylinderGeometry::new(9., 6.258),
				None
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<visual name="myLink_vis"><geometry><cylinder radius="9" length="6.258"/></geometry></visual>"#
				)
			)
		}

		#[test]
		fn no_name_origin_no_material() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Visual::new(
				None,
				Some(TransformData {
					translation: Some((4., 6.78, 1.)),
					rotation: Some((PI, 2. * PI, 0.))
				}),
				SphereGeometry::new(3.),
				None
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<visual><origin xyz="4 6.78 1" rpy="3.1415927 6.2831855 0"/><geometry><sphere radius="3"/></geometry></visual>"#
				)
			)
		}

		#[test]
		fn no_name_no_origin_material() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Visual::new(
				None,
				None,
				CylinderGeometry::new(4.5, 75.35),
				Some(
					Material::new_color(Some("material_name".to_owned()), 0.5, 0.55, 0.6, 1.)
						.into()
				)
			)
			.to_urdf(
				&mut writer,
				&URDFConfig {
					material_references: URDFMaterialReferences::OnlyMultiUseMaterials,
					..Default::default()
				}
			)
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<visual><geometry><cylinder radius="4.5" length="75.35"/></geometry><material name="material_name"><color rgba="0.5 0.55 0.6 1"/></material></visual>"#
				)
			)
		}

		#[test]
		fn name_origin_material() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Visual::new(
				Some("some_col".into()),
				Some(TransformData {
					translation: Some((5.4, 9.1, 7.8)),
					..Default::default()
				}),
				CylinderGeometry::new(4.5, 75.35),
				Some(Material::new_color(None, 0.75, 0.5, 1., 1.).into())
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<visual name="some_col"><origin xyz="5.4 9.1 7.8"/><geometry><cylinder radius="4.5" length="75.35"/></geometry><material><color rgba="0.75 0.5 1 1"/></material></visual>"#
				)
			)
		}
	}
}
