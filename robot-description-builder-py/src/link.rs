pub mod collision;
pub mod geometry;
pub mod inertial;
pub mod visual;

use std::sync::{Arc, RwLock, Weak};

use itertools::{process_results, Itertools};
use pyo3::{intern, prelude::*};
use robot_description_builder::{
	link_data::LinkParent, linkbuilding::LinkBuilder, JointBuilder, Link,
};

use collision::{PyCollision, PyCollisionBuilder};
use inertial::PyInertial;
use visual::{PyVisual, PyVisualBuilder};

use crate::{
	joint::{PyJoint, PyJointBuilder},
	PyKinematicTree,
};

pub(super) fn init_module(py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyLink>()?;
	module.add_class::<PyLinkBuilder>()?;

	collision::init_module(py, module)?;
	visual::init_module(py, module)?;
	inertial::init_module(py, module)?;
	// Inits the python geometry module with the the init_module function from the rust geometry module
	// Lots of packages do it like this and I don't know why, they do not do it like in the [PyO3 book](https://pyo3.rs/main/module#python-submodules)
	geometry::init_module(py, module)?;

	Ok(())
}

#[derive(Debug, Clone)]
#[pyclass(name = "LinkBuilder", module = "robot_description_builder.link")]
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

	/// Sets the `inertial` field to the given value, either `None` or a `Inertial`
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
	fn get_joints(&self) -> Vec<PyJointBuilder> {
		self.0.joints().iter().cloned().map(Into::into).collect()
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

	// /// Maybe direct construction
	fn build(&self) -> PyResult<Py<PyKinematicTree>> {
		// FIXME: NOT OK
		PyKinematicTree::create(self.0.clone().build_tree())
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

/// TODO: THINK COULD CHANGE TO WEAK AND RETURN ATTRIBUTE ERROR JUST LIKE WEAKREFF.PROXY
/// OR AQUIRE GIL TO MAKE A WEAKREFF.PROXY
#[derive(Debug)]
#[pyclass(
	name = "Link",
	module = "robot_description_builder.link",
	weakref,
	frozen
)]
pub struct PyLink {
	inner: Weak<RwLock<Link>>,
	/// Python weakref to the python parent tree
	tree: PyObject,
}

impl PyLink {
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
	/// The name or identifier of the `Link`
	#[getter]
	fn get_name(&self) -> PyResult<String> {
		Ok(self.try_internal()?.read().unwrap().name().clone()) // TODO: Figure out if unwrap is Ok here?
	}

	#[getter]
	/// The parent of the `Link`
	///
	/// This can be either a `KinematicTree` or a `Joint` depending if this `Link` is the root of a tree or not.  
	fn get_parent(slf: PyRef<'_, Self>) -> PyResult<Py<PyAny>> {
		match slf.try_internal()?.read().unwrap().parent() {
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
		// TODO: Still some unwraps?
		Ok(self
			.try_internal()?
			.read()
			.unwrap()
			.joints()
			.iter()
			.map(|joint| Into::<PyJoint>::into((Arc::downgrade(joint), self.tree.clone())))
			.collect())
	}

	#[getter]
	fn get_inertial(&self) -> PyResult<Option<PyInertial>> {
		Ok(self
			.try_internal()?
			.read()
			.unwrap()
			.inertial()
			.cloned()
			.map(Into::into))
	}

	#[getter]
	fn get_visuals(&self) -> PyResult<Vec<PyVisual>> {
		// TODO: Still some unwraps?
		Ok(self
			.try_internal()?
			.read()
			.unwrap()
			.visuals()
			.iter()
			.cloned()
			.map(Into::into)
			.collect())
	}

	#[getter]
	fn get_colliders(&self) -> PyResult<Vec<PyCollision>> {
		// TODO: Still some unwraps?
		Ok(self
			.try_internal()?
			.read()
			.unwrap()
			.colliders()
			.iter()
			.cloned()
			.map(Into::into)
			.collect())
	}

	/// Not Chained
	fn rebuild(&self) -> PyResult<PyLinkBuilder> {
		// TODO: Unwrap?
		Ok(self.try_internal()?.read().unwrap().rebuild().into())
	}

	fn try_attach_child(
		&self,
		link_builder: PyLinkBuilder,
		joint_builder: PyJointBuilder,
	) -> PyResult<()> {
		self.try_internal()?
			.write()
			.map_err(|_| pyo3::exceptions::PyAttributeError::new_err("Lock Poisoned"))?
			.try_attach_child(
				Into::<LinkBuilder>::into(link_builder),
				Into::<JointBuilder>::into(joint_builder),
			)
			.map_err(|_| pyo3::exceptions::PyKeyError::new_err("???"))?;
		Ok(())
	}

	// TODO: Add rebuild_chain but the API does not support it yey
	// TODO: Add attach chain methods

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let binding = self.try_internal()?;
		let link = binding.read().unwrap(); // FIXME: Unwrap ok?
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
				repr += ", visuals = [";
				// repr += &visuals.iter().map(|visual| PyVisual::from(visual.clone()).__repr__()).collect::<Vec<String>>().join(", ");
				repr += process_results(
					visuals
						.iter()
						.map(|visual| PyVisual::from(visual.clone()).__repr__(py)),
					|mut iter| iter.join(", "),
				)?
				.as_str();
				repr += "]";
			}
		}
		// TODO: EXPAND

		repr += ", ...)";
		Ok(repr)
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
