use pyo3::{prelude::*, types::PyWeakrefProxy, PyClassInitializer};

use robot_description_builder::{KinematicInterface, KinematicTree};

use super::{robot::PyRobot, PyKinematicBase};

#[cfg(feature = "experimental-transmission")]
use crate::transmission::PyTransmission;

use crate::{
	joint::{PyJoint, PyJointBuilderChain},
	link::{PyLink, PyLinkBuilderChain},
	material::PyMaterial,
	utils::{self, TryIntoPy},
};

#[derive(Debug)]
#[pyclass(
    name = "KinematicTree",
    module = "robot_description_builder.cluster_objects",
    weakref,
    extends = PyKinematicBase)]
pub struct PyKinematicTree {
	pub(super) inner: KinematicTree,
	/// Python weakref to self.
	me: Py<PyWeakrefProxy>,
}

impl PyKinematicTree {
	pub(crate) fn create(tree: KinematicTree, py: Python<'_>) -> PyResult<Py<PyKinematicTree>> {
		// Temporary make it a broken value, so we can overwrite it.
		let me = unsafe { py.None().downcast_bound_unchecked(py) }.clone();

		let base = PyKinematicBase::new(py, &tree, &me)?;

		let tree = utils::init_pyclass_initializer(
			PyClassInitializer::from((
				PyKinematicTree {
					inner: tree,
					me: me.unbind(),
				},
				base,
			)),
			py,
		)?;

		tree.borrow_mut(py).me = PyWeakrefProxy::new_bound(tree.bind(py))?.unbind();

		let tree_weak = tree.borrow(py).me.clone_ref(py);

		{
			let mut base = tree.borrow_mut(py).into_super();
			base.implementor = tree_weak;

			base.update_all(py)?;
		}

		Ok(tree)
	}

	pub(crate) fn get_weak<'py>(&self, py: Python<'py>) -> Bound<'py, PyWeakrefProxy> {
		self.me.bind(py).clone()
	}

	pub(crate) fn into_inner(self) -> KinematicTree {
		self.inner
	}
}

#[pymethods]
impl PyKinematicTree {
	#[getter]
	fn get_root_link(&self, py: Python<'_>) -> PyLink {
		(self.inner.get_root_link(), self.get_weak(py)).into()
	}

	#[getter]
	fn get_newest_link(&self, py: Python<'_>) -> PyLink {
		(self.inner.get_newest_link(), self.get_weak(py)).into()
	}

	fn get_link(&self, py: Python<'_>, name: String) -> Option<PyLink> {
		self.inner
			.get_link(&name)
			.map(|link| (link, self.get_weak(py)).into())
	}

	fn get_joint(&self, py: Python<'_>, name: String) -> Option<PyJoint> {
		self.inner
			.get_joint(&name)
			.map(|joint| (joint, self.get_weak(py)).into())
	}

	fn get_material(&self, name: String) -> Option<PyMaterial> {
		self.inner.get_material(&name).map(Into::into)
	}

	#[cfg(feature = "experimental-transmission")]
	fn get_transmission(&self, name: String) -> Option<PyTransmission> {
		self.inner
			.get_transmission(&name)
			.map(|transmission| (transmission, self.get_weak()).into())
	}

	/* TODO: This migth need to not return a Optional */
	/* TODO: Should become chained builder */
	fn yank_link(
		slf: PyRef<'_, Self>,
		name: String,
		py: Python<'_>,
	) -> PyResult<Option<Py<PyLinkBuilderChain>>> {
		match slf
			.inner
			.yank_link(&name)
			.map(|link_builder| link_builder.try_into_py(py))
		{
			Some(Ok(chained_linkbuilder)) => {
				slf.into_super().update_all(py)?;
				Ok(Some(chained_linkbuilder))
			}
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	/* TODO: This migth need to not return a Optional */
	fn yank_joint(
		slf: PyRef<'_, Self>,
		name: String,
		py: Python<'_>,
	) -> PyResult<Option<Py<PyJointBuilderChain>>> {
		match slf
			.inner
			.yank_joint(&name)
			.map(|joint_builder| joint_builder.try_into_py(py))
		{
			Some(Ok(chained_jointbuilder)) => {
				slf.into_super().update_all(py)?;
				Ok(Some(chained_jointbuilder))
			}
			Some(Err(err)) => Err(err),
			None => Ok(None),
		}
	}

	fn yank_root(&self, py: Python<'_>) -> PyResult<Py<PyLinkBuilderChain>> {
		// TODO: Is clone here ok? // FIXME: UNWRAP
		self.inner.clone().yank_root().unwrap().try_into_py(py)
	}

	fn to_robot(slf: Py<Self>, name: String, py: Python<'_>) -> PyResult<Py<PyRobot>> {
		PyRobot::create(name, slf, py)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		Ok(format!(
			"KinematicTree(root_link = {}, ...)",
			self.get_root_link(py).__repr__(py)?
		))
	}
}

impl From<PyKinematicTree> for KinematicTree {
	fn from(value: PyKinematicTree) -> Self {
		value.inner
	}
}
