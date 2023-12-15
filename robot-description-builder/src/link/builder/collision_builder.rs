use nalgebra::Matrix3;

use crate::{
	identifiers::GroupIDChanger,
	link::{
		builder::VisualBuilder,
		collision::Collision,
		geometry::{GeometryInterface, GeometryShapeData},
	},
	transform::{Mirror, Transform},
};

/// The builder for `Collision` components.
///
/// The `CollisionBuilder` is used to construct [`Collision`] elements of [`Links`](crate::link::Link).
///
/// This will configure the collision data:
/// - **[`geometry`](crate::link_data::geometry)**: The geometry used for collision checking[^mesh-warning].
/// - **[`transform`](crate::Transform)** (Optional): The transform from the [`Link`] frame to the `geometry`.
/// - **`name`** (Optional): The [_string identifier_](crate::identifiers) (or name) of this collision element. For practical purposes, it is recommended to use unique identifiers/names.
///
/// They can be added to a [`LinkBuilder`](super::LinkBuilder) while constructing a [`Link`] by calling [`add_collider`](crate::link::builder::LinkBuilder::add_collider).
///
/// A `CollisionBuilder` can be converted to a [`VisualBuilder`] to make defining [`Visual`](crate::link::visual::Visual) easier.
/// If this is used, it might be easier to first create the [`VisualBuilder`], and convert that back to a `CollisionBuilder`, since it contains more information.
///
/// [^mesh-warning]: **WARNING:** It is not recommended to use high-detail meshes for collision geometries, since this will slow down the collision checking process.
/// Also, keep in mind, that some simulators only support the use of convex meshes for collisions, if at all.
///
/// [`Link`]: crate::link::Link
// TODO: Consider making the structfields public
#[derive(Debug)]
pub struct CollisionBuilder {
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

impl CollisionBuilder {
	/// Create a new [`CollisionBuilder`] with the specified [`Geometry`](crate::link_data::geometry).
	pub fn new(geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>) -> Self {
		Self {
			name: None,
			transform: None,
			geometry: geometry.into(),
		}
	}
	/// Create a new [`CollisionBuilder`] with all fields specified.
	pub fn new_full(
		name: Option<String>,
		transform: Option<Transform>,
		geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>,
	) -> Self {
		Self {
			name,
			transform,
			geometry: geometry.into(),
		}
	}

	/// Sets the `name` of this `CollisionBuilder`.
	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Specify a `transform` for this `CollisionBuilder`.
	///
	/// The default is a no transformation (The frame of the `Collision` will be the same as the frame of the parent `Link`).
	pub fn transformed(mut self, transform: Transform) -> Self {
		self.transform = Some(transform);
		self
	}

	// TODO: IMPROVE DOCS
	/// Creates a `VisualBuilder` from this `CollisionBuilder` reference by upgrading.
	///
	/// Creates a [`VisualBuilder`] from the `CollisionBuilder` by cloning the following fields:
	///  - `name`
	///  - `transform`
	///  - `geometry`
	/// The other fields are left empty, since they are optional.
	pub fn to_visual(&self) -> VisualBuilder {
		VisualBuilder {
			name: self.name.clone(),
			transform: self.transform,
			geometry: self.geometry.boxed_clone(),
			material_description: None,
		}
	}

	pub(crate) fn build(self) -> Collision {
		Collision {
			name: self.name,
			transform: self.transform,
			geometry: self.geometry,
		}
	}

	// TODO: BETTER NAME
	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			transform: self.transform.unwrap_or_default(),
			geometry: self.geometry.shape_container(),
		}
	}
}

impl Mirror for CollisionBuilder {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		Self {
			name: self.name.as_ref().cloned(), // TODO: Rename?
			transform: self
				.transform
				.as_ref()
				.map(|transform| transform.mirrored(mirror_matrix)),
			geometry: self.geometry.boxed_mirrored(mirror_matrix),
		}
	}
}

/// Non-builder methods
impl CollisionBuilder {
	/// Gets an optional reference to the `name` of this `CollisionBuilder`.
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// Gets an optional reference to the `transform` of this `CollisionBuilder`.
	pub fn transform(&self) -> Option<&Transform> {
		self.transform.as_ref()
	}

	/// Gets a reference to the [`geometry`](crate::link_data::geometry) of this `CollisionBuilder`.
	pub fn geometry(&self) -> &Box<dyn GeometryInterface + Sync + Send> {
		&self.geometry
	}
}

impl GroupIDChanger for CollisionBuilder {
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
		if let Some(name) = self.name.as_mut() {
			name.change_group_id_unchecked(new_group_id);
		}
	}

	fn apply_group_id(&mut self) {
		if let Some(name) = self.name.as_mut() {
			name.apply_group_id();
		}
	}
}

impl PartialEq for CollisionBuilder {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.transform == other.transform
			&& *self.geometry == *other.geometry
	}
}

impl Clone for CollisionBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			transform: self.transform,
			geometry: self.geometry.boxed_clone(),
		}
	}
}

// TODO: Decide if this is ok?
impl From<CollisionBuilder> for Collision {
	fn from(value: CollisionBuilder) -> Self {
		value.build()
	}
}

#[cfg(test)]
mod tests {
	use super::CollisionBuilder;
	use crate::link::link_data::geometry::{BoxGeometry, CylinderGeometry, SphereGeometry};
	use test_log::test;
	// TODO: Write tests

