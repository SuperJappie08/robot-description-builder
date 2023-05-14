use pyo3::{basic::CompareOp, intern, prelude::*};

use robot_description_builder::link_data::geometry::{GeometryInterface, SphereGeometry};

use super::PyGeometryBase;

#[derive(Debug, Clone)]
#[pyclass(name="SphereGeometry", extends=PyGeometryBase, module="robot_description_builder.link.geometry")]
pub struct PySphereGeometry {
	inner: SphereGeometry,
}

impl PySphereGeometry {
	fn new(radius: f32) -> (PySphereGeometry, PyGeometryBase) {
		let geometry = SphereGeometry::new(radius);
		let base = PyGeometryBase::new(&geometry);
		(PySphereGeometry { inner: geometry }, base)
	}
}

#[pymethods]
impl PySphereGeometry {
	#[new]
	#[pyo3(signature = (radius))]
	fn py_new(radius: f32) -> (PySphereGeometry, PyGeometryBase) {
		// TODO: Maybe add kwargs and args checking
		// I do not think it is necessary yet.
		Self::new(radius)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		Ok(format!("{}({})", class_name, self.inner.radius))
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
}

impl From<SphereGeometry> for PySphereGeometry {
	fn from(value: SphereGeometry) -> Self {
		Self { inner: value }
	}
}
