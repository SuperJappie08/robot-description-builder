mod chained;
mod cluster_objects;
mod joint;
mod link;
mod material_mod;
mod transform_data;
mod transmission;

type ArcLock<T> = std::sync::Arc<std::sync::RwLock<T>>;
type WeakLock<T> = std::sync::Weak<std::sync::RwLock<T>>;

pub mod to_rdf;
pub use chained::Chained;
pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{Joint, JointBuilder, JointType, SmartJointBuilder};
pub use link::{helper_functions, link_data, Link};
pub use material_mod::{Material, MaterialBuilder, MaterialData};
pub use transform_data::Transform;

pub mod linkbuilding {
	use super::link;
	pub use link::builder::*;
}

#[cfg(test)]
mod tests {
	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
