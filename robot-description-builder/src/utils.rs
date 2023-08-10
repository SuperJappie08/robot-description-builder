//! TODO: Internal DOC
//!
use std::sync::{Arc, PoisonError, RwLockReadGuard, RwLockWriteGuard, Weak};

pub(crate) type ArcLock<T> = std::sync::Arc<std::sync::RwLock<T>>;
pub(crate) type WeakLock<T> = std::sync::Weak<std::sync::RwLock<T>>;

#[derive(Debug)]
#[repr(transparent)]
pub struct ErroredRead<T>(pub T);

#[inline]
pub(crate) fn errored_read_lock<T>(
	errored_lock: &ArcLock<T>,
) -> PoisonError<ErroredRead<ArcLock<T>>> {
	PoisonError::new(ErroredRead(Arc::clone(errored_lock)))
}

#[inline]
pub(crate) fn read_arclock<T>(
	arclock: &ArcLock<T>,
) -> Result<RwLockReadGuard<'_, T>, PoisonError<ErroredRead<ArcLock<T>>>> {
	arclock.read().map_err(|_| errored_read_lock(arclock))
}

// /// Upgrades and unwraps `WeakLock<T>` then read.
// ///
// /// # Safety
// /// The Upgrade must be valid
// #[inline]
// pub(crate) fn read_weaklock<'a, T>(weaklock: &'a WeakLock<T>) -> Result<RwLockReadGuard<'a, T>,PoisonError<ErroredRead<ArcLock<T>>>> {
// 	// let arclock =
//     weaklock.upgrade().unwrap().read().map_err(|_| errored_read_lock(&weaklock.upgrade().unwrap()))
// }

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

#[derive(Debug)]
#[repr(transparent)]
pub struct ErroredWrite<T>(pub T);

#[inline]
pub(crate) fn errored_write_lock<T>(
	errored_lock: &ArcLock<T>,
) -> PoisonError<ErroredWrite<ArcLock<T>>> {
	PoisonError::new(ErroredWrite(Arc::clone(errored_lock)))
}

#[inline]
pub(crate) fn write_arclock<T>(
	arclock: &ArcLock<T>,
) -> Result<RwLockWriteGuard<'_, T>, PoisonError<ErroredWrite<ArcLock<T>>>> {
	arclock.write().map_err(|_| errored_write_lock(arclock))
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
