use pyo3::prelude::*;
use rdf_builder_rs::MaterialBuilder;

#[derive(Debug)]
#[pyclass(name = "MaterialBuilder")]
pub struct PyMaterialBuilder {
	inner: MaterialBuilder,
}

#[pymethods]
impl PyMaterialBuilder {
	// #[new]
	// fn new() -> PyMaterialBuilder {
	// TODO: Take String and do path
	// TODOL Take Color and do Colours.
	// }
}
