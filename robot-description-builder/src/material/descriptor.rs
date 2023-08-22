/* DOC TODO:
 o Internal Module Doc
 - MaterialDescriptor
*/
use crate::identifiers::GroupIDChanger;

use super::{data::MaterialData, Material};

/// A descriptor for a future `Material`.
///
/// A [`MaterialDescriptor`] is used to construct a [`Material`].
///
/// A `Descriptor` is a smiliar idea as a `Builder`,
///  but there is an important difference between which has to be adressed with a differen name.
///
/// A `Builder` would always construct a new instance of a struct, in this project meaning it cannot be used twice in the same [`KinematicTree`](crate::KinematicTree).
/// Since using a `Builder` twice would result in two exactly the same objects.
///
/// A `Descriptor` on the other hand, first checks if there already exists an instance which matches it description, in this case a [`Material`].
/// If not the case a new instance ([`Material`]) is constructed and added to the index.
/// If an instance already exists, which exactly matches the description, then the 'new' [`Material`] will refer to the pre-existing data.
///
/// This is desirable in the case of [Materials](Material), since they are often reused.
/// This could also allow for changing a [`Material`] and the other used of it changing withit.
///
/// # OLD STUFF TODO: UPDATE
/// When a `MaterialDescriptor` is constructed for a specific `KinematicDataTee`, the following steps happen:
///  1. Check if the description of the `MaterialDescriptor` matches a pre-existing `Material` already in the tree.
///     - If the a `Material` matches the description, the reference to that material is returned.
///     - If no `Material` matches the desctiption, a new `Material` is constructed and inserted to the `material_index` of the `KinematicDataTree` and the reference is returned.
///     - If only the `name` of the `Material` matches, an error is raised.
#[derive(Debug, PartialEq, Clone)]
pub struct MaterialDescriptor {
	name: Option<String>,
	data: MaterialData,
}

impl MaterialDescriptor {
	/// Creates a new [`MaterialDescriptor`] with a solid color (rgba)
	///
	/// The `red`, `green`, `blue` and `alpha` fields expect a value between 0 and 1.
	///
	/// # Example
	///
	/// ```
	/// # use robot_description_builder::material::MaterialDescriptor;
	/// MaterialDescriptor::new_color(1., 0.4, 0.6, 0.5)
	/// # ;
	/// ```
	pub fn new_color(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		MaterialDescriptor {
			name: None,
			data: MaterialData::Color(red, green, blue, alpha),
		}
	}

	/// Creates a new [`MaterialDescriptor`] with a solid color (rgb).
	///
	/// The `red`, `green`, `blue` fields expect a value between 0 and 1.
	///
	/// # Example
	///
	/// ```
	/// # use robot_description_builder::material::MaterialDescriptor;
	/// MaterialDescriptor::new_rgb(1., 0.4, 0.6)
	/// # ;
	/// ```
	pub fn new_rgb(red: f32, green: f32, blue: f32) -> Self {
		MaterialDescriptor {
			name: None,
			data: MaterialData::Color(red, green, blue, 1.),
		}
	}

	/// Creates a new [`MaterialDescriptor`] with a texture.
	///
	/// `texture_path` should be a valid package path (e.g. `"package://NAME_OF_PACKAGE/path/{texture}"`). You are on your own here.
	///
	/// # Example
	///
	/// ```
	/// # use robot_description_builder::material::MaterialDescriptor;
	/// MaterialDescriptor::new_texture("package://robot_description/textures/example_texture.png")
	/// # ;
	/// ```
	pub fn new_texture(texture_path: impl Into<String>) -> Self {
		MaterialDescriptor {
			name: None,
			data: MaterialData::Texture(texture_path.into()),
		}
	}

	/// Creates a new [`MaterialDescriptor`] from a pre-existing [`MaterialData`].
	pub(crate) fn new_data(data: MaterialData) -> Self {
		MaterialDescriptor { name: None, data }
	}

	/// Adds a `name` to the [`MaterialDescriptor`], so it can later be used as a referenced [`Material`].
	///
	/// # Important
	/// When a named [`Material`] is used, it needs to be the same as all materials with the same name.
	/// Otherwise, problems will arise later down the line.
	///
	/// # Example
	///
	/// ```
	/// # use robot_description_builder::material::MaterialDescriptor;
	/// MaterialDescriptor::new_rgb(0.5, 1., 0.5).named("soft-green")
	/// # ;
	/// ```
	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Builds a [`Material`] from the [`MaterialDescriptor`].
	pub(crate) fn build(self) -> Material {
		match self.name {
			Some(name) => Material::new_named_uninited(name, self.data),
			None => Material::new_unnamed(self.data),
		}
	}
}

