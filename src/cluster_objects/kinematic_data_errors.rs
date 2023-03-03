use std::{
	cell::{BorrowError, BorrowMutError},
	error::Error,
	fmt,
};

#[derive(Debug)]
pub enum AddMaterialError {
	Borrow(BorrowError),
	BorrowMut(BorrowMutError),
	Conflict(String),
	/// To be returned when the material has no name to index by.
	NoName,
}

impl fmt::Display for AddMaterialError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
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

#[derive(Debug)]
pub enum AddLinkError {
	Borrow(BorrowError),
	BorrowMut(BorrowMutError),
	Conflict(String),
}

impl fmt::Display for AddLinkError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			Self::Conflict(name) => write!(f, "Duplicate Link name '{}'", name),
		}
	}
}

impl Error for AddLinkError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
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

impl PartialEq for AddLinkError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

#[derive(Debug)]
pub enum AddJointError {
	Borrow(BorrowError),
	BorrowMut(BorrowMutError),
	Conflict(String),
}

impl fmt::Display for AddJointError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Borrow(err) => err.fmt(f),
			Self::BorrowMut(err) => err.fmt(f),
			Self::Conflict(name) => write!(f, "Duplicate Joint name '{}'", name),
		}
	}
}

impl Error for AddJointError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Borrow(err) => Some(err),
			Self::BorrowMut(err) => Some(err),
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

impl PartialEq for AddJointError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}
