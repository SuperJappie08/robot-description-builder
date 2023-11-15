use std::io::{Read, Seek};

use itertools::Itertools;
use pyo3::{
	exceptions::{PyBufferError, PyEOFError, PyKeyError, PyTypeError},
	intern,
	prelude::*,
	types::PyDict,
};
use robot_description_builder::to_rdf::to_urdf::{
	to_urdf, URDFConfig, URDFMaterialReferences, URDFTarget,
};

use crate::{cluster_objects::PyRobot, exceptions::XMLError};

use super::dict2xmlmode;

/// TODO: description type should be more excepting
/// TODO: Mayba add a URDF object like rosbag lib
#[pyfunction(signature=(description, **kwargs))]
pub(super) fn to_urdf_string(
	description: &PyRobot,
	kwargs: Option<&PyDict>,
	py: Python<'_>,
) -> PyResult<String> {
	let urdf_config = match kwargs {
		Some(kwargs) => dict2urdfconfig(py, kwargs)?,
		None => URDFConfig::default(),
	};

	let mut buffer = to_urdf(description.as_robot(), urdf_config)
		.map_err(XMLError::from)?
		.into_inner();

	let mut out = String::new();
	buffer.rewind().map_err(|err| {
		PyBufferError::new_err(format!(
			"Could not rewind the buffer. (RUST-IO-ERROR: {})",
			err
		))
	})?;
	buffer.read_to_string(&mut out).map_err(|err| {
		PyEOFError::new_err(format!(
			"The date is invalid. (RUST-IO-READ-ERROR: {})",
			err
		))
	})?;

	Ok(out)
}

fn dict2urdfconfig(py: Python<'_>, kwds: &PyDict) -> PyResult<URDFConfig> {
	let config = URDFConfig {
		xml_mode: dict2xmlmode(py, kwds)?,
		urdf_target: dict2urdftarget(py, kwds)?,
		material_references: dict2materialref(py, kwds)?,
		..Default::default()
	};

	if !kwds.is_empty() {
		// FIXME: This should not be static
		let qualname = py
			.import(intern!(py, "robot_description_builder"))?
			.getattr(intern!(py, "to_urdf_string"))?
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;
		return Err(PyTypeError::new_err(format!(
			"{} got an unexpected keyword argument '{}'",
			qualname,
			kwds.keys()
				.into_iter()
				.map(|key| key.str())
				.process_results(|mut iter| { iter.join(", ") })?
		)));
	}

	Ok(config)
}

/// Takes `'target'` argument from a `&PyDict` to extract a `URDFTarget`
fn dict2urdftarget(py: Python<'_>, kwds: &PyDict) -> PyResult<URDFTarget> {
	if let Some(target) = kwds.get_item(intern!(py, "target"))? {
		// target specified so extract the target string
		let target = target.extract::<&str>()?;
		let target_lower = target.to_lowercase();

		kwds.del_item(intern!(py, "target"))?;
		return match target_lower.as_str() {
			// If it was None or 'standard'
			"standard" | "none" => Ok(URDFTarget::Standard),
			"gazebo" => Ok(URDFTarget::Gazebo),
			_ => Err(PyKeyError::new_err(format!(
				"Invalid URDF target '{}' specified. Valid targets: [...]",
				target
			))),
		};
	}

	// Not specified so use the default
	Ok(URDFTarget::default())
}

fn dict2materialref(py: Python<'_>, kwds: &PyDict) -> PyResult<URDFMaterialReferences> {
	if let Some(material_mode) = kwds.get_item(intern!(py, "material_refmode"))? {
		return Err(pyo3::exceptions::PyNotImplementedError::new_err(format!(
			"TODO: material_ref_mode: {}",
			material_mode
		)));
	}

	// Not specified so use the default
	Ok(URDFMaterialReferences::default())
}
