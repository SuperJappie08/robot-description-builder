use pyo3::{
	prelude::*,
	types::{PyDict, PyTuple}, intern,
};

use rdf_builder_rs::link_data::geometry::{CylinderGeometry, GeometryInterface};

use crate::geometry::PyGeometryBase;

#[derive(Debug)]
#[pyclass(name="CylinderGeometry", extends=PyGeometryBase)]
pub struct PyCylinderGeometry {
	inner: CylinderGeometry,
}

#[pymethods]
impl PyCylinderGeometry {
	#[new]
	#[pyo3(signature = (*py_args, **py_kwargs))]
	fn new(py_args: &PyTuple, py_kwargs: Option<&PyDict>) -> (PyCylinderGeometry, PyGeometryBase) {
		if py_args.is_empty() || py_args.is_none() {
			todo!()
		} else {
			if let Ok(cylinder_dim) = py_args.extract::<(f32, f32)>() {
				let geometry = CylinderGeometry::new(cylinder_dim.0, cylinder_dim.1);
				let base = PyGeometryBase::new(&geometry);
				(PyCylinderGeometry { inner: geometry }, base)
			} else {
				todo!()
			}
		}
	}

	fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let module_name = slf.get_type().getattr(intern!(slf.py(), "__module__"))?.extract::<&str>()?;
		let class_name = slf.get_type().getattr(intern!(slf.py(),"__qualname__"))?.extract::<&str>()?;

		let cylinder = slf.try_borrow()?;

		Ok(format!(
			"{}.{}({}, {})",
			module_name, class_name, cylinder.inner.radius, cylinder.inner.length
		))
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
