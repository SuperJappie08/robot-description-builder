use std::{
	cell::{BorrowError, BorrowMutError},
	error::Error,
	fmt,
};

#[derive(Debug)]
pub enum TryAddMaterialError {
	Borrow(BorrowError),
	MaterialConflict(String),
}

impl fmt::Display for TryAddMaterialError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TryAddMaterialError::Borrow(err) => err.fmt(f),
			TryAddMaterialError::MaterialConflict(name) => {
				write!(f, "Duplicate material name '{}'", name)
			}
		}
	}
}

impl Error for TryAddMaterialError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TryAddMaterialError::Borrow(err) => Some(err),
			TryAddMaterialError::MaterialConflict(_) => None,
		}
	}
}

impl From<BorrowError> for TryAddMaterialError {
	fn from(value: BorrowError) -> Self {
		TryAddMaterialError::Borrow(value)
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

#[derive(Debug)]
pub enum TryAddDataError {
	Borrow(BorrowError),
	BorrowMut(BorrowMutError),
	Conflict(String),
}

impl fmt::Display for TryAddDataError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TryAddDataError::Borrow(err) => err.fmt(f),
			TryAddDataError::BorrowMut(err) => err.fmt(f),
			TryAddDataError::Conflict(name) => write!(f, "Duplicate name '{}'", name),
		}
	}
}

impl Error for TryAddDataError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TryAddDataError::Borrow(err) => Some(err),
			TryAddDataError::BorrowMut(err) => Some(err),
			TryAddDataError::Conflict(_) => None,
		}
	}
}

impl From<BorrowError> for TryAddDataError {
	fn from(value: BorrowError) -> Self {
		TryAddDataError::Borrow(value)
	}
}

impl From<BorrowMutError> for TryAddDataError {
	fn from(value: BorrowMutError) -> Self {
		TryAddDataError::BorrowMut(value)
	}
}

impl PartialEq for TryAddDataError {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Borrow(_), Self::Borrow(_)) => true,
			(Self::BorrowMut(_), Self::BorrowMut(_)) => true,
			(Self::Conflict(l0), Self::Conflict(r0)) => l0 == r0,
			_ => false,
		}
	}
}

/// An error returned by [`KinematicTreeData::try_merge`].
///
/// TODO: Finish
#[derive(Debug, PartialEq)]
pub enum TryMergeTreeError {
	TryAddData(TryAddDataError),
}

impl fmt::Display for TryMergeTreeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TryMergeTreeError::TryAddData(err) => err.fmt(f),
		}
	}
}

impl Error for TryMergeTreeError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TryMergeTreeError::TryAddData(err) => Some(err),
		}
	}
}

impl From<TryAddDataError> for TryMergeTreeError {
	fn from(value: TryAddDataError) -> Self {
		Self::TryAddData(value)
	}
}
