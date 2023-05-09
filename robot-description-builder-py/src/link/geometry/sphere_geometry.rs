use pyo3::{intern, prelude::*};

use robot_description_builder::link_data::geometry::{GeometryInterface, SphereGeometry};

use super::PyGeometryBase;

#[derive(Debug)]
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

	fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
		// let module_name = slf
		// 	.get_type()
		// 	.getattr(intern!(slf.py(), "__module__"))?
		// 	.extract::<&str>()?;
		let class_name = slf
			.get_type()
			.getattr(intern!(slf.py(), "__qualname__"))?
			.extract::<&str>()?;

		Ok(format!(
			// "{}.{}({})",
			"{}({})",
			// module_name,
			class_name,
			slf.try_borrow()?.inner.radius
		))
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
