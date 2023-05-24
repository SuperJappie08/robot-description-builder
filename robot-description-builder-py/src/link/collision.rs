use pyo3::{intern, prelude::*};
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
	fn set_name(&mut self, name: Option<String>) {
		match (name, self.0.name().is_some()) {
			(Some(name), _) => self.0 = self.0.clone().named(name),
			(None, true) => {
				self.0 = CollisionBuilder::new_full(
					None,
					self.0.origin().copied(),
					self.0.geometry().boxed_clone(),
				);
			}
			(None, false) => (),
		}
	}

	#[getter]
	fn get_origin(&self) -> Option<PyTransform> {
		self.0.origin().copied().map(Into::into)
	}

	#[setter]
	fn set_origin(&mut self, origin: Option<PyTransform>) {
		match (origin, self.0.origin().is_some()) {
			(Some(origin), _) => self.0 = self.0.clone().tranformed(origin.into()),
			(None, true) => {
				self.0 = CollisionBuilder::new_full(
					self.0.name().cloned(),
					None,
					self.0.geometry().boxed_clone(),
				)
			}
			(None, false) => (),
		}
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.0.geometry().boxed_clone().into()
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = match self.0.name() {
			Some(name) => format!("name='{name}', "),
			None => String::new(),
		};

		data += "geometry=";
		data += self.get_geometry().__repr__(py)?.as_str();

		if let Some(transform) = self.get_origin() {
			data += ", origin=";
			data += transform.__repr__(py)?.as_str();
		}

		Ok(format!("{class_name}({data})"))
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

#[pymethods]
impl PyCollision {
	#[getter]
	fn get_name(&self) -> Option<String> {
		self.inner.name().cloned()
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.inner.geometry().boxed_clone().into()
	}

	#[getter]
	fn get_origin(&self) -> Option<PyTransform> {
		self.inner.origin().copied().map(Into::into)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = match self.inner.name() {
			Some(name) => format!("name='{name}', "),
			None => String::new(),
		};

		data += "geometry=";
		data += self.get_geometry().__repr__(py)?.as_str();

		if let Some(transform) = self.get_origin() {
			data += ", origin=";
			data += transform.__repr__(py)?.as_str();
		}

		Ok(format!("{class_name}({data})"))
	}
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
