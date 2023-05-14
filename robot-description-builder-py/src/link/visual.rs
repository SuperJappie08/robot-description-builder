use pyo3::{intern, prelude::*};

use robot_description_builder::{link_data, linkbuilding::VisualBuilder};

use super::geometry::PyGeometryBase;
use crate::{material_descriptor::PyMaterialDescriptor, transform::PyTransform};

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
	/// TODO: Figure out what a practical signature is
	#[new]
	fn new(
		geometry: &PyGeometryBase,
		name: Option<String>,
		origin: Option<PyTransform>,
		material: Option<PyMaterialDescriptor>,
	) -> PyVisualBuilder {
		Self(VisualBuilder::new_full(
			name,
			origin.map(Into::into),
			geometry.clone(),
			material.map(Into::into),
		))
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().map(Clone::clone)
	}

	#[setter]
	fn set_name(&mut self, name: Option<String>) {
		match (name, self.0.name().is_some()) {
			(Some(name), _) => self.0 = self.0.clone().named(name),
			(None, true) => {
				self.0 = VisualBuilder::new_full(
					None,
					self.0.origin().copied(),
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
	fn get_origin(&self) -> Option<PyTransform> {
		self.0.origin().copied().map(Into::into)
	}

	#[setter]
	fn set_origin(&mut self, transform: Option<PyTransform>) {
		match (transform, self.0.origin().is_some()) {
			(Some(transform), _) => self.0 = self.0.clone().tranformed(transform.into()),
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
					self.0.origin().copied(),
					self.0.geometry().boxed_clone(),
					None,
				)
			}
			(None, false) => (),
		}
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

/// Don't know if CLone is a good idea?
#[derive(Debug, Clone)]
#[pyclass(name = "Visual", module = "robot_description_builder.link.visual")]
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
	fn get_origin(&self) -> Option<PyTransform> {
		self.inner.origin().copied().map(Into::into)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let mut repr = format!(
			"{}(name = ",
			py.get_type::<Self>()
				.getattr(intern!(py, "__qualname__"))?
				.extract::<&str>()?
		);

		if let Some(name) = self.inner.name() {
			repr += format!("'{}'", name).as_str();
		} else {
			repr += "None"
		}

		// TODO: THIS ISN'T OK
		repr += &format!(
			", {}",
			Into::<PyGeometryBase>::into(self.inner.geometry().boxed_clone()).__repr__(py)?
		);

		if let Some(material) = self.inner.material() {
			// todo!()
			repr += &format!(
				", material = {}",
				// TODO: Figure out if this should be `PyMaterial` or `PyMaterialDescriptor`
				Into::<PyMaterialDescriptor>::into(material.rebuild()).__repr__(py)?
			);
		}

		repr += ")";
		Ok(repr)
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