/// Non-builder methods
impl MaterialDescriptor {
	/// Gets the optional of the [`MaterialDescriptor`] as a optional reference.
	pub fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// Gets a reference to the [`MaterialData`] of the [`MaterialDescriptor`].
	pub fn data(&self) -> &MaterialData {
		&self.data
	}
}

impl GroupIDChanger for MaterialDescriptor {
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

#[cfg(test)]
mod tests {
	use super::MaterialDescriptor;
	use test_log::test;

	mod group_id_changer {
		use super::{test, MaterialDescriptor};
		use crate::identifiers::{GroupIDChanger, GroupIDError};

		#[inline]
		fn test_change_group_id_unchecked(
			material_builder: MaterialDescriptor,
			new_group_id: &str,
			final_name: Option<&str>,
		) {
			let mut material_builder = material_builder;
			unsafe {
				material_builder.change_group_id_unchecked(new_group_id);
			}
			assert_eq!(
				material_builder.name,
				final_name.and_then(|final_name| Some(final_name.to_owned()))
			)
		}

		#[test]
		fn change_group_id_unchecked_no_name() {
			// Valid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.),
				"R04",
				None,
			);
			test_change_group_id_unchecked(MaterialDescriptor::new_rgb(1., 1., 0.), "C064w", None);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture"),
				"Yellow",
				None,
			);

