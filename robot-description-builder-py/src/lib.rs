mod joint;
mod link;
mod material;
mod material_descriptor;
mod transform;
mod utils;

use std::sync::Weak;

use itertools::Itertools;
use joint::*;
use link::*;
use material_descriptor::PyMaterialDescriptor;

use pyo3::{intern, prelude::*};

use robot_description_builder::{KinematicInterface, KinematicTree, Robot};

#[derive(Debug)]
#[pyclass(name = "Robot")]
/// Testing
struct PyRobot {
	inner: Robot,
}

#[pymethods]
impl PyRobot {
	#[getter]
	fn name(&self) -> String {
		self.inner.name().clone()
	}
}

impl From<Robot> for PyRobot {
	fn from(value: Robot) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "KinematicTree", weakref)]
struct PyKinematicTree {
	inner: KinematicTree,
	/// Python weakref to self
	me: PyObject,
}

impl PyKinematicTree {
	pub(crate) fn create(tree: KinematicTree) -> PyResult<Py<PyKinematicTree>> {
		Python::with_gil(|py| {
			let weakref = py.import(intern!(py, "weakref")).unwrap();
			let tree: Py<PyKinematicTree> = PyKinematicTree {
				inner: tree,
				me: py.None(),
			}
			.into_py(py)
			.extract(py)?;

			weakref
				.getattr("proxy")?
				.call1((&tree,))?
				.to_object(py)
				.clone_into(&mut tree.borrow_mut(py).me);

			Ok(tree)
		})
	}
}

#[pymethods]
impl PyKinematicTree {
	#[getter]
	fn root_link(&self) -> PyLink {
		(self.inner.get_root_link(), self.me.clone()).into()
	}

	#[getter]
	fn newest_link(&self) -> PyLink {
		(self.inner.get_newest_link(), self.me.clone()).into()
	}

	// #[getter] // get_links
	// fn links(&self) -> HashMap<String, std::sync::Weak<RwLock<Link>>> {
	//     self.inner.get_links().into_inner().unwrap().clone()
	// }

	// fn get_joints(&self) -> Arc<RwLock<std::collections::HashMap<String, std::sync::Weak<RwLock<rdf_builder_rs::Joint>>>>> {
	//     todo!()
	// }

	// fn get_materials(&self) -> Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<rdf_builder_rs::Material>>>>> {
	//     todo!()
	// }

	// fn get_transmissions(&self) -> Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<rdf_builder_rs::Transmission>>>>> {
	//     todo!()
	// }

	fn get_link(&self, name: String) -> Option<PyLink> {
		self.inner
			.get_link(&name)
			.map(|link| (link, self.me.clone()).into())
	}

	fn get_joint(&self, name: String) -> Option<PyJoint> {
		self.inner
			.get_joint(&name)
			.map(|joint| (joint, self.me.clone()).into())
	}

	fn yank_link(&self, name: String) -> Option<PyLinkBuilder> {
		self.inner
			.yank_link(&name)
			.map(|link_builder| link_builder.builder().clone().into())
	}

	pub fn __repr__(&self) -> PyResult<String> {
		Ok(format!(
			"KinematicTree(root_link = {}, ...)",
			self.root_link().__repr__()?
		))
	}

	/// FOR DEBUG
	fn print_refs(&self) -> String {
		self.inner
			.get_links()
			.read()
			.unwrap()
			.iter()
			.sorted_by_key(|(k, _)| (*k).clone())
			.map(|(name, link)| {
				format!(
					"{}: Strong {}, Weak: {}",
					name,
					Weak::strong_count(link),
					Weak::strong_count(link)
				)
			})
			.join("\n")
	}
}

// impl From<KinematicTree> for PyResult<Py<PyKinematicTree>> {
// 	fn from(value: KinematicTree) -> PyResult<Py<PyKinematicTree>> {
// 		Python::with_gil(|py| {
// 			let weakref = py.import(intern!(py, "weakref")).unwrap();
// 			let mut tree = PyKinematicTree {
// 				inner: value,
// 				me: py.None(),
// 			};
// 			// tree.me = weakref
// 			// 	.getattr("proxy")
// 			// 	.unwrap()
// 			// 	.call1((tree,))
// 			// 	.unwrap()
// 			// 	.into();
// 			weakref
// 				.getattr("proxy")
// 				.unwrap()
// 				.call1((tree.clone(),))
// 				.unwrap()
// 				.to_object(py)
// 				.clone_into(&mut tree.me);

// 			tree
// 		})
// 	}
// }

impl From<PyKinematicTree> for KinematicTree {
	fn from(value: PyKinematicTree) -> Self {
		value.inner
	}
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_internal")]
fn rdf_builder_py(py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	// INTERRESTING IDEA, DICT Constructors...
	m.add_class::<PyRobot>()?;
	m.add_class::<PyKinematicTree>()?;

	// PyO3 + Maturin can only generate a python module, not a convienent package
	// As a result it is easier to export everything flat
	link::init_module(py, m)?;

	transform::init_module(py, m)?;

	// m.add_class::<PyMaterial>()?;
	m.add_class::<PyMaterialDescriptor>()?;

	m.add_class::<PyJoint>()?;
	m.add_class::<PyJointBuilder>()?;
	m.add_class::<PyJointType>()?;

	Ok(())
}
