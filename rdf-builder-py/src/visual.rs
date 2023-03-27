use std::sync::Arc;

use pyo3::prelude::*;

use rdf_builder_rs::link_data::{self, geometry::BoxGeometry};

use crate::material::PyMaterial;

/// Don't know if CLone is a good idea?
#[derive(Debug, Clone)]
#[pyclass(name = "Visual")]
pub struct PyVisual {
	inner: link_data::Visual,
}

#[pymethods]
impl PyVisual {
	#[new]
	fn new(name: Option<String>, _origin: Option<(f32, f32, f32)>) -> Self {
		link_data::Visual::new(name, None, BoxGeometry::new(4., 5., 6.), None).into()
	}

	pub fn __repr__(&self) -> String {
		let mut repr = format!("Visual(name = ");

		if let Some(name) = self.inner.get_name() {
			repr += format!("'{}'", name).as_str().clone();
		} else {
			repr += "None"
		}

		// TODO: THIS ISN'T OK
		repr += &format!(", {:?}", self.inner.get_geometry());

		if let Some(material) = self.inner.get_material() {
			repr += &format!(
				", material = {}",
				Into::<PyMaterial>::into(Arc::clone(material)).__repr__()
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
