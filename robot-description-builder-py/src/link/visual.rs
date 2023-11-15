use pyo3::{intern, prelude::*};

use robot_description_builder::{link_data, linkbuilding::VisualBuilder, prelude::GroupIDChanger};

use super::{collision::PyCollisionBuilder, geometry::PyGeometryBase};
use crate::{
	identifier::GroupIDError,
	material::{PyMaterial, PyMaterialDescriptor},
	transform::PyTransform,
};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "visual")?;

	module.add_class::<PyVisual>()?;
	module.add_class::<PyVisualBuilder>()?;

	// parent_module.add_submodule(module)?;
	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(
	name = "VisualBuilder",
	module = "robot_description_builder.link.visual"
)]
pub struct PyVisualBuilder(VisualBuilder);

#[pymethods]
impl PyVisualBuilder {
	// TODO: Figure out what a practical signature is
	#[new]
	fn new(
		geometry: &PyGeometryBase,
		name: Option<String>,
		transform: Option<PyTransform>,
		material: Option<PyMaterialDescriptor>,
	) -> PyVisualBuilder {
		Self(VisualBuilder::new_full(
			name,
			transform.map(Into::into),
			geometry.clone(),
			material.map(Into::into),
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
				self.0 = VisualBuilder::new_full(
					None,
					self.0.transform().copied(),
					self.0.geometry().boxed_clone(),
					self.0.material().cloned(),
				)
			}
			(None, false) => (),
		}
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.0.geometry().boxed_clone().into()
	}

	#[getter]
	fn get_transform(&self) -> Option<PyTransform> {
		self.0.transform().copied().map(Into::into)
	}

	#[setter]
	fn set_transform(&mut self, transform: Option<PyTransform>) {
		match (transform, self.0.transform().is_some()) {
			(Some(transform), _) => self.0 = self.0.clone().transformed(transform.into()),
			(None, true) => {
				self.0 = VisualBuilder::new_full(
					self.0.name().cloned(),
					None,
					self.0.geometry().boxed_clone(),
					self.0.material().cloned(),
				)
			}
			(None, false) => (),
		}
	}

	#[getter]
	fn get_material(&self) -> Option<PyMaterialDescriptor> {
		self.0.material().cloned().map(Into::into)
	}

	#[setter]
	fn set_material(&mut self, material_description: Option<PyMaterialDescriptor>) {
		match (material_description, self.0.material().is_some()) {
			(Some(material_description), _) => {
				self.0 = self.0.clone().materialized(material_description.into())
			}
			(None, true) => {
				self.0 = VisualBuilder::new_full(
					self.0.name().cloned(),
					self.0.transform().copied(),
					self.0.geometry().boxed_clone(),
					None,
				)
			}
			(None, false) => (),
		}
	}

	/// Creates a :class:`robot_description_builder.link.collision.CollisionBuilder` from this ``VisualBuilder``.
	///
	/// :return: A :class:`robot_description_builder.link.collision.CollisionBuilder` with the data from this ``VisualBuilder``
	/// :rtype: :class:`robot_description_builder.link.collision.CollisionBuilder`
	fn as_collision(&self) -> PyCollisionBuilder {
		self.0.to_collision().into()
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

		if let Some(transform) = self.get_transform() {
			data += ", transform=";
			data += transform.__repr__(py)?.as_str();
		}

		if let Some(material_descriptor) = self.get_material() {
			data += ", material=";
			data += material_descriptor.__repr__(py)?.as_str();
		}

		Ok(format!("{class_name}({data})"))
	}

	fn change_group_id(&mut self, new_group_id: String, _py: Python<'_>) -> PyResult<()> {
		self.0
			.change_group_id(new_group_id)
			.map_err(GroupIDError::from)
	}

	fn apply_group_id(&mut self, _py: Python<'_>) {
		self.0.apply_group_id()
	}
}

impl From<VisualBuilder> for PyVisualBuilder {
	fn from(value: VisualBuilder) -> Self {
		Self(value)
	}
}

impl From<PyVisualBuilder> for VisualBuilder {
	fn from(value: PyVisualBuilder) -> Self {
		value.0
	}
}

// Don't know if Clone is a good idea?
#[derive(Debug, Clone)]
#[pyclass(
	name = "Visual",
	module = "robot_description_builder.link.visual",
	frozen
)]
pub struct PyVisual {
	inner: link_data::Visual,
}

#[pymethods]
impl PyVisual {
	#[getter]
	fn get_name(&self) -> Option<String> {
		self.inner.name().cloned()
	}

	#[getter]
	fn get_geometry(&self) -> PyGeometryBase {
		self.inner.geometry().boxed_clone().into()
	}

	#[getter]
	fn get_transform(&self) -> Option<PyTransform> {
		self.inner.transform().copied().map(Into::into)
	}

	#[getter]
	fn get_material(&self) -> Option<PyMaterial> {
		self.inner.material().cloned().map(Into::into)
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

		if let Some(transform) = self.get_transform() {
			data += ", transform=";
			data += transform.__repr__(py)?.as_str();
		}

		if let Some(material) = self.get_material() {
			data += ", material=";
			data += material.__repr__(py)?.as_str();
		}

		Ok(format!("{class_name}({data})"))
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
