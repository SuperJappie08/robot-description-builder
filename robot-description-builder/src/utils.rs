//! A module containing utilities for simplifing the use of `Arc<RwLock<T>>`.
use std::sync::{Arc, PoisonError, RwLockReadGuard, RwLockWriteGuard, Weak};

pub(crate) type ArcLock<T> = std::sync::Arc<std::sync::RwLock<T>>;
pub(crate) type WeakLock<T> = std::sync::Weak<std::sync::RwLock<T>>;

/// A type to signify the Error occured, while reading the contained value.
#[derive(Debug)]
#[repr(transparent)]
pub struct ErroredRead<T>(pub T);

/// A utility trait, which adds read and write methods with better errrors.
pub(crate) trait ArcRW: Sized {
	/// The target type of the `ArcLock`.
	///
	/// Must be equal to the generic of [`ArcLock`].
	type Target;

	/// Reads the [`ArcLock`] and converting the error to something reportable.
	///
	/// Equivalent to [`RwLock::read`](std::sync::RwLock::read()), with a more useable error.
	fn mread(&self) -> Result<RwLockReadGuard<'_, Self::Target>, PoisonError<ErroredRead<Self>>>;
	/// Writes to the [`ArcLock`] and converting the error to something reportable.
	///
	/// Equivalent to [`RwLock::write`](std::sync::RwLock::write()), with a more useable error.
	fn mwrite(&self)
		-> Result<RwLockWriteGuard<'_, Self::Target>, PoisonError<ErroredWrite<Self>>>;
}

impl<T> ArcRW for ArcLock<T> {
	type Target = T;

	#[inline]
	fn mread(&self) -> Result<RwLockReadGuard<'_, Self::Target>, PoisonError<ErroredRead<Self>>> {
		self.read().map_err(|_| errored_read_lock(self))
	}

	#[inline]
	fn mwrite(
		&self,
	) -> Result<RwLockWriteGuard<'_, Self::Target>, PoisonError<ErroredWrite<Self>>> {
		self.write().map_err(|_| errored_write_lock(self))
	}
}

/// Create an error from a reference to the `ArcLock<T>` if an error occurred while attempting to acquire a read guard to the lock.
#[inline]
pub(crate) fn errored_read_lock<T>(
	errored_lock: &ArcLock<T>,
) -> PoisonError<ErroredRead<ArcLock<T>>> {
	PoisonError::new(ErroredRead(Arc::clone(errored_lock)))
}

impl<T> PartialEq for ErroredRead<Arc<T>> {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.0, &other.0)
	}
}

impl<T> PartialEq for ErroredRead<Weak<T>> {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.0, &other.0)
	}
}

/// A type to signify the Error occured, while writing to the contained value.
#[derive(Debug)]
#[repr(transparent)]
pub struct ErroredWrite<T>(pub T);

/// Create an error from a reference to the `ArcLock<T>` if an error occurred while attempting to acquire a write guard to the lock.
#[inline]
pub(crate) fn errored_write_lock<T>(
	errored_lock: &ArcLock<T>,
) -> PoisonError<ErroredWrite<ArcLock<T>>> {
	PoisonError::new(ErroredWrite(Arc::clone(errored_lock)))
}

impl<T> PartialEq for ErroredWrite<Arc<T>> {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.0, &other.0)
	}
}

impl<T> PartialEq for ErroredWrite<Weak<T>> {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.0, &other.0)
	}
}
