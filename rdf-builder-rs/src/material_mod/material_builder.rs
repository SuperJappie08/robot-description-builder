use super::{material::Material, MaterialData};

/// FIXME: Name not final, maybe change to `MaterialDescriptor`
#[derive(Debug, PartialEq, Clone)]
pub struct MaterialBuilder {
	name: Option<String>,
	data: MaterialData,
}

impl MaterialBuilder {
	/// Creates a new [`MaterialBuilder`] with a solid color (rgba)
	///
	/// The `red`, `green`, `blue` and `alpha` fields expect a value between 0 and 1.
	///
	/// # Example
	///
	/// ```rust,text
	/// # use rdf_builder_rs::MaterialBuilder;
	/// MaterialBuilder::new_color(1., 0.4, 0.6, 0.5)
	/// # ;
	/// ```
	pub fn new_color(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		MaterialBuilder {
			name: None,
			data: MaterialData::Color(red, green, blue, alpha),
		}
	}

	/// Creates a new [`MaterialBuilder`] with a solid color (rgb)
	///
	/// The `red`, `green`, `blue` fields expect a value between 0 and 1.
	///
	/// # Example
	///
	/// ```rust,text
	/// # use rdf_builder_rs::MaterialBuilder;
	/// MaterialBuilder::new_rgb(1., 0.4, 0.6)
	/// # ;
	/// ```
	pub fn new_rgb(red: f32, green: f32, blue: f32) -> Self {
		MaterialBuilder {
			name: None,
			data: MaterialData::Color(red, green, blue, 1.),
		}
	}

	/// Creates a new [`MaterialBuilder`] with a texture.
	///
	/// `texture_path` should be a valid package path (e.g. `"package://robot_description/textures/{texture}"`). You are on your own here.
	///
	/// # Example
	///
	/// ```rust,text
	/// # use rdf_builder_rs::MaterialBuilder;
	/// MaterialBuilder::new_texture("package://robot_description/textures/example_texture.png")
	/// # ;
	/// ```
	pub fn new_texture(texture_path: impl Into<String>) -> Self {
		MaterialBuilder {
			name: None,
			data: MaterialData::Texture(texture_path.into()),
		}
	}

	/// Creates a new [`MaterialBuilder`] from a pre-existing [`MaterialData`]
	pub(crate) fn new_data(data: MaterialData) -> Self {
		MaterialBuilder { name: None, data }
	}

	/// Adds a `name` to the [`MaterialBuilder`], so it can later be used as a refenced [`Material`]
	///
	/// # Important
	/// When a named [`Material`] is used, it needs to be the same as all materials with the same name.
	/// Otherwise, problems will arise later down the line.
	///
	/// # Example
	///
	/// ```rust,text
	/// # use rdf_builder_rs::MaterialBuilder;
	/// MaterialBuilder::new_rgb(0.5, 1., 0.5).named("soft-green")
	/// # ;
	/// ```
	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	/// Builds a [`Material`] from the [`MaterialBuilder`].
	pub(crate) fn build(self) -> Material {
		match self.name {
			Some(name) => Material::Named {
				name,
				data: self.data.into(),
			},
			None => Material::Unamed(self.data),
		}
	}

	// ===== Non-Builder Methods ======

	/// Gets the optional of the [`MaterialBuilder`] as a optional reference.
	pub fn get_name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	/// Gets a reference to the [`MaterialData`] of the [`MaterialBuilder`]
	pub fn get_data(&self) -> &MaterialData {
		&self.data
	}
}
