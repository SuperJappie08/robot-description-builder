// use std::sync::{PoisonError, RwLock, Arc, Weak};

// use pyo3::PyErr;

// #[derive(Debug)]
// pub struct ErroredRead<T>(pub T);

// #[inline]
// pub(crate) fn errored_read_lock<T>(
// 	errored_lock: &Arc<RwLock<T>>,
// ) -> PoisonError<ErroredRead<Arc<RwLock<T>>>> {
// 	PoisonError::new(ErroredRead(Arc::clone(errored_lock)))
// }

// impl<T> PartialEq for ErroredRead<Arc<T>> {
// 	fn eq(&self, other: &Self) -> bool {
// 		Arc::ptr_eq(&self.0, &other.0)
// 	}
// }

// impl<T> PartialEq for ErroredRead<Weak<T>> {
// 	fn eq(&self, other: &Self) -> bool {
// 		Weak::ptr_eq(&self.0, &other.0)
// 	}
// }

// #[derive(Debug)]
// pub struct ErroredWrite<T>(pub T);

// #[inline]
// pub(crate) fn errored_write_lock<T>(
// 	errored_lock: &Arc<RwLock<T>>,
// ) -> PoisonError<ErroredWrite<Arc<RwLock<T>>>> {
// 	PoisonError::new(ErroredWrite(Arc::clone(errored_lock)))
// }

// impl<T> PartialEq for ErroredWrite<Arc<T>> {
// 	fn eq(&self, other: &Self) -> bool {
// 		Arc::ptr_eq(&self.0, &other.0)
// 	}
// }

// impl<T> PartialEq for ErroredWrite<Weak<T>> {
// 	fn eq(&self, other: &Self) -> bool {
// 		Weak::ptr_eq(&self.0, &other.0)
// 	}
// }
