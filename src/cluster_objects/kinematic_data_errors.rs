use std::{
	cell::{BorrowError, BorrowMutError},
	collections::HashMap,
	error::Error,
	fmt,
	sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak},
};

use crate::{material::Material, Joint, Link, Transmission};

#[derive(Debug)]
pub enum AddMaterialError {
	#[deprecated]
	Borrow(BorrowError),
	#[deprecated]
	BorrowMut(BorrowMutError),
	ReadMaterial, //(PoisonError<RwLockReadGuard<'a, Material>>),
	ReadIndex,    //(PoisonError<RwLockReadGuard<'a, HashMap<String, Arc<RwLock<Material>>>>>),
	WriteIndex,   //(PoisonError<RwLockWriteGuard<'a, HashMap<String, Arc<RwLock<Material>>>>>),
	Conflict(String),
	/// To be returned when the material has no name to index by.
	NoName,
}

impl fmt::Display for AddMaterialError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			// Self::ReadMaterial(err) => err.fmt(f),
			Self::ReadMaterial => write!(f, "Read material error"),
			// Self::ReadIndex(err) => err.fmt(f),
			Self::ReadIndex => write!(f, "Read MateraialIndex error"),
			// Self::WriteIndex(err) => err.fmt(f),
			Self::WriteIndex => write!(f, "Write MaterialIndex error"),
			Self::Conflict(name) => {
				write!(f, "Duplicate material name '{}'", name)
			}
			Self::NoName => write!(f, "The material has no name, to be used as index."),
		}
	}
}

impl Error for AddMaterialError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
			// Self::ReadMaterial(err) => Some(err),
			Self::ReadMaterial => None,
			// Self::ReadIndex(err) => Some(err),
			Self::ReadIndex => None,
			// Self::WriteIndex(err) => Some(err),
			Self::WriteIndex => None,
			Self::Conflict(_) => None,
			Self::NoName => None,
		}
	}
}

impl From<BorrowError> for AddMaterialError {
	fn from(value: BorrowError) -> Self {
		Self::Borrow(value)
	}
}

impl From<BorrowMutError> for AddMaterialError {
	fn from(value: BorrowMutError) -> Self {
		Self::BorrowMut(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, Material>>> for AddMaterialError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Material>>) -> Self {
		Self::ReadMaterial //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, Arc<RwLock<Material>>>>>>
	for AddMaterialError
{
	fn from(
		_value: PoisonError<RwLockReadGuard<'_, HashMap<String, Arc<RwLock<Material>>>>>,
	) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, Arc<RwLock<Material>>>>>>
	for AddMaterialError
{
	fn from(
		_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, Arc<RwLock<Material>>>>>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

#[derive(Debug)]
pub enum AddLinkError {
	#[deprecated]
	Borrow(BorrowError),
	#[deprecated]
	BorrowMut(BorrowMutError),
	ReadLink,   //(PoisonError<RwLockReadGuard<'a, Link>>),
	ReadIndex,  //(PoisonError<RwLockReadGuard<'a, HashMap<String, Weak<RwLock<Link>>>>>),
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, Weak<RwLock<Link>>>>>),
	Conflict(String),
}

impl fmt::Display for AddLinkError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			// Self::ReadLink(err) => err.fmt(f),
			Self::ReadLink => write!(f, "Read Link error"),
			// Self::ReadIndex(err) => err.fmt(f),
			Self::ReadIndex => write!(f, "Read LinkIndex error"),
			// Self::WriteIndex(err) => err.fmt(f),
			Self::WriteIndex => write!(f, "Write LinkIndex error"),
			Self::Conflict(name) => write!(f, "Duplicate Link name '{}'", name),
		}
	}
}

impl Error for AddLinkError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
			// Self::ReadLink(err) => Some(err),
			Self::ReadLink => None,
			// Self::ReadIndex(err) => Some(err),
			Self::ReadIndex => None,
			// Self::WriteIndex(err) => Some(err),
			Self::WriteIndex => None,
			Self::Conflict(_) => None,
		}
	}
}

impl From<BorrowError> for AddLinkError {
	fn from(value: BorrowError) -> Self {
		Self::Borrow(value)
	}
}

