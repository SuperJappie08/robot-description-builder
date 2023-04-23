mod geometry;
mod joint;
mod material;
mod material_builder;
mod visual;
mod visual_builder;

use joint::*;
use material::*;
use material_builder::PyMaterialBuilder;
use visual::*;

use std::sync::{Arc, RwLock};

use itertools::Itertools;
use pyo3::prelude::*;

use rdf_builder_rs::{
	linkbuilding::LinkBuilder, JointBuilder, KinematicInterface, KinematicTree, Link, Robot,
};

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
		self.inner.get_name().clone()
	}
}

impl From<Robot> for PyRobot {
	fn from(value: Robot) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug, Clone)]
#[pyclass(name = "KinematicTree")]
struct PyKinematicTree {
	inner: KinematicTree,
}

#[pymethods]
impl PyKinematicTree {
	#[getter]
	fn root_link(&self) -> PyLink {
		self.inner.get_root_link().into()
	}

	#[getter]
	fn newest_link(&self) -> PyLink {
		self.inner.get_newest_link().into()
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
		self.inner.get_link(&name).map(Into::into)
	}

	fn get_joint(&self, name: String) -> Option<PyJoint> {
		self.inner.get_joint(&name).map(Into::into)
	}

	pub fn __repr__(&self) -> String {
		format!(
			"KinematicTree(root_link = {}, ...)",
			self.root_link().__repr__()
		)
	}
}

impl From<KinematicTree> for PyKinematicTree {
	fn from(value: KinematicTree) -> Self {
		Self { inner: value }
	}
}

impl From<PyKinematicTree> for KinematicTree {
	fn from(value: PyKinematicTree) -> Self {
		value.inner
	}
}

#[derive(Debug)]
#[pyclass(name = "Link")]
struct PyLink {
	inner: Arc<RwLock<Link>>,
}

#[pymethods]
impl PyLink {
	#[staticmethod]
	/// TODO: EXPAND
	fn new(
		name: String,
		visuals: Option<Vec<PyVisual>>,
		colliders: Option<Vec<PyVisual>>,
	) -> PyKinematicTree {
		// Link::new(name).into()
		let mut builder = LinkBuilder::new(name);
		// builder.get_visuals_mut().append(
		// 	&mut visuals
		// 		.unwrap_or_default()
		// 		.iter()
		// 		.map(|visual| visual.clone().into())
		// 		.collect(),
		// );
		if visuals.is_some() {
			panic!("Not IMPLEMENTED YET")
		}

		if colliders.is_some() {
			panic!("Not IMPLEMENTED YET")
		}

		builder.build_tree().into()
	}

	#[getter]
	fn name(&self) -> String {
		self.inner.try_read().unwrap().get_name().clone() // TODO: Figure out if unwrap is Ok here?
	}

	///TODO: Joint Type Selection
	fn try_attach_child(&self, tree: PyKinematicTree, joint_builder: PyJointBuilder) {
		// FIXME: Need to do somethign with error
		self.inner
			.try_write()
			.unwrap() // TODO: Figure out if unwrap is Ok here?
			.try_attach_child_old(
				Into::<KinematicTree>::into(tree).into(),
				Into::<JointBuilder>::into(joint_builder),
			)
			.unwrap() // TODO: Figure out if unwrap is Ok here?
	}

	fn __repr__(&self) -> String {
		let link = self.inner.read().unwrap(); // FIXME: Unwrap ok?
		let mut repr = format!("Link('{}'", link.get_name());

		{
			let visuals = link.get_visuals();
			if !visuals.is_empty() {
				repr += ", visuals = [";
				// repr += &visuals.iter().map(|visual| PyVisual::from(visual.clone()).__repr__()).collect::<Vec<String>>().join(", ");
				repr += visuals
					.iter()
					.map(|visual| PyVisual::from(visual.clone()).__repr__())
					.join(", ")
					.as_str();
				repr += "]";
			}
		}
		// TODO: EXPAND

		repr += ", ...)";
		repr
	}
}

impl From<Arc<RwLock<Link>>> for PyLink {
	fn from(value: Arc<RwLock<Link>>) -> Self {
		Self { inner: value }
	}
}

#[derive(Debug)]
#[pyclass(name = "LinkBuilder")]
struct PyLinkBuilder {
	inner: LinkBuilder,
}

#[pymethods]
impl PyLinkBuilder {
	#[new]
	fn new(name: String) -> Self {
		LinkBuilder::new(name).into()
	}

	/// Maybe direct construction
	fn build(&self) -> PyKinematicTree {
		// FIXME: NOT OK
		self.inner.clone().build_tree().into()
	}
}

impl From<LinkBuilder> for PyLinkBuilder {
	fn from(value: LinkBuilder) -> Self {
		Self { inner: value }
	}
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
	Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "rdf_builder_py")]
fn rdf_builder_py(py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

	// INTERRESTING IDEA, DICT Constructors...
	m.add_class::<PyRobot>()?;
	m.add_class::<PyKinematicTree>()?;

	m.add_class::<PyLink>()?;
	m.add_class::<PyLinkBuilder>()?;

	m.add_class::<PyMaterial>()?;
	m.add_class::<PyMaterialBuilder>()?;

	m.add_class::<PyVisual>()?;

	let geometry = PyModule::new(py, "geometry")?;
	// Inits the python geometry module with the the init_module function from the rust geometry module
	// Lots of packages do it like this and I don't know why, they do not do it like in the [PyO3 book](https://pyo3.rs/main/module#python-submodules)
	geometry::init_module(geometry)?;
	m.add_submodule(geometry)?;

	m.add_class::<PyJoint>()?;
	m.add_class::<PyJointBuilder>()?;
	m.add_class::<PyJointType>()?;

	Ok(())
}
