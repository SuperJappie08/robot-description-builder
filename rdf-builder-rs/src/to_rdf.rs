#[cfg(feature = "urdf")]
pub mod to_urdf;

#[cfg(feature = "sdf")]
pub mod to_sdf;

#[cfg(not(feature = "urdf"))]
/// This is the empty version for trait bounds
pub mod to_urdf {
	pub trait ToURDF {}
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum XMLMode {
	#[default]
	NoIndent,
	Indent(char, usize),
}
