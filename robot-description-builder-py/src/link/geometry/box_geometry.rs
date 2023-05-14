use pyo3::{basic::CompareOp, intern, prelude::*};

use robot_description_builder::link_data::geometry::{BoxGeometry, GeometryInterface};

use super::PyGeometryBase;

#[derive(Debug, Clone)]
#[pyclass(name = "BoxGeometry", extends = PyGeometryBase, module="robot_description_builder.link.geometry")]
pub struct PyBoxGeometry {
	inner: BoxGeometry,
}

impl PyBoxGeometry {
	fn new(side0: f32, side1: f32, side2: f32) -> (PyBoxGeometry, PyGeometryBase) {
		let geometry = BoxGeometry::new(side0, side1, side2);
		let base = PyGeometryBase::new(&geometry);
		(PyBoxGeometry { inner: geometry }, base)
	}
}

#[pymethods]
impl PyBoxGeometry {
	/// TODO: Names of arguments might be incorrect/Require explanation
	#[new]
	#[pyo3(signature = (width, length, height))]
	fn py_new(width: f32, length: f32, height: f32) -> (PyBoxGeometry, PyGeometryBase) {
		Self::new(width, length, height)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		Ok(format!(
			"{}({}, {}, {})",
			class_name, self.inner.side1, self.inner.side2, self.inner.side3
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

	#[getter]
	fn get_size(&self) -> (f32, f32, f32) {
		(self.inner.side1, self.inner.side2, self.inner.side3)
	}

	#[setter]
	fn set_size(mut slf: PyRefMut<'_, Self>, size: (f32, f32, f32)) {
		slf.inner.side1 = size.0;
		slf.inner.side2 = size.1;
		slf.inner.side3 = size.2;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}
}

impl From<BoxGeometry> for PyBoxGeometry {
	fn from(value: BoxGeometry) -> Self {
		Self { inner: value }
	}
}
