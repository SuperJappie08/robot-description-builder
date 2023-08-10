use thiserror::Error;

use crate::{
	cluster_objects::kinematic_data_errors::PoisonReadIndexError, joint::Joint, utils::WeakLock,
};

#[derive(Debug, Error)]
pub enum BuildTransmissionError {
	#[error("Read TransmissionIndex Error: {0}")]
	ReadJointIndex(#[from] PoisonReadIndexError<String, WeakLock<Joint>>),
	#[error("Could not find Joint \"{0}\"")]
	InvalidJoint(String),
}

impl PartialEq for BuildTransmissionError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadJointIndex(l0), Self::ReadJointIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::InvalidJoint(l0), Self::InvalidJoint(r0)) => l0 == r0,
			_ => false,
		}
	}
}
