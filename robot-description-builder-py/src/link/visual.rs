use std::sync::Arc;

use pyo3::prelude::*;

use robot_description_builder::{link_data, linkbuilding::VisualBuilder};

use super::geometry::PyGeometryBase;
use crate::{material_descriptor::PyMaterialDescriptor, transform::PyTransform};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "visual")?;

	module.add_class::<PyVisual>()?;
	module.add_class::<PyVisualBuilder>()?;

	// parent_module.add_submodule(module)?;
	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(
	name = "VisualBuilder",
	module = "robot_description_builder.link.visual"
)]
pub struct PyVisualBuilder(VisualBuilder);

#[pymethods]
impl PyVisualBuilder {
	/// TODO: Figure out what a practical signature is
	#[new]
	fn new(
		geometry: &PyGeometryBase,
		name: Option<String>,
		origin: Option<PyTransform>,
		material: Option<PyMaterialDescriptor>,
	) -> PyVisualBuilder {
		Self(VisualBuilder::new_full(
			name,
			origin.map(Into::into),
			geometry.clone(),
			material.map(Into::into),
		))
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().map(Clone::clone)
	}

	#[setter]
	fn set_name(&mut self, name: String) {
		self.0 = self.0.clone().named(name);
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.0.geometry().boxed_clone().into()
	}

	#[getter]
	fn get_origin(&self) -> Option<PyTransform> {
		self.0.origin().copied().map(Into::into)
	}

	#[setter]
	fn set_origin(&mut self, transform: PyTransform) {
		self.0 = self.0.clone().tranformed(transform.into());
	}

	#[getter]
	fn get_material(&self) -> Option<PyMaterialDescriptor> {
		self.0.material().cloned().map(Into::into)
	}

	#[setter]
	fn set_material(&mut self, material_description: PyMaterialDescriptor) {
		self.0 = self.0.clone().materialized(material_description.into())
	}
}

impl From<VisualBuilder> for PyVisualBuilder {
	fn from(value: VisualBuilder) -> Self {
		Self(value)
	}
}

impl From<PyVisualBuilder> for VisualBuilder {
	fn from(value: PyVisualBuilder) -> Self {
		value.0
	}
}

/// Don't know if CLone is a good idea?
#[derive(Debug, Clone)]
#[pyclass(name = "Visual", module = "robot_description_builder.link.visual")]
pub struct PyVisual {
	inner: link_data::Visual,
}

#[pymethods]
impl PyVisual {
	pub fn __repr__(&self) -> String {
		let mut repr = String::from("Visual(name = ");

		if let Some(name) = self.inner.name() {
			repr += format!("'{}'", name).as_str();
		} else {
			repr += "None"
		}

		// TODO: THIS ISN'T OK
		repr += &format!(
			", {:?}",
			Into::<PyGeometryBase>::into(self.inner.geometry().boxed_clone()).__repr__()
		);

		if let Some(material) = self.inner.material() {
			// todo!()
			repr += &format!(
				", material = {}",
				// TODO: Figure out if this should be `PyMaterial` or `PyMaterialDescriptor`
				Into::<PyMaterialDescriptor>::into(material.rebuild()).__repr__()
			);
		}

		repr += ")";
		repr
	}
}

impl From<link_data::Visual> for PyVisual {
	fn from(value: link_data::Visual) -> Self {
		Self { inner: value }
	}
}

impl From<PyVisual> for link_data::Visual {
	fn from(value: PyVisual) -> Self {
		value.inner
	}
}
