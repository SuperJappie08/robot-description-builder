mod box_geometry;
mod cylinder_geometry;
mod mesh_geometry;
mod sphere_geometry;

use pyo3::{basic::CompareOp, exceptions::PyNotImplementedError, prelude::*};
use robot_description_builder::link_data::geometry::{GeometryInterface, GeometryShapeContainer};

pub use box_geometry::PyBoxGeometry;
pub use cylinder_geometry::PyCylinderGeometry;
pub use mesh_geometry::PyMeshGeometry;
pub use sphere_geometry::PySphereGeometry;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "geometry")?;

	module.add_class::<PyGeometryBase>()?;
	module.add_class::<PyBoxGeometry>()?;
	module.add_class::<PySphereGeometry>()?;
	module.add_class::<PyMeshGeometry>()?;
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

	fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
		match op {
			// TODO: Consider using GeometryContainer as a medium to do this
			CompareOp::Eq => (*self.inner == *other.inner).into_py(py),
			CompareOp::Ne => (*self.inner != *other.inner).into_py(py),
			_ => py.NotImplemented(),
		}
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		match self.inner.shape_container() {
			GeometryShapeContainer::Box(geometry) => {
				Into::<PyBoxGeometry>::into(geometry).__repr__(py)
			}
			GeometryShapeContainer::Cylinder(geometry) => {
				Into::<PyCylinderGeometry>::into(geometry).__repr__(py)
			}
			GeometryShapeContainer::Sphere(geometry) => {
				Into::<PySphereGeometry>::into(geometry).__repr__(py)
			}
			GeometryShapeContainer::Mesh(geometry) => {
				Into::<PyMeshGeometry>::into(geometry).__repr__(py)
			}
			other => Err(PyNotImplementedError::new_err(format!(
				"__repr__ for {other:?} via GeometryBase is not implemented yet."
			))),
		}
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
