use std::sync::Arc;

use super::MaterialData;
use crate::ArcLock;

/// A wrapper for [`MaterialData`] references.
///
/// This is neccessary for the global [`Material`](super::Material) implementation.
#[derive(Debug)]
pub enum MaterialDataReferenceWrapper<'a> {
	/// A normal Reference to a [`MaterialData`] of an unnamed/unshared [`Material`](super::Material)
	Direct(&'a MaterialData),
	/// A Global Reference to a [`MaterialData`] of a named/shared [`Material`](super::Material) via a `Arc<RwLock<T>>`.
	Global(ArcLock<MaterialData>),
}

impl<'a> MaterialDataReferenceWrapper<'a> {
	/// Check if the two referenced [`MaterialData`] structs describe the same appearance.
	pub fn same_material_data(&self, other: &MaterialDataReferenceWrapper) -> bool {
		match (self, other) {
			(
				MaterialDataReferenceWrapper::Direct(left),
				MaterialDataReferenceWrapper::Direct(right),
			) => left == right,
			(
				MaterialDataReferenceWrapper::Direct(left),
				MaterialDataReferenceWrapper::Global(right),
			) => (*left).clone() == right.read().unwrap().clone(), // FIXME: Unwrap not OK
			(
				MaterialDataReferenceWrapper::Global(left),
				MaterialDataReferenceWrapper::Direct(right),
			) => (*right).clone() == left.read().unwrap().clone(), // FIXME: Unwrap not OK
			(
				MaterialDataReferenceWrapper::Global(left),
				MaterialDataReferenceWrapper::Global(right),
			) => left.read().unwrap().clone() == right.read().unwrap().clone(), // FIXME: Unwrap not OK
		}
	}
}

impl<'a> PartialEq for MaterialDataReferenceWrapper<'a> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Direct(l0), Self::Direct(r0)) => l0 == r0,
			(Self::Global(l0), Self::Global(r0)) => Arc::ptr_eq(l0, r0),
			_ => false,
		}
	}
}

impl<'a> From<&'a MaterialData> for MaterialDataReferenceWrapper<'a> {
	fn from(value: &'a MaterialData) -> Self {
		Self::Direct(value)
	}
}

impl<'a> From<ArcLock<MaterialData>> for MaterialDataReferenceWrapper<'a> {
	fn from(value: ArcLock<MaterialData>) -> Self {
		MaterialDataReferenceWrapper::Global(value)
	}
}

impl<'a> TryFrom<MaterialDataReferenceWrapper<'a>> for MaterialData {
	type Error = std::sync::PoisonError<ArcLock<MaterialData>>;

	fn try_from(value: MaterialDataReferenceWrapper) -> Result<Self, Self::Error> {
		match value {
			MaterialDataReferenceWrapper::Direct(data) => Ok(data.clone()),
			MaterialDataReferenceWrapper::Global(arc_data) => {
				let data_ref = arc_data
					.read()
					.map(|data| data.clone())
					.map_err(|_| std::sync::PoisonError::new(Arc::clone(&arc_data)));
				data_ref
			}
		}
	}
}
