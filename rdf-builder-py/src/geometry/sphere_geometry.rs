use pyo3::{prelude::*, intern};

use rdf_builder_rs::link_data::geometry::{GeometryInterface, SphereGeometry};

use crate::geometry::PyGeometryBase;

#[derive(Debug)]
#[pyclass(name="SphereGeometry", extends=PyGeometryBase)]
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
	#[pyo3(signature = (radius))] //, *py_args, **py_kwargs))]
	fn py_new(
		radius: f32,
		// py_args: &PyTuple,
		// py_kwargs: Option<&PyDict>,
	) -> PyResult<(PySphereGeometry, PyGeometryBase)> {
		// if py_args.is_empty() || py_args.is_none() {
		// 	if let Some(radius_data) = py_kwargs.map(|dict| dict.get_item("radius")).flatten() {
		// 		match radius_data.extract::<f32>() {
		// 			Ok(radius) => Ok(Self::new(radius)),
		// 			Err(e) => Err(e),
		// 		}
		// 	} else {
		// 		Err(PyValueError::new_err("radius must be given"))
		// 	}
		// } else {
		// 	if let Ok((radius,)) = py_args.extract::<(f32,)>() {
		// 		Ok(Self::new(radius))
		// 	} else {
		// 		Err(PyValueError::new_err("radius must be a number"))
		// 	}
		// }

		// TODO: Maybe add kwargs and args checking
		Ok(Self::new(radius))
	}

	fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let module_name = slf.get_type().getattr(intern!(slf.py(), "__module__"))?.extract::<&str>()?;
		let class_name = slf.get_type().getattr(intern!(slf.py(),"__qualname__"))?.extract::<&str>()?;

		Ok(format!(
			"{}.{}({})",
			module_name,
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
