use nalgebra::Matrix3;

use crate::{
	identifiers::GroupIDChanger,
	link::{
		builder::CollisionBuilder,
		geometry::{GeometryInterface, GeometryShapeData},
		visual::Visual,
	},
	material::MaterialDescriptor,
	transform::{Mirror, Transform},
};

/// The builder for `Visual` components.
///
/// The `VisualBuilder` is used to construct [`Visual`] elements of [`Links`](crate::link::Link).
///
/// This will configure the visual data:
/// - **[`geometry`](crate::link_data::geometry)**: The geometry used for visualization.
/// - **[`material`](crate::material)** (Optional): The material is used to control the appearance of the `geometry`.
/// - **[`transform`](crate::Transform)** (Optional): The transform from the [`Link`] frame to the `geometry`.
/// - **`name`** (Optional): The [_string identifier_](crate::identifiers) (or name) of this visual element. For practical purposes, it is recommended to use unique identifiers/names.
///
/// They can be added to a [`LinkBuilder`](super::LinkBuilder) while constructing a [`Link`] by calling [`add_visual`](crate::link::builder::LinkBuilder::add_visual).
///
/// A `VisualBuilder` can be converted to a [`CollisionBuilder`] to make defining [`Collision`](crate::link::collision::Collision) easier. <br/>
/// **WARNING:** It is not recommended to use high-detail meshes for collision geometries, since this will slow down the collision checking process.
/// Also, keep in mind, that some simulators only support the use of convex meshes for collisions, if at all.
///
/// [`Link`]: crate::link::Link
#[derive(Debug)]
pub struct VisualBuilder {
	/// The [_string identifier_](crate::identifiers) or name of this visual element.
	///
	/// For practical purposes, it is recommended to use unique identifiers/names.
	pub(crate) name: Option<String>,
	/// The transform from the origin of the parent `Link` to the origin of this `Visual`.
	///
	/// This is the reference for the placement of the `geometry`.
	///
	/// In URDF this field is refered to as `<origin>`.
	pub(crate) transform: Option<Transform>,
	/// The geometry of this Visual element.
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// The material of this Visual element.
	pub(crate) material_description: Option<MaterialDescriptor>,
}

impl VisualBuilder {
	/// Create a new [`VisualBuilder`] with the specified [`Geometry`](crate::link_data::geometry).
	pub fn new(geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>) -> Self {
		Self {
			name: None,
			transform: None,
			geometry: geometry.into(),
			material_description: None,
		}
	}

	// TODO: Figure out if this will be kept [Added for easier transistion]
	/// Create a new [`VisualBuilder`] with all fields specified.
	pub fn new_full(
		name: Option<String>,
		transform: Option<Transform>,
		geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>,
		material_description: Option<MaterialDescriptor>,
	) -> Self {
		Self {
			name,
			transform,
			geometry: geometry.into(),
			material_description,
		}
	}

	/// Sets the `name` of this `VisualBuilder`.
	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Specify a `transform` for this `VisualBuilder`.
	///
	/// The default is a no transformation (The frame of the `Visual` will be the same as the frame of the parent `Link`).
	pub fn transformed(mut self, transform: Transform) -> Self {
		self.transform = Some(transform);
		self
	}

	/// Specify a `material` for this `VisualBuilder`.
	///
	/// The default is no material.
	pub fn materialized(mut self, material_description: MaterialDescriptor) -> Self {
		self.material_description = Some(material_description);
		self
	}

	// TODO: IMPROVE DOCS
	/// Creates a `CollisionBuilder` from this `VisualBuilder` reference by lossy conversion.
	///
	/// Creates a [`CollisionBuilder`] from the `VisualBuilder` by cloning the following fields:
	///  - `name`
	///  - `transform`
	///  - `geometry`
	pub fn to_collision(&self) -> CollisionBuilder {
		CollisionBuilder {
			name: self.name.clone(),
			transform: self.transform,
			geometry: self.geometry.boxed_clone(),
		}
	}

