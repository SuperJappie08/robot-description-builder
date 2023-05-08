use nalgebra::Matrix3;

use crate::{
	identifiers::GroupIDChanger,
	link::{
		collision::Collision,
		geometry::{GeometryInterface, GeometryShapeData},
	},
	transform::{Mirror, Transform},
};

#[derive(Debug)]
pub struct CollisionBuilder {
	pub(crate) name: Option<String>,
	pub(crate) origin: Option<Transform>,
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl CollisionBuilder {
	pub fn new(geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>) -> Self {
		Self {
			name: None,
			origin: None,
			geometry: geometry.into(),
		}
	}

	pub fn new_full(
		name: Option<String>,
		origin: Option<Transform>,
		geometry: impl Into<Box<dyn GeometryInterface + Sync + Send>>,
	) -> Self {
		Self {
			name,
			origin,
			geometry: geometry.into(),
		}
	}

	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	pub fn tranformed(mut self, transform: Transform) -> Self {
		self.origin = Some(transform);
		self
	}

	pub(crate) fn build(self) -> Collision {
		Collision {
			name: self.name,
			origin: self.origin,
			geometry: self.geometry,
		}
	}

	/// TODO: BETTER NAME
	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			origin: self.origin.unwrap_or_default(),
			geometry: self.geometry.shape_container(),
		}
	}
}

impl Mirror for CollisionBuilder {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		Self {
			name: self.name.as_ref().map(Clone::clone), // FIXME: NAME
			origin: self
				.origin
				.as_ref()
				.map(|transform| transform.mirrored(mirror_matrix)),
			geometry: self.geometry.boxed_mirrored(mirror_matrix), // TODO: this only works on non-chiral geometries, non chiral meshes could maybe be scaled to neg
		}
	}
}

/// Non-builder methods
impl CollisionBuilder {
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	pub fn origin(&self) -> Option<&Transform> {
		self.origin.as_ref()
	}

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
		self.name == other.name && self.origin == other.origin && *self.geometry == *other.geometry
	}
}

impl Clone for CollisionBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
		}
	}
}

/// TODO: Decide if this is ok?
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
