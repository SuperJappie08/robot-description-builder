use std::sync::{Arc, RwLock, Weak};

use pyo3::prelude::*;
use robot_description_builder::{Joint, JointBuilder, JointType};

use crate::link::PyLink;

#[derive(Debug, Clone)]
#[pyclass(name = "JointBuilder")]
pub struct PyJointBuilder {
	inner: JointBuilder,
}

#[pymethods]
impl PyJointBuilder {
	#[new]
	fn new(name: String, joint_type: PyJointType) -> PyJointBuilder {
		// ODDITY: use `Joint::new` because `JointBuilder::new` is private to the crate
		JointBuilder::new(name, joint_type.into()).into()
	}

	// /// TEMP implementation
	// fn add_origin_offset(&mut self, x: f32, y: f32, z: f32) {
	// 	self.inner = self.inner.clone().add_origin_offset((x, y, z));
	// }
}

impl From<JointBuilder> for PyJointBuilder {
	fn from(value: JointBuilder) -> Self {
		Self { inner: value }
	}
}

impl From<PyJointBuilder> for JointBuilder {
	fn from(value: PyJointBuilder) -> Self {
		value.inner
	}
}

#[derive(Debug)]
#[pyclass(name = "Joint", frozen)]
pub struct PyJoint {
	inner: Weak<RwLock<Joint>>,
	/// Python weakref to the python parent tree
	tree: PyObject,
}

impl PyJoint {
	fn try_internal(&self) -> PyResult<Arc<RwLock<Joint>>> {
		match self.inner.upgrade() {
			Some(l) => Ok(l),
			None => Err(pyo3::exceptions::PyReferenceError::new_err(
				"Joint already collected",
			)),
		}
	}
}

#[pymethods]
impl PyJoint {
	#[getter]
	fn name(&self) -> PyResult<String> {
		Ok(self.try_internal()?.read().unwrap().name().clone()) // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn parent_link(&self) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.read().unwrap().parent_link(),
			self.tree.clone(),
		)
			.into()) // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn child_link(&self) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.read().unwrap().child_link(),
			self.tree.clone(),
		)
			.into()) // TODO: Figure out if unwrap is Ok here?
	}

	// #[getter]
	// fn origin(&self) -> (f32, f32, f32) {
	// 	todo!()
	// }
}

impl From<(Weak<RwLock<Joint>>, PyObject)> for PyJoint {
	fn from(value: (Weak<RwLock<Joint>>, PyObject)) -> Self {
		Self {
			inner: value.0,
			tree: value.1,
		}
	}
}

impl From<(Arc<RwLock<Joint>>, PyObject)> for PyJoint {
	fn from(value: (Arc<RwLock<Joint>>, PyObject)) -> Self {
		Self {
			inner: Arc::downgrade(&value.0),
			tree: value.1,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass(name = "JointType")]
pub enum PyJointType {
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