	pub(crate) fn build(self) -> Visual {
		let material = self
			.material_description
			.map(|description| description.build());

		Visual {
			name: self.name,
			transform: self.transform,
			geometry: self.geometry,
			material,
		}
	}

	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			transform: self.transform.unwrap_or_default(),
			geometry: self.geometry.shape_container(),
		}
	}
}

impl Mirror for VisualBuilder {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		Self {
			name: self.name.as_ref().cloned(), // TODO: Rename?
			transform: self
				.transform
				.as_ref()
				.map(|transform| transform.mirrored(mirror_matrix)),
			geometry: self.geometry.boxed_mirrored(mirror_matrix),
			material_description: self.material_description.clone(),
		}
	}
}

/// Non-builder methods
impl VisualBuilder {
	/// Gets an optional reference to the `name` of this `VisualBuilder`.
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// Gets an optional reference to the `transform` of this `VisualBuilder`.
	pub fn transform(&self) -> Option<&Transform> {
		self.transform.as_ref()
	}

	/// Gets a reference to the [`geometry`](crate::link_data::geometry) of this `VisualBuilder`.
	pub fn geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}

	/// Gets an optional reference to the [`MaterialDescriptor`](crate::material::MaterialDescriptor) of this `VisualBuilder`.
	pub fn material(&self) -> Option<&MaterialDescriptor> {
		self.material_description.as_ref()
	}
}

impl GroupIDChanger for VisualBuilder {
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
		if let Some(name) = self.name.as_mut() {
			name.change_group_id_unchecked(new_group_id);
		}

		if let Some(material_builder) = self.material_description.as_mut() {
			material_builder.change_group_id_unchecked(new_group_id);
		}
	}

	fn apply_group_id(&mut self) {
		if let Some(name) = self.name.as_mut() {
			name.apply_group_id();
		}

		if let Some(material_builder) = self.material_description.as_mut() {
			material_builder.apply_group_id();
		}
	}
}

impl PartialEq for VisualBuilder {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.transform == other.transform
			&& *self.geometry == *other.geometry
			&& self.material_description == other.material_description
	}
}

