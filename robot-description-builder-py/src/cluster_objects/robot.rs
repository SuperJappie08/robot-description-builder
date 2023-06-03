use pyo3::{intern, prelude::*};
use robot_description_builder::Robot;

use crate::utils;

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
	me: PyObject,
}

impl PyRobot {
	pub(super) fn create(
		name: String,
		tree: Py<PyKinematicTree>,
		py: Python<'_>,
	) -> PyResult<Py<Self>> {
		let weakref = py.import(intern!(py, "weakref"))?;

		let inner = tree.borrow(py).clone().into_inner().to_robot(name);
		// Only drops the reference.
		drop(tree);

		let base = PyKinematicBase::new(py, &inner, &py.None())?;

		let robot = utils::init_pyclass_initializer(
			PyClassInitializer::new(
				Self {
					inner,
					me: py.None(),
				},
				base.into(),
			),
			py,
		)?;

		weakref
			.getattr("proxy")?
			.call1((&robot,))?
			.to_object(py)
			.clone_into(&mut robot.borrow_mut(py).me);

		let robot_weak = robot.borrow(py).me.clone();

		{
			let mut base = robot.borrow_mut(py).into_super();
			base.implementor = robot_weak;

			base.update_all(py)?;
		}

		Ok(robot)
	}
}

#[pymethods]
impl PyRobot {
	#[getter]
	fn name(&self) -> String {
		self.inner.name().clone()
	}
}

// impl From<Robot> for PyRobot {
// 	fn from(value: Robot) -> Self {
// 		Self { inner: value }
// 	}
// }
