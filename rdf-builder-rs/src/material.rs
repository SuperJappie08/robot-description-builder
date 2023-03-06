#[derive(Debug, PartialEq, Clone)]
pub struct Material {
	pub name: Option<String>,
	material: MaterialData,
}

impl Material {
	/// TODO: FIGURE OUT IF I WANT THIS
	pub fn get_name(&self) -> Option<String> {
		self.name.clone()
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA
	Color(f32, f32, f32, f32),
	/// TODO: TO TEXTURE OR NOT?
	Texture,
}
