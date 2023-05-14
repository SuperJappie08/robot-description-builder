use pyo3::{basic::CompareOp, intern, prelude::*};

use robot_description_builder::link_data::geometry::{CylinderGeometry, GeometryInterface};

use super::PyGeometryBase;

#[derive(Debug, Clone)]
#[pyclass(name = "CylinderGeometry", extends = PyGeometryBase, module="robot_description_builder.link.geometry")]
pub struct PyCylinderGeometry {
	inner: CylinderGeometry,
}

impl PyCylinderGeometry {
	fn new(radius: f32, length: f32) -> (PyCylinderGeometry, PyGeometryBase) {
		let geometry = CylinderGeometry::new(radius, length);
		let base = PyGeometryBase::new(&geometry);
		(PyCylinderGeometry { inner: geometry }, base)
	}
}

#[pymethods]
impl PyCylinderGeometry {
	#[new]
	#[pyo3(signature = (radius, length))]
	fn py_new(radius: f32, length: f32) -> (PyCylinderGeometry, PyGeometryBase) {
		Self::new(radius, length)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		Ok(format!(
			"{}({}, {})",
			class_name, self.inner.radius, self.inner.length
		))
	}

	// Might be excessive due to also being implemented on super class
	fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
		match op {
			CompareOp::Eq => (self.inner == other.inner).into_py(py),
			CompareOp::Ne => (self.inner != other.inner).into_py(py),
			// TODO: Consider implementing Gt and Lt based on volume?
			_ => py.NotImplemented(),
		}
	}

	/// TODO: Maybe change to dict? or remove
	#[getter]
	fn get_size(&self) -> (f32, f32) {
		(self.inner.radius, self.inner.length)
	}

	#[getter]
	fn get_radius(&self) -> f32 {
		self.inner.radius
	}

	#[setter]
	fn set_radius(mut slf: PyRefMut<'_, Self>, radius: f32) {
		slf.inner.radius = radius;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}

	#[getter]
	fn get_length(&self) -> f32 {
		self.inner.length
	}

	#[setter]
	fn set_length(mut slf: PyRefMut<'_, Self>, length: f32) {
		slf.inner.length = length;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}

	// #[setter]
	// fn set_size(mut self_: PyRefMut<'_, Self>, size: (f32, f32, f32)) {
	//     self_.inner.side1 = size.0;
	//     self_.inner.side2 = size.1;
	//     self_.inner.side3 = size.2;

	//     let data = self_.inner.boxed_clone();

	//     let mut super_class = self_.into_super();
	//     super_class.inner = data;
	// }
}

impl From<CylinderGeometry> for PyCylinderGeometry {
	fn from(value: CylinderGeometry) -> Self {
		Self { inner: value }
	}
}