	mod group_id_changer {
		use super::{test, BoxGeometry, CollisionBuilder, CylinderGeometry, SphereGeometry};
		use crate::identifiers::{GroupIDChanger, GroupIDError};

		#[test]
		fn change_group_id_unchecked() {
			#[inline]
			fn test(collision_builder: CollisionBuilder, new_group_id: &str, name: Option<&str>) {
				let mut collision_builder = collision_builder;
				unsafe {
					collision_builder.change_group_id_unchecked(new_group_id);
				}
				assert_eq!(
					collision_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)),
				"7",
				None,
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)),
				"[[invalid]]",
				None,
			);
			test(CollisionBuilder::new(SphereGeometry::new(3.3e9)), "", None);

			// Named, but no GroupID
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7",
				Some("ThisCoolName"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"valid4",
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"bol",
				Some("Bal"),
			);

			// Named with GroupID and valid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"7",
				Some("Leg_[[7]]_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]]_link_0313c"),
				"valid4",
				Some("Arm_[[valid4]]_link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"G0-02",
				Some("Bal_[[G0-02]]_this_doesn't_matter"),
			);

			// Named with GroupID and invalid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"[[7",
				Some("Leg_[[[[7]]_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]]_link_0313c"),
				"[[invalid]]",
				Some("Arm_[[[[invalid]]]]_link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"",
				Some("Bal_[[]]_this_doesn't_matter"),
			);
		}

		#[test]
		fn change_group_id() {
			#[inline]
			fn test(
				collision_builder: CollisionBuilder,
				new_group_id: &str,
				result_change: Result<(), GroupIDError>,
				name: Option<&str>,
			) {
				let mut collision_builder = collision_builder;
				assert_eq!(
					collision_builder.change_group_id(new_group_id),
					result_change
				);
				assert_eq!(
					collision_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name, valid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)),
				"7",
				Ok(()),
				None,
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)),
				"valid5",
				Ok(()),
				None,
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(7.)),
				"R04",
				Ok(()),
				None,
			);

			// No Name, invalid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)),
				"7]]",
				Err(GroupIDError::new_close("7]]")),
				None,
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				None,
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)),
				"",
				Err(GroupIDError::new_empty()),
				None,
			);

			// Named, but no GroupID
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7",
				Ok(()),
				Some("ThisCoolName"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"valid4",
				Ok(()),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"bol",
				Ok(()),
				Some("Bal"),
			);

			// Named, but no GroupID and invalid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				"7]]",
				Err(GroupIDError::new_close("7]]")),
				Some("ThisCoolName"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				"",
				Err(GroupIDError::new_empty()),
				Some("Bal"),
			);

			// Named with GroupID and valid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"7",
				Ok(()),
				Some("Leg_[[7]]_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]]_link_0313c"),
				"valid4",
				Ok(()),
				Some("Arm_[[valid4]]_link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"G0-02",
				Ok(()),
				Some("Bal_[[G0-02]]_this_doesn't_matter"),
			);

			// Named with GroupID and invalid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				"[[7",
				Err(GroupIDError::new_open("[[7")),
				Some("Leg_[[L01]]_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]]_link_0313c"),
				"[[invalid]]",
				Err(GroupIDError::new_open("[[invalid]]")),
				Some("Arm_[[B01d]]_link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				"",
				Err(GroupIDError::new_empty()),
				Some("Bal_[[F900]]_this_doesn't_matter"),
			);
		}

		#[test]
		fn apply_group_id() {
			#[inline]
			fn test(collision_builder: CollisionBuilder, name: Option<&str>) {
				let mut collision_builder = collision_builder;
				collision_builder.apply_group_id();
				assert_eq!(
					collision_builder.name,
					name.and_then(|name| Some(name.to_owned()))
				)
			}

			// No Name
			test(CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)), None);
			test(CollisionBuilder::new(CylinderGeometry::new(32., 5.)), None);
			test(CollisionBuilder::new(SphereGeometry::new(7.)), None);

			// Named, but no GroupID
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("ThisCoolName"),
				Some("ThisCoolName"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[]"),
				Some("ADAdsadsdasdDS[]"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)).named("Bal"),
				Some("Bal"),
			);

			// Named, but escaped
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("This[\\[Cool]\\]Name"),
				Some("This[[Cool]]Name"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.)).named("ADAdsadsdasdDS[\\[]"),
				Some("ADAdsadsdasdDS[[]"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9)).named("Bal]\\]"),
				Some("Bal]]"),
			);

			// Named with GroupID and valid
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.)).named("Leg_[[L01]]_l04_col"),
				Some("Leg_L01_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]]_link_0313c"),
				Some("Arm_B01d_link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_doesn't_matter"),
				Some("Bal_F900_this_doesn't_matter"),
			);

			// Named with mixed
			test(
				CollisionBuilder::new(BoxGeometry::new(1., 2., 3.))
					.named("Leg_[\\[L01]\\]_[[l04]]_col"),
				Some("Leg_[[L01]]_l04_col"),
			);
			test(
				CollisionBuilder::new(CylinderGeometry::new(32., 5.))
					.named("Arm_[[B01d]\\]_[\\[link_0313c]]"),
				Some("Arm_B01d]]_[[link_0313c"),
			);
			test(
				CollisionBuilder::new(SphereGeometry::new(3.3e9))
					.named("Bal_[[F900]]_this_[\\[doesn't]\\]_matter"),
				Some("Bal_F900_this_[[doesn't]]_matter"),
			);
		}
	}
}
