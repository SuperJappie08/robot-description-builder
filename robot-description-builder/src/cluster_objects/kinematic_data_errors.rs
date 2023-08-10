use thiserror::Error;

use std::{
	collections::HashMap,
	sync::{Arc, PoisonError},
};

use crate::{
	joint::Joint,
	link::Link,
	material::data::MaterialData,
	transmission::{BuildTransmissionError, Transmission},
	utils::{ArcLock, ErroredRead, ErroredWrite, WeakLock},
};

use super::kinematic_data_tree::KinematicDataTree;

pub(crate) type PoisonReadIndexError<K, V> = PoisonError<ErroredRead<ArcLock<HashMap<K, V>>>>;
pub(crate) type PoisonWriteIndexError<K, V> = PoisonError<ErroredWrite<ArcLock<HashMap<K, V>>>>;

#[derive(Debug, Error)]
pub enum AddMaterialError {
	/// Error that results from `PoisonError<RwLockReadGuard<'_, MaterialData>>` occurs when attempting to read a poisoned `Arc<RwLock<MaterialData>>`.
	#[error("Read Material Error: {0}")]
	ReadMaterial(#[from] PoisonError<ErroredRead<ArcLock<MaterialData>>>),
	/// Error that results from `PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<MaterialData>>>>` occurs when attempting to read a poisoned `HashMap<String, ArcLock<MaterialData>>`.
	#[error("Read MaterialIndex Error: {0}")]
	ReadIndex(#[from] PoisonReadIndexError<String, ArcLock<MaterialData>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<MaterialData>>>>` occurs when attempting to write to a poisoned `HashMap<String, ArcLock<MaterialData>>`.
	#[error("Write MaterialIndex Error: {0}")]
	WriteIndex(#[from] PoisonWriteIndexError<String, ArcLock<MaterialData>>),
	#[error("Duplicate Material name '{0}'")]
	Conflict(String),
}

impl PartialEq for AddMaterialError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadMaterial(l0), Self::ReadMaterial(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddLinkError {
	/// Error that results from `PoisonError<RwLockReadGuard<'_, Link>>` occurs when attempting to read a poisoned `Arc<RwLock<Link>>`.
	#[error("Read Link Error: {0}")]
	ReadLink(#[from] PoisonError<ErroredRead<ArcLock<Link>>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, Link>>` occurs when attempting to write to a poisoned `Arc<RwLock<Link>>`.
	#[error("Write Link Error: {0}")]
	WriteLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Link>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>>`.
	#[error("Read LinkIndex Error: {0}")]
	ReadIndex(#[from] PoisonReadIndexError<String, WeakLock<Link>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Link>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>>`.
	#[error("Write LinkIndex Error: {0}")]
	WriteIndex(#[from] PoisonWriteIndexError<String, WeakLock<Link>>),
	#[error("Duplicate Link name '{0}'")]
	Conflict(String),
	#[error(transparent)]
	AddJoint(#[from] Box<AddJointError>),
	#[error(transparent)]
	AddMaterial(#[from] AddMaterialError),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, Weak<RwLock<Link>>>>` occurs when attempting to write to a poisoned `RwLock<Weak<RwLock<Link>>>>`. (Only used for `KinematicDataTree``.newest_link`)
	#[error("Accesses `newest_link` failed: {0}")]
	AccesNewestLink(#[from] PoisonError<ErroredWrite<Arc<KinematicDataTree>>>),
}

impl PartialEq for AddLinkError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadLink(l0), Self::ReadLink(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			(Self::AddJoint(l0), Self::AddJoint(r0)) => *l0 == *r0,
			(Self::AddMaterial(l0), Self::AddMaterial(r0)) => l0 == r0,
			(Self::AccesNewestLink(l0), Self::AccesNewestLink(r0)) => l0.get_ref() == r0.get_ref(),
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddJointError {
	/// Error that results from `PoisonError<RwLockReadGuard<'_, Joint>>` occurs when attempting to read a poisoned `Arc<RwLock<Joint>>`.
	#[error("Read Joint Error: {0}")]
	ReadJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Joint>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>`.
	#[error("Read JointIndex Error: {0}")]
	ReadIndex(#[from] PoisonReadIndexError<String, WeakLock<Joint>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Joint>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>`.
	#[error("Write JointIndex Error: {0}")]
	WriteIndex(#[from] PoisonWriteIndexError<String, WeakLock<Joint>>),
	#[error("Duplicate Joint name '{0}'")]
	Conflict(String),
	#[error(transparent)]
	AddLink(#[from] Box<AddLinkError>),
}

impl PartialEq for AddJointError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadJoint(l0), Self::ReadJoint(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			(Self::AddLink(l0), Self::AddLink(r0)) => *l0 == *r0,
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddTransmissionError {
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Transmission>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Transmission>>>>>`.
	#[error("Read TransmissionIndex Error: {0}")]
	ReadIndex(#[from] PoisonReadIndexError<String, ArcLock<Transmission>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Transmission>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Transmission>>>>>`.
	#[error("Write TransmissionIndex Error: {0}")]
	WriteIndex(#[from] PoisonWriteIndexError<String, ArcLock<Transmission>>),
	#[error("Duplicate Transmission name '{0}'")]
	Conflict(String),
	#[error(transparent)]
	BuildTransmission(#[from] BuildTransmissionError),
}

impl PartialEq for AddTransmissionError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			(Self::BuildTransmission(l0), Self::BuildTransmission(r0)) => l0 == r0,
			_ => false,
		}
	}
}
