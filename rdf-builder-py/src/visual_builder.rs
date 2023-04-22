use pyo3::prelude::*;

use rdf_builder_rs::linkbuilding::VisualBuilder;

use crate::{geometry::PyGeometryBase, material_builder::PyMaterialBuilder};

#[derive(Debug)]
#[pyclass(name = "VisualBuilder")]
pub struct PyVisualBuilder {
	inner: VisualBuilder,
}

#[pymethods]
impl PyVisualBuilder {
	/// TODO: Figure out what a practical signature is
	#[new]
	fn new(
		geometry: &PyGeometryBase,
		material: Option<&PyMaterialBuilder>,
	) -> PyResult<PyVisualBuilder> {
		todo!("No way to represent geometries yet")
	}
}