impl From<BorrowMutError> for AddLinkError {
	fn from(value: BorrowMutError) -> Self {
		Self::BorrowMut(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, Link>>> for AddLinkError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Link>>) -> Self {
		Self::ReadLink //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, Weak<RwLock<Link>>>>>> for AddLinkError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, HashMap<String, Weak<RwLock<Link>>>>>) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Link>>>>>> for AddLinkError {
	fn from(
		_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, Weak<RwLock<Link>>>>>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddLinkError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::ReadLink, Self::ReadLink) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub enum AddJointError {
	#[deprecated]
	Borrow(BorrowError),
	#[deprecated]
	BorrowMut(BorrowMutError),
	ReadJoint,  //(PoisonError<RwLockReadGuard<'a, Joint>>),
	ReadIndex,  //(PoisonError<RwLockReadGuard<'a, HashMap<String, Weak<RwLock<Joint>>>>>),
	WriteIndex, //(PoisonError<RwLockWriteGuard<'a, HashMap<String, Weak<RwLock<Joint>>>>>),
	Conflict(String),
}

impl fmt::Display for AddJointError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			// Self::ReadJoint(err) => err.fmt(f),
			Self::ReadJoint => write!(f, "Read Joint Error"),
			// Self::ReadIndex(err) => err.fmt(f),
			Self::ReadIndex => write!(f, "Read JointIndex Error"),
			// Self::WriteIndex(err) => err.fmt(f),
			Self::WriteIndex => write!(f, "Write JointIndex Error"),
			Self::Conflict(name) => write!(f, "Duplicate Joint name '{}'", name),
		}
	}
}

impl Error for AddJointError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
			// Self::ReadJoint(err) => Some(err),
			Self::ReadJoint => None,
			// Self::ReadIndex(err) => Some(err),
			Self::ReadIndex => None,
			// Self::WriteIndex(err) => Some(err),
			Self::WriteIndex => None,
			Self::Conflict(_) => None,
		}
	}
}

impl From<BorrowError> for AddJointError {
	fn from(value: BorrowError) -> Self {
		Self::Borrow(value)
	}
}

impl From<BorrowMutError> for AddJointError {
	fn from(value: BorrowMutError) -> Self {
		Self::BorrowMut(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, Joint>>> for AddJointError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Joint>>) -> Self {
		Self::ReadJoint //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, std::sync::Weak<RwLock<Joint>>>>>>
	for AddJointError
{
	fn from(
		_value: PoisonError<RwLockReadGuard<'_, HashMap<String, std::sync::Weak<RwLock<Joint>>>>>,
	) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, std::sync::Weak<RwLock<Joint>>>>>>
	for AddJointError
{
	fn from(
		_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, std::sync::Weak<RwLock<Joint>>>>>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddJointError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::ReadJoint, Self::ReadJoint) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub enum AddTransmissionError {
	#[deprecated]
	Borrow(BorrowError),
	#[deprecated]
	BorrowMut(BorrowMutError),
	ReadTransmission, //(PoisonError<RwLockReadGuard<'a, Transmission>>),
	ReadIndex,        //(PoisonError<RwLockReadGuard<'a, HashMap<String, Arc<RwLock<Transmission>>>>>),
	WriteIndex,       //(PoisonError<RwLockWriteGuard<'a, HashMap<String, Arc<RwLock<Transmission>>>>>),
	Conflict(String),
}

impl fmt::Display for AddTransmissionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			// Self::ReadTransmission(err) => err.fmt(f),
			Self::ReadTransmission => write!(f, "Read Transmission Error"),
			// Self::ReadIndex(err) => err.fmt(f),
			Self::ReadIndex => write!(f, "Read TransmissionIndex Error"),
			// Self::WriteIndex(err) => err.fmt(f),
			Self::WriteIndex => write!(f, "Write TransmissionIndex Error"),
			Self::Conflict(name) => write!(f, "Duplicate Transmission name '{}'", name),
		}
	}
}

impl Error for AddTransmissionError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
			// Self::ReadTransmission(err) => Some(err),
			Self::ReadTransmission => None,
			// Self::ReadIndex(err) => Some(err),
			Self::ReadIndex => None,
			// Self::WriteIndex(err) => Some(err),
			Self::WriteIndex => None,
			Self::Conflict(_) => None,
		}
	}
}

impl From<BorrowError> for AddTransmissionError {
	fn from(value: BorrowError) -> Self {
		Self::Borrow(value)
	}
}

impl From<BorrowMutError> for AddTransmissionError {
	fn from(value: BorrowMutError) -> Self {
		Self::BorrowMut(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, Transmission>>> for AddTransmissionError {
	fn from(_value: PoisonError<RwLockReadGuard<'_, Transmission>>) -> Self {
		Self::ReadTransmission //(value)
	}
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, Arc<RwLock<Transmission>>>>>>
	for AddTransmissionError
{
	fn from(
		_value: PoisonError<RwLockReadGuard<'_, HashMap<String, Arc<RwLock<Transmission>>>>>,
	) -> Self {
		Self::ReadIndex //(value)
	}
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, Arc<RwLock<Transmission>>>>>>
	for AddTransmissionError
{
	fn from(
		_value: PoisonError<RwLockWriteGuard<'_, HashMap<String, Arc<RwLock<Transmission>>>>>,
	) -> Self {
		Self::WriteIndex //(value)
	}
}

impl PartialEq for AddTransmissionError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::ReadTransmission, Self::ReadTransmission) => true,
			(Self::ReadIndex, Self::ReadIndex) => true,
			(Self::WriteIndex, Self::WriteIndex) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}
