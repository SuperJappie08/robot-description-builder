//! A module for yanking errors.
use std::sync::PoisonError;

use thiserror::Error;

use crate::{
	utils::{ArcLock, ErroredRead, ErroredWrite},
	Joint, Link,
};

// FIXME: Maybe not transparent?
/// The errortype for `yank_link` methods.
#[derive(Debug, Error)]
pub enum YankLinkError {
	/// An Error, that results from `PoisonError<RwLockReadGuard<'_, Joint>>`.
	/// It occurs when an read attempt is made when the parent [`Joint`] of the [`Link`] being yanked is poisoned.
	#[error(transparent)]
	ReadParentJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	/// An Error, that results from `PoisonError<RwLockWriteGuard<'_, Link>>`.
	/// It occurs when the Parent [`Link`] of the Parent [`Joint`] (GrandParent) of the [`Link`] being yanked is poisoned.
	#[error(transparent)]
	WriteGrandParentLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
}
