pub mod collision;
pub mod geometry;
pub mod inertial;
pub mod visual;

use std::sync::{Arc, RwLock, Weak};

use itertools::Itertools;
use pyo3::{ffi, intern, prelude::*, pyclass::CompareOp, AsPyPointer};
use robot_description_builder::{
	link_data::LinkParent, linkbuilding::LinkBuilder, prelude::GroupIDChanger, Chained,
	JointBuilder, Link,
};

use collision::{PyCollision, PyCollisionBuilder};
use inertial::PyInertial;
use visual::{PyVisual, PyVisualBuilder};

use crate::{
	cluster_objects::{PyKinematicBase, PyKinematicTree},
	exceptions::{AttachChainError, RebuildBranchError},
	identifier::GroupIDError,
	joint::{PyJoint, PyJointBuilder, PyJointBuilderBase, PyJointBuilderChain},
	transform::PyMirrorAxis,
	utils::{init_pyclass_initializer, PyReadWriteable, TryIntoPy},
};

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyLink>()?;
	module.add_class::<PyLinkBuilder>()?;
	module.add_class::<PyLinkBuilderChain>()?;

	collision::init_module(py, module)?;
	visual::init_module(py, module)?;
	inertial::init_module(py, module)?;
	// Inits the python geometry module with the the init_module function from the rust geometry module
	// Lots of packages do it like this and I don't know why, they do not do it like in the [PyO3 book](https://pyo3.rs/main/module#python-submodules)
	geometry::init_module(py, module)?;

	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(
	name = "LinkBuilder",
	module = "robot_description_builder.link",
	subclass
)]
pub struct PyLinkBuilder(LinkBuilder);

#[pymethods]
impl PyLinkBuilder {
	#[new]
	fn new(name: String) -> Self {
		LinkBuilder::new(name).into()
	}

	#[getter]
	fn get_name(&self) -> String {
		self.0.name().clone()
	}

	#[getter]
	fn get_visuals(&self) -> Vec<PyVisualBuilder> {
		self.0.visuals().iter().cloned().map(Into::into).collect()
	}

	#[getter]
	fn get_colliders(&self) -> Vec<PyCollisionBuilder> {
		self.0.colliders().iter().cloned().map(Into::into).collect()
	}

	#[getter]
	fn get_inertial(&self) -> Option<PyInertial> {
		self.0.inertial().copied().map(Into::into)
	}

	/// Sets the `inertial` field to the given value, either `None` or a `Inertial`.
	///
	/// # Notes:
	/// The `joint` field gets lost when assigning `None` a `LinkBuilder` which already has a `Inertial`.
	#[setter]
	fn set_inertial(&mut self, inertial_data: Option<PyInertial>) {
		match (inertial_data, self.0.inertial().is_some()) {
			(Some(inertial), _) => self.0 = self.0.clone().add_intertial(inertial.into()),
			(None, true) => {
				self.0 = {
					let mut new_builder = LinkBuilder::new(self.0.name());

					new_builder = self
						.0
						.visuals()
						.iter()
						.cloned()
						.fold(new_builder, |builder, visual| builder.add_visual(visual));
					new_builder = self
						.0
						.colliders()
						.iter()
						.cloned()
						.fold(new_builder, |builder, collider| {
							builder.add_collider(collider)
						});

					// TODO: Notify users about Joint Loss or fix it

					new_builder
				}
			}
			(None, false) => (),
		}
	}

	#[getter]
	fn get_joints(&self, py: Python<'_>) -> PyResult<Vec<Py<PyJointBuilder>>> {
		self.0
			.joints()
			.iter()
			.cloned()
			.map(|obj| (PyJointBuilder, obj.into_py(py)).into())
			.map(|initer| init_pyclass_initializer(initer, py))
			.process_results(|iter| iter.collect())
	}

	fn add_visual(&mut self, visual: PyVisualBuilder) -> Self {
		self.0 = self.0.clone().add_visual(visual.into());
		self.clone()
	}

	fn add_collider(&mut self, collision: PyCollisionBuilder) -> Self {
		self.0 = self.0.clone().add_collider(collision.into());
		self.clone()
	}

	fn add_inertial(&mut self, inertial: PyInertial) -> Self {
		self.0 = self.0.clone().add_intertial(inertial.into());

		self.clone()
	}

