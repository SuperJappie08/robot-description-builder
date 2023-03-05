#[derive(Debug, PartialEq, Clone)]
pub struct Material {
	pub name: Option<String>,
	material: MaterialData,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA
	Color(f32, f32, f32, f32),
	/// TODO: TO TEXTURE OR NOT?
	Texture,
}
