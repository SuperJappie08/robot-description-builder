use pyo3::{intern, prelude::*, PyClassInitializer};

use robot_description_builder::{KinematicInterface, KinematicTree};

use super::{robot::PyRobot, PyKinematicBase};

use crate::{
	joint::{PyJoint, PyJointBuilderChain},
	link::{PyLink, PyLinkBuilderChain},
	material::PyMaterial,
	transmission::PyTransmission,
	utils::{self, TryIntoPy},
};

#[derive(Debug, Clone)]
#[pyclass(
    name = "KinematicTree",
    module = "robot_description_builder.cluster_objects",
    weakref,
    extends = PyKinematicBase)]
pub struct PyKinematicTree {
	inner: KinematicTree,
	/// Python weakref to self.
	me: PyObject,
}

impl PyKinematicTree {
	pub(crate) fn create(tree: KinematicTree, py: Python<'_>) -> PyResult<Py<PyKinematicTree>> {
		let weakref = py.import(intern!(py, "weakref")).unwrap();

		let base = PyKinematicBase::new(py, &tree, &py.None())?;

		let tree: Py<PyKinematicTree> = utils::init_pyclass_initializer(
			PyClassInitializer::from((
				PyKinematicTree {
					inner: tree,
					me: py.None(),
				},
				base,
			)),
			py,
		)?;

		weakref
			.getattr(intern!(py, "proxy"))?
			.call1((&tree,))?
			.to_object(py)
			.clone_into(&mut tree.borrow_mut(py).me);

		let tree_weak = tree.borrow(py).me.clone();

		{
			let mut base = tree.borrow_mut(py).into_super();
			base.implementor = tree_weak;

			base.update_all(py)?;
		}

		Ok(tree)
	}

	pub(crate) fn get_weak(&self) -> PyObject {
		self.me.clone()
	}

	pub(crate) fn into_inner(self) -> KinematicTree {
		self.inner
	}
}

#[pymethods]
impl PyKinematicTree {
	#[getter]
	fn get_root_link(&self) -> PyLink {
		(self.inner.get_root_link(), self.get_weak()).into()
	}

	#[getter]
	fn get_newest_link(&self) -> PyLink {
		(self.inner.get_newest_link(), self.get_weak()).into()
	}

	fn get_link(&self, name: String) -> Option<PyLink> {
		self.inner
			.get_link(&name)
			.map(|link| (link, self.get_weak()).into())
	}

	fn get_joint(&self, name: String) -> Option<PyJoint> {
		self.inner
			.get_joint(&name)
			.map(|joint| (joint, self.get_weak()).into())
	}

	fn get_material(&self, name: String) -> Option<PyMaterial> {
		self.inner.get_material(&name).map(Into::into)
	}

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
			self.get_root_link().__repr__(py)?
		))
	}
}

impl From<PyKinematicTree> for KinematicTree {
	fn from(value: PyKinematicTree) -> Self {
		value.inner
	}
}
