use std::sync::Arc;

use crate::{ArcLock, MaterialData};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

use super::material_data_reference::MaterialDataReferenceWrapper;

/// FIXME: TEMP PUB
#[derive(Debug)]
pub enum MaterialStage {
	PreInit(MaterialData),
	Initialized(ArcLock<MaterialData>),
}

impl MaterialStage {
	/// Gets the Strong count of the `MaterialData`,
	/// returns 0 if the `LocalMaterial` is not fully initialized yet.
	pub fn get_used_count(&self) -> usize {
		match self {
			MaterialStage::PreInit(_) => 0,
			MaterialStage::Initialized(arc_data) => Arc::strong_count(arc_data),
		}
	}

	pub(crate) fn initialize(&mut self, material_data: ArcLock<MaterialData>) {
		match self {
			MaterialStage::PreInit(_) => *self = MaterialStage::Initialized(material_data),
			MaterialStage::Initialized(data) => {
				debug_assert!(Arc::ptr_eq(data, &material_data));
			}
		}
	}

	pub(crate) fn get_data(&self) -> MaterialDataReferenceWrapper {
		match self {
			MaterialStage::PreInit(data) => data.into(),
			MaterialStage::Initialized(arc_data) => Arc::clone(arc_data).into(), //Unwrap not Ok
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for MaterialStage {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		match self {
			MaterialStage::PreInit(data) => data.to_urdf(writer, urdf_config),
			MaterialStage::Initialized(arc_data) => {
				arc_data.read().unwrap().to_urdf(writer, urdf_config) // FIXME: UNWRAP NOT OK
			}
		}
	}
}

impl PartialEq for MaterialStage {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::PreInit(l0), Self::PreInit(r0)) => l0 == r0,
			(Self::Initialized(l0), Self::Initialized(r0)) => Arc::ptr_eq(l0, r0),
			_ => false,
		}
	}
}

impl Clone for MaterialStage {
	fn clone(&self) -> Self {
		match self {
			Self::PreInit(arg0) => Self::PreInit(arg0.clone()),
			Self::Initialized(arg0) => Self::Initialized(Arc::clone(arg0)),
		}
	}
}

impl From<MaterialData> for MaterialStage {
	fn from(value: MaterialData) -> Self {
		Self::PreInit(value)
	}
}