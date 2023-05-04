use pyo3::prelude::*;

use rdf_builder_rs::{link_data, linkbuilding::VisualBuilder};

use super::geometry::PyGeometryBase;
use crate::{material_builder::PyMaterialBuilder, transform::PyTransform};

pub(super) fn init_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
	let module = PyModule::new(py, "visual")?;

	module.add_class::<PyVisual>()?;
	module.add_class::<PyVisualBuilder>()?;

	parent_module.add_submodule(module)?;
	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "VisualBuilder")]
pub struct PyVisualBuilder(VisualBuilder);

#[pymethods]
impl PyVisualBuilder {
	/// TODO: Figure out what a practical signature is
	#[new]
	fn new(
		geometry: &PyGeometryBase,
		name: Option<String>,
		origin: Option<PyTransform>,
		material: Option<PyMaterialBuilder>,
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
	fn get_material(&self) -> Option<PyMaterialBuilder> {
		self.0.material().cloned().map(Into::into)
	}

	#[setter]
	fn set_material(&mut self, material_description: PyMaterialBuilder) {
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
#[pyclass(name = "Visual")]
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
			todo!()
			// repr += &format!(
			// 	", material = {}",
			// 	Into::<PyMaterial>::into(Arc::clone(material)).__repr__()
			// );
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
