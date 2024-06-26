mod base_joint_builder;
mod generic_joint_builder;
mod smartjointbuilder;
use std::sync::{Arc, RwLock, Weak};

use pyo3::{exceptions::PyReferenceError, prelude::*, types::PyWeakrefProxy};
use robot_description_builder::{joint_data, Chained, Joint, JointBuilder, JointType};

use crate::{
	exceptions::RebuildBranchError,
	link::PyLink,
	transform::{PyMirrorAxis, PyTransform},
	utils::{init_pyclass_initializer, PyReadWriteable, TryIntoPy},
};

pub use base_joint_builder::{PyJointBuilderBase, PyJointBuilderMethods};
pub use generic_joint_builder::PyJointBuilder;

pub(super) fn init_module(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
	module.add_class::<PyJoint>()?;
	module.add_class::<PyJointBuilder>()?;
	module.add_class::<PyJointType>()?;
	module.add_class::<PyJointBuilderChain>()?;

	module.add_class::<PyJointBuilderBase>()?;
	Ok(())
}

#[derive(FromPyObject)]
pub(in crate::joint) struct PyLimit {
	lower: Option<f32>,
	upper: Option<f32>,
	effort: f32,
	velocity: f32,
}

impl From<PyLimit> for joint_data::LimitData {
	fn from(value: PyLimit) -> Self {
		Self {
			lower: value.lower,
			upper: value.upper,
			effort: value.effort,
			velocity: value.velocity,
		}
	}
}

#[derive(Debug, Clone)]
#[pyclass(name="JointBuilderChain", module="robot_description_builder.joint", extends=PyJointBuilder)]
pub struct PyJointBuilderChain;

impl PyJointBuilderChain {
	fn from_chained(py: Python<'_>, chained: Chained<JointBuilder>) -> PyClassInitializer<Self> {
		PyClassInitializer::from(IntoPy::<PyJointBuilderBase>::into_py(
			(*chained).clone(),
			py,
		))
		.add_subclass(PyJointBuilder)
		.add_subclass(Self)
	}

	pub fn as_chained(slf: PyRef<'_, Self>) -> Chained<JointBuilder> {
		let builder = slf.into_super().as_ref().builder.clone();
		unsafe { Chained::new(builder) }
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

	fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
		let class_name = slf.get_type().qualname()?;

		let super_slf = slf.borrow().into_super();

		// TODO: EXPAND
		Ok(format!(
			"{class_name}({}, {}, ...)",
			super_slf.as_ref().get_name(),
			super_slf.as_ref().get_joint_type().__pyo3__repr__()
		))
	}
}

impl TryIntoPy<Py<PyJointBuilderChain>> for Chained<JointBuilder> {
	fn try_into_py(self, py: Python<'_>) -> PyResult<Py<PyJointBuilderChain>> {
		init_pyclass_initializer(PyJointBuilderChain::from_chained(py, self), py)
	}
}

#[derive(Debug)]
#[pyclass(name = "Joint", module = "robot_description_builder.joint", frozen)]
pub struct PyJoint {
	inner: Weak<RwLock<Joint>>,
	/// Python weakref.proxy to the python parent tree.
	tree: Py<PyWeakrefProxy>,
}

impl PyJoint {
	pub(crate) fn new_weak(
		py: Python<'_>,
		joint: &Weak<RwLock<Joint>>,
		tree: &Py<PyWeakrefProxy>,
	) -> Self {
		Self {
			inner: Weak::clone(joint),
			tree: tree.clone_ref(py),
		}
	}

	fn try_internal(&self) -> PyResult<Arc<RwLock<Joint>>> {
		match self.inner.upgrade() {
			Some(l) => Ok(l),
			None => Err(PyReferenceError::new_err("Joint already collected")),
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
	fn get_parent_link(&self, py: Python<'_>) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.py_read()?.parent_link(),
			self.tree.bind(py).clone(),
		)
			.into())
	}

	#[getter]
	fn get_child_link(&self, py: Python<'_>) -> PyResult<PyLink> {
		Ok((
			self.try_internal()?.py_read()?.child_link(),
			self.tree.bind(py).clone(),
		)
			.into())
	}

	#[getter]
	fn get_transform(&self) -> PyResult<Option<PyTransform>> {
		let transform = *self.try_internal()?.py_read()?.transform();
		match transform.contains_some() {
			true => Ok(Some(transform.into())),
			false => Ok(None),
		}
	}

	#[getter]
	fn get_axis(&self) -> PyResult<Option<(f32, f32, f32)>> {
		Ok(self.try_internal()?.py_read()?.axis())
	}

	fn rebuild(&self, py: Python<'_>) -> PyResult<Py<PyJointBuilder>> {
		init_pyclass_initializer(
			(
				PyJointBuilder,
				self.try_internal()?.py_read()?.rebuild().into_py(py),
			)
				.into(),
			py,
		)
	}

	fn rebuild_branch(&self, py: Python<'_>) -> PyResult<Py<PyJointBuilderChain>> {
		self.try_internal()?
			.py_read()?
			.rebuild_branch()
			.map_err(RebuildBranchError::from)?
			.try_into_py(py)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let binding = self.try_internal()?;
		let joint = binding.py_read()?;
		let mut repr = format!(
			"{}('{}', {}",
			py.get_type_bound::<Self>().qualname()?,
			joint.name(),
			Into::<PyJointType>::into(joint.joint_type()).__pyo3__repr__()
		);

		// TODO: EXPAND

		repr += ", ...)";
		Ok(repr)
	}
}

impl From<(Weak<RwLock<Joint>>, Bound<'_, PyWeakrefProxy>)> for PyJoint {
	fn from(value: (Weak<RwLock<Joint>>, Bound<'_, PyWeakrefProxy>)) -> Self {
		Self {
			inner: value.0,
			tree: value.1.unbind(),
		}
	}
}

impl From<(Arc<RwLock<Joint>>, Bound<'_, PyWeakrefProxy>)> for PyJoint {
	fn from(value: (Arc<RwLock<Joint>>, Bound<'_, PyWeakrefProxy>)) -> Self {
		Self {
			inner: Arc::downgrade(&value.0),
			tree: value.1.unbind(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[pyclass(name = "JointType", module = "robot_description_builder.joint", eq)]
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
