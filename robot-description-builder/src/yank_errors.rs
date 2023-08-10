use std::sync::PoisonError;

use thiserror::Error;

use crate::{
	utils::{ArcLock, ErroredRead, ErroredWrite},
	Joint, Link,
};

/// FIXME: Maybe not transparent?
#[derive(Debug, Error)]
pub enum YankLinkError {
	#[error(transparent)]
	ReadParentJoint(#[from] PoisonError<ErroredRead<ArcLock<Joint>>>),
	#[error(transparent)]
	WriteGrandParentLink(#[from] PoisonError<ErroredWrite<ArcLock<Link>>>),
}
