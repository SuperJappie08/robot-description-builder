use std::sync::{Arc, RwLock, Weak};

use pyo3::{intern, prelude::*};
use robot_description_builder::{Joint, JointBuilder, JointType};

use crate::{
	link::{PyLink, PyLinkBuilder},
	transform::PyTransform,
	utils::PyReadWriteable,
};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyJoint>()?;
	module.add_class::<PyJointBuilder>()?;
	module.add_class::<PyJointType>()?;

	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "JointBuilder", module = "robot_description_builder.joint")]
pub struct PyJointBuilder(JointBuilder);

#[pymethods]
impl PyJointBuilder {
	#[new]
	fn new(name: String, joint_type: PyJointType) -> Self {
		Self(JointBuilder::new(name, joint_type.into()))
	}

	#[getter]
	pub fn get_name(&self) -> String {
		self.0.name().clone()
	}

	#[getter]
	fn get_joint_type(&self) -> PyJointType {
		(*self.0.joint_type()).into()
	}

	// TODO: Origin

	#[getter]
	fn get_child(&self) -> Option<PyLinkBuilder> {
		self.0.child().cloned().map(Into::into)
	}

	#[getter]
	fn get_axis(&self) -> Option<(f32, f32, f32)> {
		self.0.axis()
	}

	#[setter]
	fn set_axis(&mut self, axis: Option<(f32, f32, f32)>) {
		match (axis, self.0.axis().is_some()) {
			(Some(axis), _) => self.0.with_axis(axis),
			(None, true) => {
				// This would be easier
				// self.inner = JointBuilder{
				// 	axis: None,
				// 	..self.inner.clone()
				// }
				// TODO: This is a lot of work, it is easier to change the Rust libary
				todo!()
			}
			(None, false) => (),
		}
	}

	// /// TEMP implementation
	// fn add_origin_offset(&mut self, x: f32, y: f32, z: f32) {
	// 	self.inner = self.inner.clone().add_origin_offset((x, y, z));
	// }

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;
		// TODO: EXPAND
		Ok(format!(
			"{class_name}({}, {}, ...)",
			self.get_name(),
			self.get_joint_type().__pyo3__repr__()
		))
	}
}

impl From<JointBuilder> for PyJointBuilder {
	fn from(value: JointBuilder) -> Self {
		Self(value)
	}
}

impl From<PyJointBuilder> for JointBuilder {
	fn from(value: PyJointBuilder) -> Self {
		value.0
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "Joint", module = "robot_description_builder.joint", frozen)]
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
	pub fn get_name(&self) -> PyResult<String> {
		Ok(self.try_internal()?.py_read()?.name().clone())
	}

	#[getter]
	fn get_joint_type(&self) -> PyResult<PyJointType> {
		Ok(self.try_internal()?.py_read()?.joint_type().into())
	}

	#[getter]
	fn get_parent_link(&self) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.py_read()?.parent_link(),
			self.tree.clone(),
		)
			.into())
	}

	#[getter]
	fn get_child_link(&self) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.py_read()?.child_link(),
			self.tree.clone(),
		)
			.into())
	}

	#[getter]
	fn get_origin(&self) -> PyResult<Option<PyTransform>> {
		let origin = *self.try_internal()?.py_read()?.origin();
		match origin.contains_some() {
			true => Ok(Some(origin.into())),
			false => Ok(None),
		}
	}

	#[getter]
	fn get_axis(&self) -> PyResult<Option<(f32, f32, f32)>> {
		Ok(self.try_internal()?.py_read()?.axis())
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let binding = self.try_internal()?;
		let joint = binding.py_read()?;
		let mut repr = format!(
			"{}('{}', {}",
			py.get_type::<Self>()
				.getattr(intern!(py, "__qualname__"))?
				.extract::<&str>()?,
			joint.name(),
			Into::<PyJointType>::into(joint.joint_type()).__pyo3__repr__()
		);

		// TODO: EXPAND

		repr += ", ...)";
		Ok(repr)
	}
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
#[pyclass(name = "JointType", module = "robot_description_builder.joint")]
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
