use std::sync::{Arc, RwLock};

use pyo3::prelude::*;
use rdf_builder_rs::{Joint, JointBuilder, JointType};

use crate::PyLink;

#[derive(Debug)]
#[pyclass(name = "Joint")]
pub struct PyJoint {
	inner: Arc<RwLock<Joint>>,
}

#[pymethods]
impl PyJoint {
	// #[new]
	// fn build_a_joint() ->

	#[getter]
	fn name(&self) -> String {
		self.inner.try_read().unwrap().get_name().clone() // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn parent_link(&self) -> PyLink {
		self.inner.try_read().unwrap().get_parent_link().into() // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn child_link(&self) -> PyLink {
		self.inner.try_read().unwrap().get_child_link().into() // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	fn origin(&self) -> (f32, f32, f32) {
		todo!()
	}
}

impl From<Arc<RwLock<Joint>>> for PyJoint {
	fn from(value: Arc<RwLock<Joint>>) -> Self {
		Self { inner: value }
	}
}

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

	/// TEMP implementation
	fn add_origin_offset(&mut self, x: f32, y: f32, z: f32) {
		self.inner = self.inner.clone().add_origin_offset((x, y, z));
	}
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