			// Invalid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.),
				"[[R04",
				None,
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_rgb(1., 1., 0.),
				"C064w]]",
				None,
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture"),
				"",
				None,
			);
		}

		#[test]
		fn change_group_id_unchecked_with_name() {
			// name with field and Valid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[[L01]]_mat"),
				"R04",
				Some("Leg_[[R04]]_mat"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[[dsd]]_dsdadavj,hnmn b v"),
				"C064w",
				Some("rgb_[[C064w]]_dsdadavj,hnmn b v"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[GroupID]]"),
				"Yellow",
				Some("SomeCoolTexture[[Yellow]]"),
			);

			// Named with field and Invalid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[[L01]]_mat"),
				"[[R04",
				Some("Leg_[[[[R04]]_mat"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("[[CADcs]]SomeColor"),
				"C064w]]",
				Some("[[C064w]]]]SomeColor"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[GroupID]]"),
				"",
				Some("SomeCoolTexture[[]]"),
			);
			// name without field and Valid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_L01_mat"),
				"R04",
				Some("Leg_L01_mat"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[\\[dsd]\\]_dsdadavj,hnmn b v"),
				"C064w",
				Some("rgb_[\\[dsd]\\]_dsdadavj,hnmn b v"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[\\[GroupID]]"),
				"Yellow",
				Some("SomeCoolTexture[\\[GroupID]]"),
			);

			// Named without field and Invalid
			test_change_group_id_unchecked(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_L01_mat"),
				"[[R04",
				Some("Leg_L01_mat"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("[[CADcs]\\]SomeColor"),
				"C064w]]",
				Some("[[CADcs]\\]SomeColor"),
			);
			test_change_group_id_unchecked(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture_GroupID_"),
				"",
				Some("SomeCoolTexture_GroupID_"),
			);
		}

		#[inline]
		fn test_change_group_id(
			material_builder: MaterialDescriptor,
			new_group_id: &str,
			change_result: Result<(), GroupIDError>,
			final_name: Option<&str>,
		) {
			let mut material_builder = material_builder;
			assert_eq!(
				material_builder.change_group_id(new_group_id),
				change_result
			);
			assert_eq!(
				material_builder.name,
				final_name.and_then(|final_name| Some(final_name.to_owned()))
			)
		}

		#[test]
		fn change_group_id_no_name() {
			// Valid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.),
				"R04",
				Ok(()),
				None,
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.),
				"C064w",
				Ok(()),
				None,
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture"),
				"Yellow",
				Ok(()),
				None,
			);

			// Invalid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.),
				"[[R04",
				Err(GroupIDError::new_open("[[R04")),
				None,
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.),
				"C064w]]",
				Err(GroupIDError::new_close("C064w]]")),
				None,
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture"),
				"",
				Err(GroupIDError::new_empty()),
				None,
			);
		}

		#[test]
		fn change_group_id_with_name() {
			// name with field and Valid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[[L01]]_mat"),
				"R04",
				Ok(()),
				Some("Leg_[[R04]]_mat"),
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[[dsd]]_dsdadavj,hnmn b v"),
				"C064w",
				Ok(()),
				Some("rgb_[[C064w]]_dsdadavj,hnmn b v"),
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[GroupID]]"),
				"Yellow",
				Ok(()),
				Some("SomeCoolTexture[[Yellow]]"),
			);

			// Named with field and Invalid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[[L01]]_mat"),
				"[[R04",
				Err(GroupIDError::new_open("[[R04")),
				Some("Leg_[[L01]]_mat"),
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("[[CADcs]]SomeColor"),
				"C064w]]",
				Err(GroupIDError::new_close("C064w]]")),
				Some("[[CADcs]]SomeColor"),
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[GroupID]]"),
				"",
				Err(GroupIDError::new_empty()),
				Some("SomeCoolTexture[[GroupID]]"),
			);
			// name without field and Valid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_L01_mat"),
				"R04",
				Ok(()),
				Some("Leg_L01_mat"),
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[\\[dsd]\\]_dsdadavj,hnmn b v"),
				"C064w",
				Ok(()),
				Some("rgb_[\\[dsd]\\]_dsdadavj,hnmn b v"),
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[\\[GroupID]]"),
				"Yellow",
				Ok(()),
				Some("SomeCoolTexture[\\[GroupID]]"),
			);

			// Named without field and Invalid
			test_change_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_L01_mat"),
				"[[R04",
				Err(GroupIDError::new_open("[[R04")),
				Some("Leg_L01_mat"),
			);
			test_change_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("[[CADcs]\\]SomeColor"),
				"C064w]]",
				Err(GroupIDError::new_close("C064w]]")),
				Some("[[CADcs]\\]SomeColor"),
			);
			test_change_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture_GroupID_"),
				"",
				Err(GroupIDError::new_empty()),
				Some("SomeCoolTexture_GroupID_"),
			);
		}

		#[inline]
		fn test_apply_group_id(material_builder: MaterialDescriptor, final_name: Option<&str>) {
			let mut material_builder = material_builder;
			material_builder.apply_group_id();
			assert_eq!(
				material_builder.name,
				final_name.and_then(|final_name| Some(final_name.to_owned()))
			)
		}

		#[test]
		fn apply_group_id_no_name() {
			test_apply_group_id(MaterialDescriptor::new_color(1., 0.5, 0.25, 0.), None);
			test_apply_group_id(MaterialDescriptor::new_rgb(1., 1., 0.), None);
			test_apply_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture"),
				None,
			);
		}

		#[test]
		fn apply_group_id_with_name() {
			// name with field and Valid
			test_apply_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[[L01]]_mat"),
				Some("Leg_L01_mat"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[[dsd]]_dsdadavj,hnmn b v"),
				Some("rgb_dsd_dsdadavj,hnmn b v"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[GroupID]]"),
				Some("SomeCoolTextureGroupID"),
			);

			// name with field and Valid and escpaed
			test_apply_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_[\\[[[L01]]_mat]\\]"),
				Some("Leg_[[L01_mat]]"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.)
					.named("rgb_[[dsd]]_d[\\[sdadavj]\\],hnmn b v"),
				Some("rgb_dsd_d[[sdadavj]],hnmn b v"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[[Gro[\\[upID]]"),
				Some("SomeCoolTextureGro[[upID"),
			);

			// This one is has too many opening and closign brackets
			test_apply_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.)
					.named("rgb_[[dsd]]_d[[sdadavj]\\],hnmn b v"),
				Some("rgb_[[dsd]]_d[[sdadavj]\\],hnmn b v"),
			);

			// name without field and Valid
			test_apply_group_id(
				MaterialDescriptor::new_color(1., 0.5, 0.25, 0.).named("Leg_L01_mat"),
				Some("Leg_L01_mat"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_rgb(1., 1., 0.).named("rgb_[\\[dsd]\\]_dsdadavj,hnmn b v"),
				Some("rgb_[[dsd]]_dsdadavj,hnmn b v"),
			);
			test_apply_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[\\[GroupID"),
				Some("SomeCoolTexture[[GroupID"),
			);

			// This one is has not one of the required amounts of correct brackets
			test_apply_group_id(
				MaterialDescriptor::new_texture("package://some/texture/path/text.texture")
					.named("SomeCoolTexture[\\[GroupID]]"),
				Some("SomeCoolTexture[\\[GroupID]]"),
			);
		}
	}
}
