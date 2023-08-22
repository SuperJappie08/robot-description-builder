mod kinematic_tree;
mod robot;

use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

pub use kinematic_tree::PyKinematicTree;
pub use robot::PyRobot;

use pyo3::{ffi, prelude::*, types::PyDict};
use robot_description_builder::{
	material::{data::MaterialData, Material},
	Joint, KinematicInterface, Link,
};

use crate::{joint::PyJoint, link::PyLink, material::PyMaterial, utils::PyReadWriteable};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyKinematicBase>()?;
	module.add_class::<PyKinematicTree>()?;
	module.add_class::<PyRobot>()?;

	Ok(())
}

#[derive(Debug)]
#[pyclass(
	name = "KinematicBase",
	module = "robot_description_builder.cluster_objects",
	subclass,
	weakref
)]
pub struct PyKinematicBase {
	links_dict: Py<PyDict>,
	joints_dict: Py<PyDict>,
	material_dict: Py<PyDict>,

	// Weakref to object above
	implementor: PyObject,
	links_weak: Weak<RwLock<HashMap<String, Weak<RwLock<Link>>>>>,
	joints_weak: Weak<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>,
	material_weak: Weak<RwLock<HashMap<String, Arc<RwLock<MaterialData>>>>>,
}

impl PyKinematicBase {
	pub(in crate::cluster_objects) fn new(
		py: Python<'_>,
		tree: &impl KinematicInterface,
		weak_ref: &PyObject,
	) -> PyResult<Self> {
		let links_strong = tree.get_links();
		let joints_strong = tree.get_joints();
		let materials_strong = tree.get_materials();

		let result = Self {
			links_dict: PyDict::new(py).into_py(py),
			joints_dict: PyDict::new(py).into_py(py),
			material_dict: PyDict::new(py).into_py(py),
			implementor: weak_ref.clone(),
			links_weak: Arc::downgrade(&links_strong),
			joints_weak: Arc::downgrade(&joints_strong),
			material_weak: Arc::downgrade(&materials_strong),
		};

		result.update_all(py)?;

		Ok(result)
	}

	#[inline]
	pub(crate) fn update_all(&self, py: Python<'_>) -> PyResult<()> {
		self.update_links(py)?;
		self.update_joints(py)?;
		self.update_materials(py)
	}

	#[inline]
	pub(in crate::cluster_objects) fn update_links(&self, py: Python<'_>) -> PyResult<()> {
		self.links_dict.as_ref(py).update({
			self.links_weak
				.upgrade()
				.unwrap() // This unwrap is Ok
				.py_read()?
				.iter()
				.map(|(key, value)| (key.clone(), PyLink::new_weak(value, &self.implementor)))
				.collect::<HashMap<_, _>>()
				.into_py(py)
				.downcast::<PyDict>(py)?
				.as_mapping()
		})
	}

	#[inline]
	pub(in crate::cluster_objects) fn update_joints(&self, py: Python<'_>) -> PyResult<()> {
		self.joints_dict.as_ref(py).update({
			self.joints_weak
				.upgrade()
				.unwrap() // This unwrap is Ok
				.py_read()?
				.iter()
				.map(|(key, value)| (key.clone(), PyJoint::new_weak(value, &self.implementor)))
				.collect::<HashMap<_, _>>()
				.into_py(py)
				.downcast::<PyDict>(py)?
				.as_mapping()
		})
	}

	#[inline]
	pub(in crate::cluster_objects) fn update_materials(&self, py: Python<'_>) -> PyResult<()> {
		self.material_dict.as_ref(py).update({
			self.material_weak
				.upgrade()
				.unwrap() // This unwrap is Ok
				.py_read()?
				.iter()
				.map(|(key, value)| {
					(
						key.clone(),
						Into::<Material>::into((key.clone(), value.clone())).into(),
					)
				})
				.collect::<HashMap<_, PyMaterial>>()
				.into_py(py)
				.downcast::<PyDict>(py)?
				.as_mapping()
		})
	}
}

#[pymethods]
impl PyKinematicBase {
	#[getter]
	fn get_links(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_links(py)?;

		unsafe {
			Py::from_owned_ptr_or_err(
				py,
				ffi::PyDictProxy_New(self.links_dict.clone().as_ref(py).as_mapping().into_ptr()),
			)
		}
	}

	#[getter]
	fn get_joints(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_joints(py)?;

		unsafe {
			Py::from_owned_ptr_or_err(
				py,
				ffi::PyDictProxy_New(self.joints_dict.clone().as_ref(py).as_mapping().into_ptr()),
			)
		}
	}

	#[getter]
	fn get_materials(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_materials(py)?;

		unsafe {
			Py::from_owned_ptr_or_err(
				py,
				ffi::PyDictProxy_New(
					self.material_dict
						.clone()
						.as_ref(py)
						.as_mapping()
						.into_ptr(),
				),
			)
		}
	}
}
