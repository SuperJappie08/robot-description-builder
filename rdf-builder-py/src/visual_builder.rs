use pyo3::prelude::*;

use rdf_builder_rs::linkbuilding::VisualBuilder;

#[derive(Debug)]
#[pyclass(name = "VisualBuilder")]
pub struct PyVisualBuilder {
	inner: VisualBuilder,
}

#[pymethods]
impl PyVisualBuilder {
	#[new]
	fn new() -> PyResult<PyVisualBuilder> {
		todo!("No way to represent geometries yet")
	}
}
