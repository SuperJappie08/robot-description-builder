mod cluster_objects;
mod joint;
mod link;
mod material;

pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{Joint, JointBuilder, JointInterface, JointType, OffsetMode, SmartJointBuilder};
pub use link::{helper_functions, Link};
pub use material::{Material, MaterialData};

#[derive(Debug, PartialEq, Eq)]
/// TODO: IMPLEMENT PROPPERLY, THIS IS TEMPORARY
pub struct Transmission {
	pub name: String,
}

#[cfg(test)]
mod tests {
	// use super::*;

	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
