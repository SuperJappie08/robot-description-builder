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

// TODO: Improve Doc
#[derive(Debug, Error)]
pub enum AddMaterialError {
	//TODO: IMPR DOC
	/// Error that results from `PoisonError<RwLockReadGuard<'_, MaterialData>>` occurs when attempting to read a poisoned `Arc<RwLock<MaterialData>>`.
	#[error("The lock of the new Material is poisoned and therefore could not be read")]
	ReadMaterial(#[from] PoisonError<ErroredRead<ArcLock<MaterialData>>>),
	//TODO: IMPR DOC
	/// Error that results from `PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<MaterialData>>>>` occurs when attempting to read a poisoned `HashMap<String, ArcLock<MaterialData>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index (Might lose some data), however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the Material Index is poisoned and therefore could not be read")]
	ReadIndex(#[from] PoisonReadIndexError<String, ArcLock<MaterialData>>),
	//TODO: IMPR DOC
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<MaterialData>>>>` occurs when attempting to write to a poisoned `HashMap<String, ArcLock<MaterialData>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index (Might lose some data), however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the Material Index is poisoned and therefore could be written to")]
	WriteIndex(#[from] PoisonWriteIndexError<String, ArcLock<MaterialData>>),
	/// An Error, which occurse when a named `Material` is being registered and the name is already in use by another `Material` with a different `MaterialDescription`.
	#[error("The Material name '{0}' is already in use by another Material with non-matching descriptions")]
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
	#[error("The lock of the new Link is poisoned and therefore could not be read")]
	ReadNewLink(#[from] PoisonError<ErroredRead<ArcLock<Link>>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, Link>>` occurs when attempting to write to a poisoned `Arc<RwLock<Link>>`.
	#[error("The lock of the new Link is poisoned and therefore could not be written to")]
	WriteNewLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Link>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index, however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the Link Index is poisoned and therefor could not be read")]
	ReadIndex(#[from] PoisonReadIndexError<String, WeakLock<Link>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Link>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index, however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the Link Index is poisoned and therefor could not be written to")]
	WriteIndex(#[from] PoisonWriteIndexError<String, WeakLock<Link>>),
	#[error(
		"The new Link could not be added since its name '{0}' is already in use by another Link"
	)]
	Conflict(String),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, Weak<RwLock<Link>>>>` occurs when attempting to write to a poisoned `RwLock<Weak<RwLock<Link>>>>`. (Only used for `KinematicDataTree``.newest_link`).
	/* In the future the lock could be saved by overwriting with a newly generated index, however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the `newest_link` on the KinematicDataTree is poisoned and therefore could not be accessed")]
	AccessNewestLink(#[from] PoisonError<ErroredWrite<Arc<KinematicDataTree>>>),
}

impl PartialEq for AddLinkError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadNewLink(l0), Self::ReadNewLink(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			(Self::AccessNewestLink(l0), Self::AccessNewestLink(r0)) => {
				l0.get_ref() == r0.get_ref()
			}
			_ => false,
		}
	}
}

// TODO: Improve Doc
#[derive(Debug, Error)]
pub enum AddJointError {
	/// Error that results from `PoisonError<RwLockReadGuard<'_, Joint>>` occurs when attempting to read a poisoned `Arc<RwLock<Joint>>`.
	#[error("The lock of the new Joint is poisoned and therefore could not be read")]
	ReadNewJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Joint>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index, however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of the Joint Index is poisoned and therefore could not be read")]
	ReadIndex(#[from] PoisonReadIndexError<String, WeakLock<Joint>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Joint>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>`.
	/* In the future the lock could be saved by overwriting with a newly generated index, however waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("The lock of Joint Index is poisoned and therefore could not be written to")]
	WriteIndex(#[from] PoisonWriteIndexError<String, WeakLock<Joint>>),
	#[error(
		"The new Joint could not be added since its name '{0}' is already in use by another Joint"
	)]
	Conflict(String),
}

impl PartialEq for AddJointError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadNewJoint(l0), Self::ReadNewJoint(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::ReadIndex(l0), Self::ReadIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::WriteIndex(l0), Self::WriteIndex(r0)) => l0.get_ref() == r0.get_ref(),
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddTransmissionError {
	/// Error that results from `PoisonError<RwLockReaddGuard<'_, HashMap<String, Weak<RwLock<Transmission>>>>` occurs when attempting to read a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Transmission>>>>>`.
	/* In the future this lock might be saveable but waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
	#[error("Read TransmissionIndex Error: {0}")]
	ReadIndex(#[from] PoisonReadIndexError<String, ArcLock<Transmission>>),
	/// Error that results from `PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Transmission>>>>` occurs when attempting to write to a poisoned `Arc<RwLock<HashMap<String, Weak<RwLock<Transmission>>>>>`.
	/* In the future this lock might be saveable but waiting for
	"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
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

#[derive(Debug, PartialEq, Error)]
pub enum AttachChainError {
	#[error("An error occured when registering a Link: {0}")]
	Link(#[from] AddLinkError),
	#[error("An error occured when registering a Joint: {0}")]
	Joint(#[from] AddJointError),
	#[error("An error occured when registering a Material: {0}")]
	Material(#[from] AddMaterialError),
}

impl From<PoisonError<ErroredWrite<Arc<KinematicDataTree>>>> for AttachChainError {
	fn from(value: PoisonError<ErroredWrite<Arc<KinematicDataTree>>>) -> Self {
		Self::Link(value.into())
	}
}
