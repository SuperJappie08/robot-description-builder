mod kinematic_tree;
mod robot;

use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

pub use kinematic_tree::PyKinematicTree;
pub use robot::PyRobot;

use pyo3::{
	prelude::*,
	types::{PyDict, PyWeakrefProxy},
};
use robot_description_builder::{
	material::{data::MaterialData, Material},
	Joint, KinematicInterface, Link,
};

use crate::{
	joint::PyJoint,
	link::PyLink,
	material::PyMaterial,
	utils::{new_pydict_proxy, PyReadWriteable},
};

pub(super) fn init_module(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
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
	implementor: Py<PyWeakrefProxy>,
	links_weak: Weak<RwLock<HashMap<String, Weak<RwLock<Link>>>>>,
	joints_weak: Weak<RwLock<HashMap<String, Weak<RwLock<Joint>>>>>,
	material_weak: Weak<RwLock<HashMap<String, Arc<RwLock<MaterialData>>>>>,
}

impl PyKinematicBase {
	pub(in crate::cluster_objects) fn new(
		py: Python<'_>,
		tree: &impl KinematicInterface,
		weak_ref: &Bound<'_, PyWeakrefProxy>,
	) -> PyResult<Self> {
		let links_strong = tree.get_links();
		let joints_strong = tree.get_joints();
		let materials_strong = tree.get_materials();

		let result = Self {
			links_dict: PyDict::new_bound(py).unbind(),
			joints_dict: PyDict::new_bound(py).unbind(),
			material_dict: PyDict::new_bound(py).unbind(),
			implementor: weak_ref.clone().unbind(),
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
		self.links_dict.bind(py).update({
			self.links_weak
				.upgrade()
				.unwrap() // This unwrap is Ok
				.py_read()?
				.iter()
				.map(|(key, value)| (key.clone(), PyLink::new_weak(py, value, &self.implementor)))
				.collect::<HashMap<_, _>>()
				.into_py(py)
				.downcast_bound::<PyDict>(py)?
				.as_mapping()
		})
	}

	#[inline]
	pub(in crate::cluster_objects) fn update_joints(&self, py: Python<'_>) -> PyResult<()> {
		self.joints_dict.bind(py).update({
			self.joints_weak
				.upgrade()
				.unwrap() // This unwrap is Ok
				.py_read()?
				.iter()
				.map(|(key, value)| (key.clone(), PyJoint::new_weak(py, value, &self.implementor)))
				.collect::<HashMap<_, _>>()
				.into_py(py)
				.downcast_bound::<PyDict>(py)?
				.as_mapping()
		})
	}

	#[inline]
	pub(in crate::cluster_objects) fn update_materials(&self, py: Python<'_>) -> PyResult<()> {
		self.material_dict.bind(py).update({
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
				.downcast_bound::<PyDict>(py)?
				.as_mapping()
		})
	}
}

#[pymethods]
impl PyKinematicBase {
	#[getter]
	fn get_links(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_links(py)?;

		new_pydict_proxy(py, &self.links_dict)
	}

	#[getter]
	fn get_joints(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_joints(py)?;

		new_pydict_proxy(py, &self.joints_dict)
	}

	#[getter]
	fn get_materials(&mut self, py: Python<'_>) -> PyResult<PyObject> {
		self.update_materials(py)?;

		new_pydict_proxy(py, &self.material_dict)
	}
}
