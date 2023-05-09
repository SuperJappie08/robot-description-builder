mod box_geometry;
mod cylinder_geometry;
mod sphere_geometry;

use pyo3::prelude::*;
use robot_description_builder::link_data::geometry::GeometryInterface;

pub use box_geometry::PyBoxGeometry;
pub use cylinder_geometry::PyCylinderGeometry;
pub use sphere_geometry::PySphereGeometry;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "geometry")?;

	module.add_class::<PyGeometryBase>()?;
	module.add_class::<PyBoxGeometry>()?;
	module.add_class::<PySphereGeometry>()?;
	module.add_class::<PyCylinderGeometry>()?;

	// parent_module.add_submodule(module)?;

	Ok(())
}

#[pyclass(
	name = "GeometryBase",
	module = "robot_description_builder.link.geometry",
	subclass
)]
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

	fn bounding_box(&self) -> (f32, f32, f32) {
		self.inner.bounding_box()
	}

	pub fn __repr__(&self) -> String {
		todo!()
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

impl From<PyGeometryBase> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: PyGeometryBase) -> Self {
		value.inner
	}
}

impl Clone for PyGeometryBase {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.boxed_clone(),
		}
	}
}
