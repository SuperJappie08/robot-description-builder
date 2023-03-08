use thiserror::Error;

use std::{
	collections::HashMap,
	sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
	joint::JointInterface, link::Link, material::Material, ArcLock, Transmission, WeakLock,
};

#[derive(Debug, Error)]
pub enum AddMaterialError {
	#[error("Read Material Error")]
	ReadMaterial, //(PoisonError<RwLockReadGuard<'a, Material>>),
	#[error("Read MaterialIndex Error")]
	ReadIndex, //(PoisonError<RwLockReadGuard<'a, HashMap<String, ArcLock<Material>>>>),
	#[error("Write MaterialIndex Error")]
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, ArcLock<Material>>>>),
	#[error("Duplicate Material name '{0}'")]
	Conflict(String),
	/// To be returned when the material has no name to index by.
	#[error("The material has no name, to be used as index.")]
	NoName,
}

impl From<PoisonError<RwLockReadGuard<'_, Material>>> for AddMaterialError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Material>>) -> Self {
		Self::ReadMaterial //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<Material>>>>>
	for AddMaterialError
{
	fn from(_value: PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<Material>>>>) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<Material>>>>>
	for AddMaterialError
{
	fn from(_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<Material>>>>) -> Self {
		Self::WriteIndex //(value)
	}
}

#[derive(Debug, Error)]
pub enum AddLinkError {
	#[error("Read Link Error")]
	ReadLink, //(PoisonError<RwLockReadGuard<'a, Link>>),
	#[error("Read LinkIndex Error")]
	ReadIndex, //(PoisonError<RwLockReadGuard<'a, HashMap<String, WeakLock<Link>>>>),
	#[error("Write LinkIndex Error")]
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, WeakLock<Link>>>>),
	#[error("Duplicate Link name '{0}'")]
	Conflict(String),
}

impl From<PoisonError<RwLockReadGuard<'_, Link>>> for AddLinkError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Link>>) -> Self {
		Self::ReadLink //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, WeakLock<Link>>>>> for AddLinkError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, HashMap<String, WeakLock<Link>>>>) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, WeakLock<Link>>>>> for AddLinkError {
	fn from(_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, WeakLock<Link>>>>) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddLinkError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadLink, Self::ReadLink) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddJointError {
	#[error("Read Joint Error")]
	ReadJoint, //(PoisonError<RwLockReadGuard<'a, Joint>>),
	#[error("Read JointIndex Error")]
	ReadIndex, //(PoisonError<RwLockReadGuard<'a, HashMap<String, WeakLock<Joint>>>>),
	#[error("Write JointIndex Error")]
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, WeakLock<Joint>>>>),
	#[error("Duplicate Joint name '{0}'")]
	Conflict(String),
}

impl From<PoisonError<RwLockReadGuard<'_, Box<dyn JointInterface + Sync + Send>>>>
	for AddJointError
{
	fn from(
		_value: PoisonError<RwLockReadGuard<'_, Box<dyn JointInterface + Sync + Send>>>,
	) -> Self {
		Self::ReadJoint //(value)
	}
}

impl
	From<
		PoisonError<
			RwLockReadGuard<'_, HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
		>,
	> for AddJointError
{
	fn from(
		_value: PoisonError<
			RwLockReadGuard<'_, HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
		>,
	) -> Self {
		Self::ReadIndex //(value)
	}
}

impl
	From<
		PoisonError<
			RwLockWriteGuard<'_, HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
		>,
	> for AddJointError
{
	fn from(
		_value: PoisonError<
			RwLockWriteGuard<'_, HashMap<String, WeakLock<Box<dyn JointInterface + Sync + Send>>>>,
		>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddJointError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadJoint, Self::ReadJoint) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug, Error)]
pub enum AddTransmissionError {
	#[error("Read Transmission Error")]
	ReadTransmission, //(PoisonError<RwLockReadGuard<'a, Transmission>>),
	#[error("Read TransmissionIndex Error")]
	ReadIndex, //(PoisonError<RwLockReadGuard<'a, HashMap<String, ArcLock<Transmission>>>>),
	#[error("Write TransmissionIndex Error")]
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, ArcLock<Transmission>>>>),
	#[error("Duplicate Transmission name '{0}'")]
	Conflict(String),
}

impl From<PoisonError<RwLockReadGuard<'_, Transmission>>> for AddTransmissionError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Transmission>>) -> Self {
		Self::ReadTransmission //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<Transmission>>>>>
	for AddTransmissionError
{
	fn from(
		_value: PoisonError<RwLockReadGuard<'_, HashMap<String, ArcLock<Transmission>>>>,
	) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<Transmission>>>>>
	for AddTransmissionError
{
	fn from(
		_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, ArcLock<Transmission>>>>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddTransmissionError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::ReadTransmission, Self::ReadTransmission) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}
