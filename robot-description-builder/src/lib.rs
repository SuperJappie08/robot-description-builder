#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![warn(rustdoc::redundant_explicit_links)]
// #![warn(missing_docs)]
// This is fine for now.
#![doc = include_str!("../README.md")]

mod chained;
mod cluster_objects;
mod joint;
mod link;
mod transform;
mod utils;
mod yank_errors;

pub mod identifiers;
pub mod material;
pub mod to_rdf;
#[doc(hidden)]
// FIXME: Hidden until correctly implemented
pub mod transmission;
pub use chained::Chained;
pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{joint_data, Joint, JointBuilder, JointType, SmartJointBuilder};
pub use link::{helper_functions, link_data, Link};
pub use transform::{MirrorAxis, Transform};

pub mod linkbuilding {
	pub use super::link::builder::*;
}

// TODO: Docs
pub mod prelude {
	pub use super::cluster_objects::KinematicInterface;
	pub use super::identifiers::GroupIDChanger;
	// TODO: maybe add builders to prelude?
	// pub use joint::{SmartJointBuilder};
	// pub use material::MaterialDescriptor;
}

/// Error-types used in `robot-description-builder`.
pub mod errors {
	pub use super::cluster_objects::kinematic_data_errors::{
		AddJointError, AddLinkError, AddMaterialError, AddTransmissionError, AttachChainError,
	};
	pub use super::identifiers::GroupIDError;
	pub use super::yank_errors::{RebuildBranchError, YankJointError, YankLinkError};
}

/// The important libraries re-exported.
#[cfg(feature = "wrapper")]
pub mod reexport {
	pub use nalgebra;
	#[cfg(feature = "xml")]
	pub use quick_xml;
}

#[cfg(feature = "smart-joint-extension")]
/// The types required when extending `SmartJointBuilder` functionality.
// TODO: EXPAND
pub mod smart_joint_extension {
	pub mod types {
		pub use crate::joint::smartjointbuilder::{
			ContinuousType, FixedType, FloatingType, NoType, PlanarType, PrismaticType,
			RevoluteType,
		};
	}
	pub use super::joint::smartjointbuilder::smartparams;
}

#[cfg(test)]
mod tests {
	// #![allow(missing_docs)]
	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
