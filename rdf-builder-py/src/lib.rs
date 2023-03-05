use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use rdf_builder_rs::{Joint, JointType, KinematicInterface, KinematicTree, Link, Robot};

#[pyclass(name = "Robot")]
struct PyRobot {
	inner: Robot,
}

#[pymethods]
impl PyRobot {
	#[getter]
	fn name(&self) -> String {
		self.inner.name.clone()
	}
}

impl From<Robot> for PyRobot {
	fn from(value: Robot) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "KinematicTree")]
struct PyKinematicTree {
	inner: KinematicTree,
}

#[pymethods]
impl PyKinematicTree {
	#[getter]
	fn root_link(&self) -> PyLink {
		self.inner.get_root_link().into()
	}

	#[getter]
	fn newest_link(&self) -> PyLink {
		self.inner.get_newest_link().into()
	}

	// #[getter] // get_links
	// fn links(&self) -> HashMap<String, std::sync::Weak<RwLock<Link>>> {
	//     self.inner.get_links().into_inner().unwrap().clone()
	// }

	// fn get_joints(&self) -> Arc<RwLock<std::collections::HashMap<String, std::sync::Weak<RwLock<rdf_builder_rs::Joint>>>>> {
	//     todo!()
	// }

	// fn get_materials(&self) -> Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<rdf_builder_rs::Material>>>>> {
	//     todo!()
	// }

	// fn get_transmissions(&self) -> Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<rdf_builder_rs::Transmission>>>>> {
	//     todo!()
	// }

	fn get_link(&self, name: String) -> Option<PyLink> {
		self.inner.get_link(&name).map(Into::into)
	}

	fn get_joint(&self, name: String) -> Option<PyJoint> {
		self.inner.get_joint(&name).map(Into::into)
	}
}

impl From<KinematicTree> for PyKinematicTree {
	fn from(value: KinematicTree) -> Self {
		Self { inner: value }
	}
}

impl From<PyKinematicTree> for KinematicTree {
	fn from(value: PyKinematicTree) -> Self {
		value.inner
	}
}

#[derive(Debug)]
#[pyclass(name = "Link")]
struct PyLink {
	inner: Arc<RwLock<Link>>,
}

#[pymethods]
impl PyLink {
	#[staticmethod]
	fn new(name: String) -> PyKinematicTree {
		Link::new(name).into()
	}

	#[getter]
	fn name(&self) -> String {
		self.inner.try_read().unwrap().get_name() // TODO: Figure out if unwrap is Ok here?
	}

	///TODO: Joint Type Selection
	fn try_attach_child(&self, tree: PyKinematicTree, joint_name: String, joint_type: PyJointType) {
		// FIXME: Need to do somethign with error
		self.inner
			.try_write()
			.unwrap() // TODO: Figure out if unwrap is Ok here?
			.try_attach_child(
				Into::<KinematicTree>::into(tree).into(),
				joint_name,
				joint_type.into(),
			)
			.unwrap() // TODO: Figure out if unwrap is Ok here?
	}
}

impl From<Arc<RwLock<Link>>> for PyLink {
	fn from(value: Arc<RwLock<Link>>) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug)]
#[pyclass(name = "Joint")]
struct PyJoint {
	inner: Arc<RwLock<Joint>>,
}

#[pymethods]
impl PyJoint {
	#[getter]
	fn name(&self) -> String {
		self.inner.try_read().unwrap().name.clone() // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn parent_link(&self) -> PyLink {
		self.inner.read().unwrap().get_parent_link().into() // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn child_link(&self) -> PyLink {
		self.inner.read().unwrap().get_child_link().into() // TODO: Figure out if unwrap is Ok here?
	}
}

impl From<Arc<RwLock<Joint>>> for PyJoint {
	fn from(value: Arc<RwLock<Joint>>) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass(name = "JointType")]
enum PyJointType {
	Fixed,
	Revolute,
	Continuous,
	Prismatic,
	Floating,
	Planar,
}

impl From<JointType> for PyJointType {
	fn from(value: JointType) -> Self {
		match value {
			JointType::Fixed => Self::Fixed,
			JointType::Revolute => Self::Revolute,
			JointType::Continuous => Self::Continuous,
			JointType::Prismatic => Self::Prismatic,
			JointType::Floating => Self::Floating,
			JointType::Planar => Self::Planar,
		}
	}
}

impl From<PyJointType> for JointType {
	fn from(value: PyJointType) -> Self {
		match value {
			PyJointType::Fixed => Self::Fixed,
			PyJointType::Revolute => Self::Revolute,
			PyJointType::Continuous => Self::Continuous,
			PyJointType::Prismatic => Self::Prismatic,
			PyJointType::Floating => Self::Floating,
			PyJointType::Planar => Self::Planar,
		}
	}
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rdf_builder_py(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	m.add_class::<PyRobot>()?;
	m.add_class::<PyKinematicTree>()?;
	m.add_class::<PyLink>()?;
	m.add_class::<PyJoint>()?;
	m.add_class::<PyJointType>()?;

	Ok(())
}
