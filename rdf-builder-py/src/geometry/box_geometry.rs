use pyo3::{
	prelude::*,
	types::{PyDict, PyTuple}, intern,
};

use rdf_builder_rs::link_data::geometry::{BoxGeometry, GeometryInterface};

use crate::geometry::PyGeometryBase;

#[pyclass(name = "BoxGeometry", extends = PyGeometryBase, module = "geometry")]
pub struct PyBoxGeometry {
	inner: BoxGeometry,
}

#[pymethods]
impl PyBoxGeometry {
	#[new]
	#[pyo3(signature = (*py_args, **py_kwargs))]
	fn new(py_args: &PyTuple, py_kwargs: Option<&PyDict>) -> (PyBoxGeometry, PyGeometryBase) {
		if py_args.is_empty() || py_args.is_none() {
			todo!()
		} else {
			if let Ok(box_dim) = py_args.extract::<(f32, f32, f32)>() {
				let geometry = BoxGeometry::new(box_dim.0, box_dim.1, box_dim.2);
				let base = PyGeometryBase::new(&geometry);
				(PyBoxGeometry { inner: geometry }, base)
			} else {
				todo!()
			}
		}
	}

	fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let module_name = slf.get_type().getattr(intern!(slf.py(), "__module__"))?.extract::<&str>()?;
		let class_name = slf.get_type().getattr(intern!(slf.py(),"__qualname__"))?.extract::<&str>()?;

		let box_ref = slf.try_borrow()?;

		Ok(format!(
			"{}.{}({}, {}, {})",
			module_name, class_name, box_ref.inner.side1, box_ref.inner.side2, box_ref.inner.side3
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
