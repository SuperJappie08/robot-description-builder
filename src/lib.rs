mod cluster_objects;
mod joint;
mod link;

pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{Joint, JointType};
pub use link::Link;

//tmp
#[derive(Debug, PartialEq, Eq)]
pub struct Material {
	pub name: String,
}

//tmp
#[derive(Debug, PartialEq, Eq)]
struct Transmission;

pub fn add(left: usize, right: usize) -> usize {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}
