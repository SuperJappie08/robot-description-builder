use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use rdf_builder_rs::Material;

#[derive(Debug)]
#[pyclass(name = "Material")]
pub struct PyMaterial {
	inner: Arc<RwLock<Material>>,
}

#[pymethods]
impl PyMaterial {
	#[new]
	/// TODO: FINIS
	fn new(name: Option<String>) -> Self {
		Arc::new(RwLock::new(Material::new_color(name, 1., 1., 1., 1.))).into()
	}

	pub fn __repr__(&self) -> String {
		let mut repr = String::from("Material(");

		{
			let material = self.inner.read().unwrap(); //FIXME: UNWRAP OK?

			if let Some(name) = material.get_name() {
				repr += &format!("name = '{}', ", name);
			}

			repr += &match material.get_material_data() {
				rdf_builder_rs::MaterialData::Color(r, g, b, a) => {
					format!("rgba=({}, {}, {}, {})", r, g, b, a)
				}
				rdf_builder_rs::MaterialData::Texture(path) => format!("texture_path={}", path),
			};
		}

		repr.push(')');
		repr
	}
}

impl From<Arc<RwLock<Material>>> for PyMaterial {
	fn from(value: Arc<RwLock<Material>>) -> Self {
		Self { inner: value }
	}
}

impl From<PyMaterial> for Arc<RwLock<Material>> {
	fn from(value: PyMaterial) -> Self {
		value.inner
	}
}
