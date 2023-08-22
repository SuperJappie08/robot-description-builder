#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::{
	identifiers::GroupID,
	link::{builder::VisualBuilder, geometry::GeometryInterface},
	link_data::geometry::GeometryShapeData,
	material::Material,
	transform::Transform,
};

#[derive(Debug)]
pub struct Visual {
	// TODO: Figure out if I want to keep the name optional?.
	pub(crate) name: Option<String>,
	/// The transform from the origin of the parent `Link` to the origin of this `Visual`.
	///
	/// This is the reference for the placement of the `geometry`.
	///
	/// In URDF this field is refered to as `<origin>`
	pub(crate) transform: Option<Transform>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub(crate) material: Option<Material>,
}

impl Visual {
	/// Create a new [`VisualBuilder`] with the specified [`geometry`](crate::link_data::geometry).
	pub fn builder(geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>) -> VisualBuilder {
		VisualBuilder::new(geometry)
	}

	// TODO: Is docthe test helpfull?
	/// Gets the reference to the name of the `Visual`
	///
	/// # Example
	/// Unwraps are hidden for brevity.
	/// ```rust
	/// # use robot_description_builder::{
	/// #     link_data::{geometry::SphereGeometry, Visual},
	/// #     linkbuilding::{LinkBuilder, VisualBuilder},
	/// #     KinematicInterface,
	/// # };
	/// let vis: VisualBuilder = Visual::builder(SphereGeometry::new(1.));
	/// let tree = LinkBuilder::new("example-1")
	///     .add_visual(vis.clone())
	///     .build_tree();
	///
	/// assert_eq!(
	///     tree.get_root_link()
	///         .read()
	/// #       .unwrap()
	///         .visuals()
	///         .first()
	/// #       .unwrap()
	///         .name(),
	///     None
	/// );
	///
	/// let tree = LinkBuilder::new("example-2")
	///     .add_visual(vis.named("Some Name"))
	///     .build_tree();
	///
	/// assert_eq!(
	///     tree.get_root_link()
	///         .read()
	/// #       .unwrap()
	///         .visuals()
	///         .first()
	/// #       .unwrap()
	///         .name(),
	///     Some(&"Some Name".to_owned())
	/// )
	/// ```
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	// TODO: Maybe make optional reference?
	pub fn transform(&self) -> Option<&Transform> {
		self.transform.as_ref()
	}

	pub fn geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}

	pub fn material(&self) -> Option<&Material> {
		self.material.as_ref()
	}

	pub(crate) fn material_mut(&mut self) -> Option<&mut Material> {
		self.material.as_mut()
	}

	pub fn rebuild(&self) -> VisualBuilder {
		VisualBuilder {
			name: self.name.clone(),
			transform: self.transform,
			geometry: self.geometry.boxed_clone(),
			material_description: self.material.as_ref().map(|material| material.describe()),
		}
	}

	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			transform: self.transform.unwrap_or_default(),
			geometry: self.geometry.shape_container(),
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Visual {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("visual");
		if let Some(name) = self.name() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.display().as_bytes().into(),
			});
		}
		element.write_inner_content(|writer| {
			// Could make this with `get_geometry_data``
			if let Some(transform) = self.transform() {
				transform.to_urdf(writer, urdf_config)?
			}

			self.geometry()
				.shape_container()
				.to_urdf(writer, urdf_config)?;
			if let Some(material) = self.material() {
				material.to_urdf(writer, urdf_config)?;
			}
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Visual {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.transform == other.transform
			&& *self.geometry == *other.geometry
			&& match (&self.material, &other.material) {
				(None, None) => true,
				(Some(own_material), Some(other_material)) => {
					// FIXME: The Or is for testing pursposes, It might need to be incorparted into the Lib, but then we need a differnt way
					// Needed for unnamed materials, which do not share a reference.
					own_material == other_material
				}
				_ => false,
			}
	}
}

impl Clone for Visual {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			transform: self.transform,
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
			builder::VisualBuilder,
			geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
			visual::Visual,
		},
		transform::Transform,
	};

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, *};
		use crate::{
			material::MaterialDescriptor,
			to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialReferences},
		};
		use std::io::Seek;

		fn test_to_urdf_visual(visual: VisualBuilder, result: String, urdf_config: &URDFConfig) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(visual.build().to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		fn no_name_no_origin_no_material() {
			test_to_urdf_visual(
				Visual::builder(BoxGeometry::new(1.0, 2.0, 3.0)),
				String::from(r#"<visual><geometry><box size="1 2 3"/></geometry></visual>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn name_no_origin_no_material() {
			test_to_urdf_visual(
				Visual::builder(CylinderGeometry::new(9., 6.258)).named("myLink_vis"),
				String::from(
					r#"<visual name="myLink_vis"><geometry><cylinder radius="9" length="6.258"/></geometry></visual>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn no_name_origin_no_material() {
			test_to_urdf_visual(
				Visual::builder(SphereGeometry::new(3.))
					.tranformed(Transform::new((4., 6.78, 1.), (PI, 2. * PI, 0.))),
				String::from(
					r#"<visual><origin xyz="4 6.78 1" rpy="3.1415927 6.2831855 0"/><geometry><sphere radius="3"/></geometry></visual>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn no_name_no_origin_material() {
			test_to_urdf_visual(
				Visual::builder(CylinderGeometry::new(4.5, 75.35)).materialized(
					MaterialDescriptor::new_color(0.5, 0.55, 0.6, 1.).named("material_name"),
				),
				String::from(
					r#"<visual><geometry><cylinder radius="4.5" length="75.35"/></geometry><material name="material_name"><color rgba="0.5 0.55 0.6 1"/></material></visual>"#,
				),
				&URDFConfig {
					material_references: URDFMaterialReferences::OnlyMultiUseMaterials,
					..Default::default()
				},
			);
		}

		#[test]
		fn name_origin_material() {
			test_to_urdf_visual(
				Visual::builder(CylinderGeometry::new(4.5, 75.35))
					.named("some_col")
					.tranformed(Transform::new_translation(5.4, 9.1, 7.8))
					.materialized(MaterialDescriptor::new_color(0.75, 0.5, 1., 1.)),
				String::from(
					r#"<visual name="some_col"><origin xyz="5.4 9.1 7.8"/><geometry><cylinder radius="4.5" length="75.35"/></geometry><material><color rgba="0.75 0.5 1 1"/></material></visual>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