impl Clone for VisualBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			transform: self.transform,
			geometry: self.geometry.boxed_clone(),
			material_description: self.material_description.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::VisualBuilder;
	use crate::link::link_data::geometry::{BoxGeometry, CylinderGeometry, SphereGeometry};
	use test_log::test;
	// TODO: Write tests
	mod group_id_changer {
		use super::{test, BoxGeometry, CylinderGeometry, SphereGeometry, VisualBuilder};
		use crate::identifiers::{GroupIDChanger, GroupIDError};

		#[test]
		fn change_group_id_unchecked_no_material() {
			#[inline]
			fn test(collision_builder: VisualBuilder, new_group_id: &str, name: Option<&str>) {
				let mut visual_builder = collision_builder;
				unsafe {
					visual_builder.change_group_id_unchecked(new_group_id);
				}
				assert_eq!(
					visual_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name
			test(VisualBuilder::new(BoxGeometry::new(1., 2., 3.)), "7", None);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)),
				"[[invalid]]",
				None,
			);
			test(VisualBuilder::new(SphereGeometry::new(3.3e9)), "", None);

			// Named, but no GroupID
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7",
				Some("ThisCoolName"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"valid4",
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"bol",
				Some("Bal"),
			);

			// Named with GroupID and valid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"7",
				Some("Leg_[[7]]_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("Arm_[[B01d]]_link_0313c"),
				"valid4",
				Some("Arm_[[valid4]]_link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"G0-02",
				Some("Bal_[[G0-02]]_this_doesn't_matter"),
			);

			// Named with GroupID and invalid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"[[7",
				Some("Leg_[[[[7]]_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("Arm_[[B01d]]_link_0313c"),
				"[[invalid]]",
				Some("Arm_[[[[invalid]]]]_link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"",
				Some("Bal_[[]]_this_doesn't_matter"),
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn change_group_id_unchecked_with_material() {
			todo!()
		}

		#[test]
		fn change_group_id_no_material() {
			#[inline]
			fn test(
				visual_builder: VisualBuilder,
				new_group_id: &str,
				result_change: Result<(), GroupIDError>,
				name: Option<&str>,
			) {
				let mut visual_builder = visual_builder;
				assert_eq!(visual_builder.change_group_id(new_group_id), result_change);
				assert_eq!(
					visual_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name, valid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)),
				"7",
				Ok(()),
				None,
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)),
				"valid5",
				Ok(()),
				None,
			);
			test(
				VisualBuilder::new(SphereGeometry::new(7.)),
				"R04",
				Ok(()),
				None,
			);

			// No Name, invalid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)),
				"7]]",
				Err(GroupIDError::new_close("7]]")),
				None,
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				None,
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)),
				"",
				Err(GroupIDError::new_empty()),
				None,
			);

			// Named, but no GroupID
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7",
				Ok(()),
				Some("ThisCoolName"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"valid4",
				Ok(()),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"bol",
				Ok(()),
				Some("Bal"),
			);

			// Named, but no GroupID and invalid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7]]",
				Err(GroupIDError::new_close("7]]")),
				Some("ThisCoolName"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"",
				Err(GroupIDError::new_empty()),
				Some("Bal"),
			);

			// Named with GroupID and valid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"7",
				Ok(()),
				Some("Leg_[[7]]_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("Arm_[[B01d]]_link_0313c"),
				"valid4",
				Ok(()),
				Some("Arm_[[valid4]]_link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"G0-02",
				Ok(()),
				Some("Bal_[[G0-02]]_this_doesn't_matter"),
			);

			// Named with GroupID and invalid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"[[7",
				Err(GroupIDError::new_open("[[7")),
				Some("Leg_[[L01]]_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("Arm_[[B01d]]_link_0313c"),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				Some("Arm_[[B01d]]_link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"",
				Err(GroupIDError::new_empty()),
				Some("Bal_[[F900]]_this_doesn't_matter"),
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn change_group_id_with_material() {
			todo!()
		}

		#[test]
		fn apply_group_id_no_material() {
			#[inline]
			fn test(visual_builder: VisualBuilder, name: Option<&str>) {
				let mut collision_builder = visual_builder;
				collision_builder.apply_group_id();
				assert_eq!(
					collision_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name
			test(VisualBuilder::new(BoxGeometry::new(1., 2., 3.)), None);
			test(VisualBuilder::new(CylinderGeometry::new(32., 5.)), None);
			test(VisualBuilder::new(SphereGeometry::new(7.)), None);

			// Named, but no GroupID
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				Some("ThisCoolName"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				Some("Bal"),
			);

			// Named, but escaped
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("This[\\[Cool]\\]Name"),
				Some("This[[Cool]]Name"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[\\[]"),
				Some("ADAdsadsdasdDS[[]"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9)).named("Bal]\\]"),
				Some("Bal]]"),
			);

			// Named with GroupID and valid
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				Some("Leg_L01_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.)).named("Arm_[[B01d]]_link_0313c"),
				Some("Arm_B01d_link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				Some("Bal_F900_this_doesn't_matter"),
			);

			// Named with mixed
			test(
				VisualBuilder::new(BoxGeometry::new(1., 2., 3.))
					.named("Leg_[\\[L01]\\]_[[l04]]_col"),
				Some("Leg_[[L01]]_l04_col"),
			);
			test(
				VisualBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]\\]_[\\[link_0313c]]"),
				Some("Arm_B01d]]_[[link_0313c"),
			);
			test(
				VisualBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_[\\[doesn't]\\]_matter"),
				Some("Bal_F900_this_[[doesn't]]_matter"),
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn apply_group_id_with_material() {
			todo!()
		}
	}
}
