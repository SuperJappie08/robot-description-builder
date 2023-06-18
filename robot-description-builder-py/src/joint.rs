mod base_joint_builder;
mod smartjointbuilder;
use std::sync::{Arc, RwLock, Weak};

use pyo3::{exceptions::PyTypeError, intern, prelude::*, types::PyDict};
use robot_description_builder::{prelude::GroupIDChanger, Chained, Joint, JointBuilder, JointType};

use crate::{
	identifier::GroupIDError,
	link::{PyLink, PyLinkBuilder},
	transform::{PyMirrorAxis, PyTransform},
	utils::{init_pyclass_initializer, PyReadWriteable, TryIntoPy},
};

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyJoint>()?;
	module.add_class::<PyJointBuilder>()?;
	module.add_class::<PyJointType>()?;
	module.add_class::<PyJointBuilderChain>()?;

	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(
	name = "JointBuilder",
	module = "robot_description_builder.joint",
	subclass
)]
pub struct PyJointBuilder {
	builder: JointBuilder,
	transform: Option<Py<PyTransform>>,
}

impl PyJointBuilder {
	/// Internal new
	pub(crate) fn new(name: String, joint_type: PyJointType) -> Self {
		Self {
			builder: JointBuilder::new(name, joint_type.into()),
			transform: None,
		}
	}
}

#[pymethods]
impl PyJointBuilder {
	#[new]
	#[pyo3(signature = (name, joint_type, **kwds))]
	fn py_new(
		name: String,
		joint_type: PyJointType,
		kwds: Option<&PyDict>,
		py: Python<'_>,
	) -> PyResult<Self> {
		let mut result = Self::new(name, joint_type);

		if let Some(kwds) = kwds {
			if let Some(transform) = kwds
				.get_item(intern!(py, "transform"))
				.map(FromPyObject::extract)
			{
				result.transform = transform?;
				kwds.del_item(intern!(py, "transform"))?;
			}

			if !kwds.is_empty() {
				let qual_name = py
					.get_type::<Self>()
					.getattr(intern!(py, "__new__"))?
					.getattr(intern!(py, "__qualname__"))?;
				return Err(PyTypeError::new_err(format!(
					"{}() got an unexpected keyword argument {}",
					qual_name,
					kwds.keys()
						.get_item(0)
						.expect("The dict should not have been empty")
						.repr()?
				)));
			}
		}

		Ok(result)
	}

	#[getter]
	pub fn get_name(&self) -> String {
		self.builder.name().clone()
	}

	#[getter]
	fn get_joint_type(&self) -> PyJointType {
		(*self.builder.joint_type()).into()
	}

	#[getter]
	fn get_transform(&self) -> Option<Py<PyTransform>> {
		self.transform.clone()
	}

	#[setter]
	fn set_transform(&mut self, transform: Option<Py<PyTransform>>) {
		self.transform = transform
	}
	// TODO: transform advanced

	#[getter]
	fn get_child(&self) -> Option<PyLinkBuilder> {
		self.builder.child().cloned().map(Into::into)
	}

	#[getter]
	fn get_axis(&self) -> Option<(f32, f32, f32)> {
		self.builder.axis()
	}

	#[setter]
	fn set_axis(&mut self, axis: Option<(f32, f32, f32)>) {
		match (axis, self.builder.axis().is_some()) {
			(Some(axis), _) => self.builder.with_axis(axis),
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

	fn change_group_id(&mut self, new_group_id: String, _py: Python<'_>) -> PyResult<()> {
		self.builder
			.change_group_id(new_group_id)
			.map_err(GroupIDError::from)
	}

	fn apply_group_id(&mut self, _py: Python<'_>) {
		self.builder.apply_group_id()
	}
}

impl IntoPy<PyJointBuilder> for JointBuilder {
	fn into_py(self, py: Python<'_>) -> PyJointBuilder {
		PyJointBuilder {
			transform: self
				.transform()
				.copied()
				.map(Into::into)
				.map(|transform: PyTransform| {
					Py::new(py, transform).unwrap() // FIXME: Ok? This unwrap is mostly interpreter errors
				}),
			builder: self,
		}
	}
}

impl From<PyJointBuilder> for JointBuilder {
	fn from(mut value: PyJointBuilder) -> Self {
		if let Some(py_transform) = value.transform {
			value
				.builder
				.set_transform_simple(Python::with_gil(|py| (*py_transform.borrow(py)).into()))
		}

		value.builder
	}
}

#[derive(Debug, Clone)]
#[pyclass(name="JointBuilderChain", module="robot_description_builder.joint", extends=PyJointBuilder)]
pub struct PyJointBuilderChain;

impl PyJointBuilderChain {
	fn from_chained(py: Python<'_>, chained: Chained<JointBuilder>) -> PyClassInitializer<Self> {
		(Self, (*chained).clone().into_py(py)).into()
	}

	pub fn as_chained(slf: PyRef<'_, Self>) -> Chained<JointBuilder> {
		unsafe { Chained::new(slf.into_super().builder.clone()) }
	}
}

#[pymethods]
impl PyJointBuilderChain {
	fn mirror(slf: PyRef<'_, Self>, axis: PyMirrorAxis) -> PyResult<Py<Self>> {
		let py = slf.py();
		init_pyclass_initializer(
			Self::from_chained(py, Self::as_chained(slf).mirror(axis.into())),
			py,
		)
	}

	fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
		let class_name = slf
			.py()
			.get_type::<Self>()
			.getattr(intern!(slf.py(), "__qualname__"))?
			.extract::<&str>()?;

		// TODO: EXPAND
		Ok(format!(
			"{class_name}({}, {}, ...)",
			slf.as_ref().get_name(),
			slf.as_ref().get_joint_type().__pyo3__repr__()
		))
	}
}

impl TryIntoPy<Py<PyJointBuilderChain>> for Chained<JointBuilder> {
	fn try_into_py(self, py: Python<'_>) -> PyResult<Py<PyJointBuilderChain>> {
		init_pyclass_initializer(PyJointBuilderChain::from_chained(py, self), py)
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
	pub(crate) fn new_weak(joint: &Weak<RwLock<Joint>>, tree: &PyObject) -> Self {
		Self {
			inner: Weak::clone(joint),
			tree: tree.clone(),
		}
	}

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

	fn rebuild(&self, py: Python<'_>) -> PyResult<PyJointBuilder> {
		Ok(self.try_internal()?.py_read()?.rebuild().into_py(py))
	}

	fn rebuild_branch(&self, py: Python<'_>) -> PyResult<Py<PyJointBuilderChain>> {
		self.try_internal()?
			.py_read()?
			.rebuild_branch()
			.try_into_py(py)
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
