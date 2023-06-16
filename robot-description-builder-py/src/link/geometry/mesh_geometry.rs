use pyo3::{basic::CompareOp, intern, prelude::*};

use robot_description_builder::link_data::geometry::{GeometryInterface, MeshGeometry};

use super::PyGeometryBase;

#[derive(Debug, Clone)]
#[pyclass(name = "MeshGeometry", extends = PyGeometryBase, module="robot_description_builder.link.geometry")]
pub struct PyMeshGeometry {
	inner: MeshGeometry,
}

impl PyMeshGeometry {
	fn new(
		path: String,
		bounding_box: (f32, f32, f32),
		scale: Option<(f32, f32, f32)>,
	) -> (PyMeshGeometry, PyGeometryBase) {
		let geometry = MeshGeometry::new(path, bounding_box, scale);
		let base = PyGeometryBase::new(&geometry);
		(PyMeshGeometry { inner: geometry }, base)
	}
}

#[pymethods]
impl PyMeshGeometry {
	/// TODO: Names of arguments might be incorrect/Require explanation
	#[new]
	#[pyo3(signature = (path, bounding_box, scale=None))]
	fn py_new(
		path: String,
		bounding_box: (f32, f32, f32),
		scale: Option<(f32, f32, f32)>,
	) -> (PyMeshGeometry, PyGeometryBase) {
		Self::new(path, bounding_box, scale)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		Ok(format!(
			"{}(path='{}', bounding_box={:?}, scale={:?})",
			class_name, self.inner.path, self.inner.bounding_box, self.inner.scale
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
	fn get_path(&self) -> String {
		self.inner.path.clone()
	}

	#[setter]
	fn set_path(mut slf: PyRefMut<'_, Self>, path: String) {
		slf.inner.path = path;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}

	#[getter]
	fn get_bounding_box(&self) -> (f32, f32, f32) {
		self.inner.bounding_box
	}

	#[setter]
	fn set_bounding_box(mut slf: PyRefMut<'_, Self>, bounding_box: (f32, f32, f32)) {
		slf.inner.bounding_box.0 = bounding_box.0;
		slf.inner.bounding_box.1 = bounding_box.1;
		slf.inner.bounding_box.2 = bounding_box.2;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}

	#[getter]
	fn get_scale(&self) -> (f32, f32, f32) {
		self.inner.scale
	}

	#[setter]
	fn set_scale(mut slf: PyRefMut<'_, Self>, scale: (f32, f32, f32)) {
		slf.inner.scale.0 = scale.0;
		slf.inner.scale.1 = scale.1;
		slf.inner.scale.2 = scale.2;

		let data = slf.inner.boxed_clone();

		let mut super_class = slf.into_super();
		super_class.inner = data;
	}
}

impl From<MeshGeometry> for PyMeshGeometry {
	fn from(value: MeshGeometry) -> Self {
		Self { inner: value }
	}
}