	fn build(&self, py: Python<'_>) -> PyResult<Py<PyKinematicTree>> {
		PyKinematicTree::create(self.0.clone().build_tree(), py)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = format!("'{}'", self.0.name());

		data += ", joints=[";
		data += self
			.get_joints(py)?
			.into_iter()
			.map(|joint_builder| joint_builder.into_ref(py).repr())
			.process_results(|mut iter| iter.join(", "))?
			.as_str();
		data += "]";

		if let Some(inertial) = self.get_inertial() {
			data += ", inertial=";
			data += inertial.__repr__(py)?.as_str();
		}

		if !self.0.visuals().is_empty() {
			data += ", visuals=[";
			data += self
				.get_visuals()
				.into_iter()
				.map(|visual_builder| visual_builder.__repr__(py))
				.process_results(|mut iter| iter.join(", "))?
				.as_str();
			data += "]";
		}

		if !self.0.colliders().is_empty() {
			data += ", colliders=[";
			data += self
				.get_colliders()
				.into_iter()
				.map(|collision_builder| collision_builder.__repr__(py))
				.process_results(|mut iter| iter.join(", "))?
				.as_str();
			data += "]";
		}

		Ok(format!("{class_name}({data})"))
	}

	fn change_group_id(&mut self, new_group_id: String, _py: Python<'_>) -> PyResult<()> {
		self.0
			.change_group_id(new_group_id)
			.map_err(GroupIDError::from)
	}

	fn apply_group_id(&mut self, _py: Python<'_>) {
		self.0.apply_group_id()
	}
}

impl From<LinkBuilder> for PyLinkBuilder {
	fn from(value: LinkBuilder) -> Self {
		Self(value)
	}
}

impl From<PyLinkBuilder> for LinkBuilder {
	fn from(value: PyLinkBuilder) -> Self {
		value.0
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "LinkBuilderChain", module = "robot_description_builder.link", extends=PyLinkBuilder)]
pub struct PyLinkBuilderChain;

impl PyLinkBuilderChain {
	fn from_chained(chained: Chained<LinkBuilder>) -> PyClassInitializer<Self> {
		(Self, Into::<PyLinkBuilder>::into((*chained).clone())).into()
	}

	fn as_chained(slf: PyRef<'_, Self>) -> Chained<LinkBuilder> {
		unsafe { Chained::new(slf.into_super().0.clone()) }
	}
}

#[pymethods]
impl PyLinkBuilderChain {
	fn mirror(slf: PyRef<'_, Self>, axis: PyMirrorAxis) -> PyResult<Py<Self>> {
		let py = slf.py();
		init_pyclass_initializer(
			Self::from_chained(Self::as_chained(slf).mirror(axis.into())),
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
		Ok(format!("{class_name}({}, ...)", slf.as_ref().get_name(),))
	}
}

impl TryIntoPy<Py<PyLinkBuilderChain>> for Chained<LinkBuilder> {
	fn try_into_py(self, py: Python<'_>) -> PyResult<Py<PyLinkBuilderChain>> {
		init_pyclass_initializer(PyLinkBuilderChain::from_chained(self), py)
	}
}

#[derive(Debug)]
#[pyclass(
	name = "Link",
	module = "robot_description_builder.link",
	weakref,
	frozen
)]
pub struct PyLink {
	inner: Weak<RwLock<Link>>,
	/// Python weakref.proxy to the python parent tree.
	tree: PyObject,
}

impl PyLink {
	pub(crate) fn new_weak(link: &Weak<RwLock<Link>>, tree: &PyObject) -> Self {
		Self {
			inner: link.clone(),
			tree: tree.clone(),
		}
	}

	fn try_internal(&self) -> PyResult<Arc<RwLock<Link>>> {
		match self.inner.upgrade() {
			Some(l) => Ok(l),
			None => Err(pyo3::exceptions::PyReferenceError::new_err(
				"Link already collected",
			)),
		}
	}
}

#[pymethods]
impl PyLink {
	/// The name or identifier of the `Link`.
	#[getter]
	fn get_name(&self) -> PyResult<String> {
		Ok(self.try_internal()?.py_read()?.name().clone())
	}

	#[getter]
	/// The parent of the `Link`.
	///
	/// This can be either a `KinematicTree` or a `Joint` depending if this `Link` is the root of a tree or not.
	fn get_parent(slf: PyRef<'_, Self>) -> PyResult<Py<PyAny>> {
		match slf.try_internal()?.py_read()?.parent() {
			LinkParent::KinematicTree(_) => Ok(slf.tree.clone()),
			LinkParent::Joint(joint) => Ok(Into::<PyJoint>::into((
				Weak::upgrade(joint).unwrap(),
				slf.tree.clone(),
			))
			.into_py(slf.py())),
		}
	}

