use crate::material::material::Material;
use crate::material::MaterialData;

/// FIXME: Name not final
#[derive(Debug, PartialEq, Clone)]
pub struct MaterialBuilder {
	name: Option<String>,
	data: MaterialData,
}

impl MaterialBuilder {
	pub fn new_color(red: f32, green: f32, blue: f32, alpha: f32) -> MaterialBuilder {
		MaterialBuilder {
			name: None,
			data: MaterialData::Color(red, green, blue, alpha),
		}
	}

	pub fn new_texture<TexturePath: Into<String>>(texture_path: TexturePath) -> MaterialBuilder {
		MaterialBuilder {
			name: None,
			data: MaterialData::Texture(texture_path.into()),
		}
	}

	pub(crate) fn new_data(data: MaterialData) -> MaterialBuilder {
		MaterialBuilder { name: None, data }
	}

	pub fn named<Name: Into<String>>(mut self, name: Name) -> MaterialBuilder {
		self.name = Some(name.into());
		self
	}

	pub(crate) fn build(self) -> Material {
		match self.name {
			Some(name) => Material::Named {
				name,
				data: self.data.into(),
			},
			None => Material::Unamed(self.data),
		}
	}
}
