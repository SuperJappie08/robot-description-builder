//! A module for yanking errors.
// TODO: RENAME?
use std::sync::PoisonError;

use thiserror::Error;

use crate::{
	utils::{ArcLock, ErroredRead, ErroredWrite},
	Joint, Link,
};

// FIXME: Maybe not transparent?
#[derive(Debug, Error)]
pub enum RebuildBranchError {
	#[error(transparent)]
	ReadChildJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	#[error(transparent)]
	ReadChildLink(#[from] PoisonError<ErroredRead<ArcLock<Link>>>),
}

// FIXME: Maybe not transparent?
/// The errortype for `yank_link` methods.
#[derive(Debug, Error)]
pub enum YankLinkError {
	#[error(transparent)]
	RebuildBranch(#[from] RebuildBranchError),
	/// An Error, that results from `PoisonError<RwLockReadGuard<'_, Joint>>`.
	/// It occurs when an read attempt is made when the parent [`Joint`] of the [`Link`] being yanked is poisoned.
	#[error(transparent)]
	ReadParentJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	/// An Error, that results from `PoisonError<RwLockWriteGuard<'_, Link>>`.
	/// It occurs when the Parent [`Link`] of the Parent [`Joint`] (GrandParent) of the [`Link`] being yanked is poisoned.
	#[error(transparent)]
	WriteGrandParentLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
}

// FIXME: Maybe not transparent?
#[derive(Debug, Error)]
pub enum YankJointError {
	#[error(transparent)]
	RebuildBranch(#[from] RebuildBranchError),
	#[error(transparent)]
	WriteParentLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
	#[error(transparent)]
	ReadYankedJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
}
