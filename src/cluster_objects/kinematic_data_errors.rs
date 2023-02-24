use std::{cell::BorrowError, error::Error, fmt};

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
pub enum TryAddDataError {
	Borrow(BorrowError),
	Conflict(String),
}

impl fmt::Display for TryAddDataError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TryAddDataError::Borrow(err) => err.fmt(f),
			TryAddDataError::Conflict(name) => write!(f, "Duplicate name '{}'", name),
		}
	}
}

impl Error for TryAddDataError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TryAddDataError::Borrow(err) => Some(err),
			TryAddDataError::Conflict(_) => None,
		}
	}
}

impl From<BorrowError> for TryAddDataError {
	fn from(value: BorrowError) -> Self {
		TryAddDataError::Borrow(value)
	}
}
