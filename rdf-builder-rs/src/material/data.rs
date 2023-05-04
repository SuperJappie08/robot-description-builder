//! The raw `Matarial` data handlers.
//!
//! TODO: EXPAND
/* DOC TODO:
 - Module DOC
*/
use std::sync::Arc;

use crate::ArcLock;

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

/// A enum containing all allowed `Material` types and their data.
#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA.
	///
	/// The fields need to be between 0 and 1 (for most simulators). (Not enforced)
	Color(f32, f32, f32, f32),
	/// Texture, containing the texture path as a valid package path (e.g. `"package://robot_description/textures/{texture}"`). You are on your own here.
	Texture(String),
}

#[cfg(feature = "urdf")]
impl ToURDF for MaterialData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		match self {
			MaterialData::Color(red, green, blue, alpha) => {
				writer
					.create_element("color")
					.with_attribute(Attribute {
						key: QName(b"rgba"),
						value: format!("{} {} {} {}", red, green, blue, alpha)
							.as_bytes()
							.into(),
					})
					.write_empty()?;
				Ok(())
			}
			MaterialData::Texture(texture_path) => {
				writer
					.create_element("texture")
					.with_attribute(Attribute {
						key: QName(b"filename"),
						value: texture_path.clone().as_bytes().into(),
					})
					.write_empty()?;
				Ok(())
			}
		}
	}
}

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
	///
	/// If one of the `MaterialData`s is the [`Global`](MaterialDataReferenceWrapper::Global) and it is poisoned,
	/// then we replace the data from the [`Direct`](MaterialDataReferenceWrapper::Direct) with the other one and return `true`.  
	pub fn same_material_data(&self, other: &MaterialDataReferenceWrapper) -> bool {
		#[allow(unreachable_code)] // This is for the Future Feature support
		match (self, other) {
			(
				MaterialDataReferenceWrapper::Direct(left),
				MaterialDataReferenceWrapper::Direct(right),
			) => left == right,
			(
				MaterialDataReferenceWrapper::Direct(left),
				MaterialDataReferenceWrapper::Global(right),
			) => match !right.is_poisoned() {
				true => (*left).clone() == right.read().unwrap().clone(), // We can safely unwrap, since we have checked for poisoning.
				false => {
					// When the right lock has been poisoned, recover by overwriting with the left [`MaterialData`]
					*right.write().map_err(|err| err.into_inner()).unwrap() = (*left).clone();
					todo!("Unpoisoning is still a nightly-only experimental feature. (mutex_unpoison #96469)");
					true
				}
			},
			(
				MaterialDataReferenceWrapper::Global(left),
				MaterialDataReferenceWrapper::Direct(right),
			) => {
				match !left.is_poisoned() {
					true => (*right).clone() == left.read().unwrap().clone(), // We can safely unwrap, since we have checked for poisoning.
					false => {
						// When the left lock has been poisoned, recover by overwriting with the right [`MaterialData`]
						*left.write().map_err(|err| err.into_inner()).unwrap() = (*right).clone();
						todo!("Unpoisoning is still a nightly-only experimental feature. (mutex_unpoison #96469)");
						true
					}
				}
			}
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
