use pyo3::{intern, prelude::*};

use robot_description_builder::link_data::geometry::{BoxGeometry, GeometryInterface};

use super::PyGeometryBase;

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

	fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
		// let module_name = slf
		// 	.get_type()
		// 	.getattr(intern!(slf.py(), "__module__"))?
		// 	.extract::<&str>()?;
		let class_name = slf
			.get_type()
			.getattr(intern!(slf.py(), "__qualname__"))?
			.extract::<&str>()?;

		let box_ref = slf.try_borrow()?;

		Ok(format!(
			// "{}.{}({}, {}, {})",
			"{}({}, {}, {})",
			// module_name,
			class_name,
			box_ref.inner.side1,
			box_ref.inner.side2,
			box_ref.inner.side3
		))
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
