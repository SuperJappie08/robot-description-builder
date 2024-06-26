use pyo3::{prelude::*, types::PyWeakrefProxy};
use robot_description_builder::{KinematicInterface, Robot};

use crate::{link::PyLink, utils};

use super::{PyKinematicBase, PyKinematicTree};

#[derive(Debug)]
#[pyclass(
    name = "Robot",
    module="robot_description_builder.cluster_objects",
    weakref,
    extends = PyKinematicBase)]
pub struct PyRobot {
	inner: Robot,
	// Weakref to self
	me: Py<PyWeakrefProxy>,
}

impl PyRobot {
	pub(super) fn create(
		name: String,
		tree: Py<PyKinematicTree>,
		py: Python<'_>,
	) -> PyResult<Py<Self>> {
		let inner = tree.borrow(py).inner.clone().to_robot(name);
		// Only drops the reference.
		drop(tree);

		// Temporary make it a broken value, so we can overwrite it.
		let me = unsafe { py.None().downcast_bound_unchecked(py) }.clone();

		let base = PyKinematicBase::new(py, &inner, &me)?;

		let robot = utils::init_pyclass_initializer(
			PyClassInitializer::new(
				Self {
					inner,
					me: me.unbind(),
				},
				base.into(),
			),
			py,
		)?;

		robot.borrow_mut(py).me = PyWeakrefProxy::new_bound(robot.bind(py))?.unbind();

		let robot_weak = robot.borrow(py).me.bind(py).clone();

		{
			let mut base = robot.borrow_mut(py).into_super();
			base.implementor = robot_weak.unbind();

			base.update_all(py)?;
		}

		Ok(robot)
	}

	pub(crate) fn get_weak<'py>(&self, py: Python<'py>) -> Bound<'py, PyWeakrefProxy> {
		self.me.bind(py).clone()
	}

	pub fn as_robot(&self) -> &Robot {
		&self.inner
	}
}

#[pymethods]
impl PyRobot {
	#[getter]
	fn name(&self) -> String {
		self.inner.name().clone()
	}
	#[getter]
	fn get_root_link(&self, py: Python<'_>) -> PyLink {
		(self.inner.get_root_link(), self.get_weak(py)).into()
	}

	#[getter]
	fn get_newest_link(&self, py: Python<'_>) -> PyLink {
		(self.inner.get_newest_link(), self.get_weak(py)).into()
	}
}

// impl From<Robot> for PyRobot {
// 	fn from(value: Robot) -> Self {
// 		Self { inner: value }
// 	}
// }

impl From<PyRobot> for Robot {
	fn from(value: PyRobot) -> Self {
		value.inner
	}
}
