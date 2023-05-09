use pyo3::prelude::*;
use robot_description_builder::{link_data::Collision, linkbuilding::CollisionBuilder, Transform};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "collision")?;

	module.add_class::<PyCollision>()?;
	module.add_class::<PyCollisionBuilder>()?;

	// parent_module.add_submodule(module)?;

	Ok(())
}

use super::geometry::PyGeometryBase;
use crate::transform::PyTransform;

/// TODO: Considering skipping the wrapping here and doing it manually
#[derive(Debug, PartialEq, Clone)]
#[pyclass(
	name = "CollisionBuilder",
	module = "robot_description_builder.link.collision"
)]
pub struct PyCollisionBuilder(CollisionBuilder);

#[pymethods]
impl PyCollisionBuilder {
	#[new]
	fn py_new(
		geometry: &PyGeometryBase,
		name: Option<String>,
		origin: Option<PyTransform>,
	) -> Self {
		Self(CollisionBuilder::new_full(
			name,
			origin.map(Into::<Transform>::into),
			geometry.clone(),
		))
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().cloned()
	}

	#[setter]
	fn set_name(&mut self, name: String) {
		self.0 = self.0.clone().named(name)
	}

	#[getter]
	fn get_origin(&self) -> Option<PyTransform> {
		self.0.origin().copied().map(Into::into)
	}

	#[setter]
	fn set_origin(&mut self, origin: PyTransform) {
		self.0 = self.0.clone().tranformed(origin.into())
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.0.geometry().boxed_clone().into()
	}
}

impl From<PyCollisionBuilder> for CollisionBuilder {
	fn from(value: PyCollisionBuilder) -> Self {
		value.0
	}
}

impl From<CollisionBuilder> for PyCollisionBuilder {
	fn from(value: CollisionBuilder) -> Self {
		Self(value)
	}
}

#[derive(Debug, PartialEq, Clone)]
#[pyclass(
	name = "Collision",
	module = "robot_description_builder.link.collision"
)]
pub struct PyCollision {
	inner: Collision,
}

impl From<Collision> for PyCollision {
	fn from(value: Collision) -> Self {
		Self { inner: value }
	}
}

impl From<PyCollision> for Collision {
	fn from(value: PyCollision) -> Self {
		value.inner
	}
}
