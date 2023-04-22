mod box_geometry;
mod cylinder_geometry;
mod sphere_geometry;

use pyo3::prelude::*;
use rdf_builder_rs::link_data::geometry::GeometryInterface;

pub use box_geometry::PyBoxGeometry;
pub use cylinder_geometry::PyCylinderGeometry;
pub use sphere_geometry::PySphereGeometry;

pub(crate) fn init_module(module: &PyModule) -> PyResult<()> {
	module.add_class::<PyGeometryBase>()?;
	module.add_class::<PyBoxGeometry>()?;
	module.add_class::<PySphereGeometry>()?;
	module.add_class::<PyCylinderGeometry>()?;

	Ok(())
}

#[pyclass(name = "GeometryBase", subclass)]
#[derive(Debug)]
pub struct PyGeometryBase {
	inner: Box<dyn GeometryInterface + Send + Sync>,
}

#[pymethods]
impl PyGeometryBase {
	fn volume(&self) -> f32 {
		self.inner.volume()
	}

	fn surface_area(&self) -> f32 {
		self.inner.surface_area()
	}
}

impl PyGeometryBase {
	fn new(geometry: &(dyn GeometryInterface + Sync + Send)) -> Self {
		Self {
			inner: geometry.into(),
		}
	}
}

impl From<Box<dyn GeometryInterface + Sync + Send>> for PyGeometryBase {
	fn from(value: Box<dyn GeometryInterface + Sync + Send>) -> Self {
		Self { inner: value }
	}
}

impl From<&(dyn GeometryInterface + Sync + Send)> for PyGeometryBase {
	fn from(value: &(dyn GeometryInterface + Sync + Send)) -> Self {
		Self {
			inner: value.into(),
		}
	}
}

impl Clone for PyGeometryBase {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.boxed_clone(),
		}
	}
}