	#[getter]
	fn get_joints(&self) -> PyResult<Vec<PyJoint>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.joints()
			.iter()
			.map(|joint| Into::<PyJoint>::into((Arc::downgrade(joint), self.tree.clone())))
			.collect())
	}

	#[getter]
	fn get_inertial(&self) -> PyResult<Option<PyInertial>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.inertial()
			.cloned()
			.map(Into::into))
	}

	#[getter]
	fn get_visuals(&self) -> PyResult<Vec<PyVisual>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.visuals()
			.iter()
			.cloned()
			.map(Into::into)
			.collect())
	}

	#[getter]
	fn get_colliders(&self) -> PyResult<Vec<PyCollision>> {
		Ok(self
			.try_internal()?
			.py_read()?
			.colliders()
			.iter()
			.cloned()
			.map(Into::into)
			.collect())
	}

	fn rebuild(&self) -> PyResult<PyLinkBuilder> {
		Ok(self.try_internal()?.py_read()?.rebuild().into())
	}

	fn rebuild_branch(&self, py: Python<'_>) -> PyResult<Py<PyLinkBuilderChain>> {
		self.try_internal()?
			.py_read()?
			.rebuild_branch()
			.map_err(RebuildBranchError::from)?
			.try_into_py(py)
	}

	fn try_attach_child(
		&self,
		joint_builder: PyJointBuilderBase,
		link_builder: PyLinkBuilder,
		py: Python<'_>,
	) -> PyResult<()> {
		self.try_internal()?
			.py_write()?
			.try_attach_child(
				Into::<JointBuilder>::into(joint_builder),
				Into::<LinkBuilder>::into(link_builder),
			)
			.map_err(AttachChainError::from)?;

		let tree = unsafe {
			PyObject::from_borrowed_ptr(py, ffi::PyWeakref_GetObject(self.tree.as_ptr()))
		};
		tree.extract::<PyRef<'_, PyKinematicBase>>(py)?
			.update_all(py)?;

		Ok(())
	}

	// TODO: Add rebuild_chain but the API does not support it yey
	// TODO: Add attach chain methods

	fn attach_joint_chain(
		&self,
		joint_chain: PyRef<'_, PyJointBuilderChain>,
		py: Python<'_>,
	) -> PyResult<()> {
		self.try_internal()?
			.py_write()?
			.attach_joint_chain(PyJointBuilderChain::as_chained(joint_chain))
			.map_err(AttachChainError::from)?;

		let tree = unsafe {
			PyObject::from_borrowed_ptr(py, ffi::PyWeakref_GetObject(self.tree.as_ptr()))
		};
		tree.extract::<PyRef<'_, PyKinematicBase>>(py)?
			.update_all(py)?;

		Ok(())
	}

	/// TODO: Maybe rewrite
	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let binding = self.try_internal()?;
		let link = binding.py_read()?;
		let mut repr = format!(
			"{}('{}'",
			py.get_type::<Self>()
				.getattr(intern!(py, "__qualname__"))?
				.extract::<&str>()?,
			link.name()
		);

		{
			let visuals = link.visuals();
			if !visuals.is_empty() {
				repr += ", visuals=[";
				repr += visuals
					.iter()
					.map(|visual| PyVisual::from(visual.clone()).__repr__(py))
					.process_results(|mut iter| iter.join(", "))?
					.as_str();
				repr += "]";
			}
		}

		{
			let colliders = link.colliders();
			if !colliders.is_empty() {
				repr += ", colliders=[";
				repr += colliders
					.iter()
					.map(|collider| PyCollision::from(collider.clone()).__repr__(py))
					.process_results(|mut iter| iter.join(", "))?
					.as_str();
				repr += "]";
			}
		}

		{
			if let Some(inertial) = link.inertial() {
				repr += ", inertial=";
				repr += PyInertial::from(*inertial).__repr__(py)?.as_str();
			}
		}

		repr += ")";
		Ok(repr)
	}

	fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
		// FIXME: This auto genrates __hash__ is that ok?
		match op {
			CompareOp::Eq => {
				(self.inner.ptr_eq(&other.inner) && self.tree.is(&other.tree)).into_py(py)
			}
			CompareOp::Ne => {
				(!self.inner.ptr_eq(&other.inner) || !self.tree.is(&other.tree)).into_py(py)
			}
			_ => py.NotImplemented(),
		}
	}
}

impl From<(Weak<RwLock<Link>>, PyObject)> for PyLink {
	fn from(value: (Weak<RwLock<Link>>, PyObject)) -> Self {
		Self {
			inner: value.0,
			tree: value.1,
		}
	}
}

impl From<(Arc<RwLock<Link>>, PyObject)> for PyLink {
	fn from(value: (Arc<RwLock<Link>>, PyObject)) -> Self {
		Self {
			inner: Arc::downgrade(&value.0),
			tree: value.1,
		}
	}
}
