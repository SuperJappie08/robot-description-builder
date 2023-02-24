mod kinematic_data_errors;
pub mod kinematic_tree;
pub(crate) mod kinematic_tree_data;

pub trait KinematicInterface {
	// NOTE: THIS IS NOT FINAL;
	fn merge<Kinematic: KinematicInterface>(&mut self, other: Kinematic) -> Self;
}
