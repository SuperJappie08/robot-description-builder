pub mod to_urdf;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum XMLMode {
	#[default]
	NoIndent,
	Indent(char, usize),
}
