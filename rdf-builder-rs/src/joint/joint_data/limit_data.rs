#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct LimitData {
	pub lower: Option<f32>,
	pub upper: Option<f32>,
	pub effort: f32,
	pub velocity: f32,
}
