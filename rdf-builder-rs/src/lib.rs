mod chained;
mod cluster_objects;
mod joint;
mod link;
mod transform;

type ArcLock<T> = std::sync::Arc<std::sync::RwLock<T>>;
type WeakLock<T> = std::sync::Weak<std::sync::RwLock<T>>;

pub mod identifiers;
pub mod material;
pub mod to_rdf;
pub mod transmission;
pub use chained::Chained;
pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{joint_data, Joint, JointBuilder, JointType, SmartJointBuilder};
pub use link::{helper_functions, link_data, Link};
pub use transform::{MirrorAxis, Transform};

pub mod linkbuilding {
	use super::link;
	pub use link::builder::*;
}

/// TODO: Docs
pub mod prelude {
	pub use super::cluster_objects::KinematicInterface;
	pub use super::identifiers::GroupIDChanger;
	// TODO: maybe add builders to prelude?
	// pub use joint::{SmartJointBuilder};
	// pub use material::MaterialDescriptor;
}

#[cfg(test)]
mod tests {
	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
