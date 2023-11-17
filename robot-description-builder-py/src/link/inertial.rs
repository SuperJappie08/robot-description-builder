use pyo3::{intern, prelude::*};
use robot_description_builder::link_data::Inertial;

use crate::transform::PyTransform;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "inertial")?;

	module.add_class::<PyInertial>()?;

	// parent_module.add_submodule(module)?;
	Ok(())
}

#[derive(Debug, PartialEq, Clone, Default)]
#[pyclass(
	name = "Inertial",
	module = "robot_description_builder.link",
	frozen,
	get_all
)] // Is set_all ok, since we would need to send the data back to the tree which is weird, immutability is also an option, set_all)]
pub struct PyInertial {
	pub transform: Option<PyTransform>,
	pub mass: f32,
	pub ixx: f32, // Not the nicesest way of doing this.
	pub ixy: f32,
	pub ixz: f32,
	pub iyy: f32,
	pub iyz: f32,
	pub izz: f32,
}

#[allow(clippy::too_many_arguments)]
#[pymethods]
impl PyInertial {
	#[new]
	#[pyo3(signature = (mass, ixx, iyy, izz, ixy=0., ixz=0., iyz=0., transform=None))]
	fn py_new(
		mass: f32,
		ixx: f32,
		iyy: f32,
		izz: f32,
		ixy: f32,
		ixz: f32,
		iyz: f32,
		transform: Option<PyTransform>,
	) -> Self {
		Self {
			transform,
			mass,
			ixx,
			ixy,
			ixz,
			iyy,
			iyz,
			izz,
		}
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut repr = format!(
			"{}(mass = {}, ixx = {}, ixy = {}, ixz = {}, iyy = {}, iyz = {}, izz = {}",
			class_name, self.mass, self.ixx, self.ixy, self.ixz, self.iyy, self.iyz, self.izz
		);

		if let Some(transform) = self.transform {
			repr.push_str(format!(", transform = {}", transform.__repr__(py)?).as_str());
		}

		repr.push(')');

		Ok(repr)
	}

	fn __bool__(&self) -> bool {
		// The transform is not checked since it is meanining less without an mass or an inertia
		self.mass.abs() != 0.
			|| self.ixx.abs() != 0.
			|| self.ixy.abs() != 0.
			|| self.ixz.abs() != 0.
			|| self.iyy.abs() != 0.
			|| self.iyz.abs() != 0.
			|| self.izz.abs() != 0.
	}
}

impl From<Inertial> for PyInertial {
	fn from(value: Inertial) -> Self {
		Self {
			transform: value.transform.map(Into::into),
			mass: value.mass,
			ixx: value.ixx,
			ixy: value.ixy,
			ixz: value.ixz,
			iyy: value.iyy,
			iyz: value.iyz,
			izz: value.izz,
		}
	}
}

impl From<PyInertial> for Inertial {
	fn from(value: PyInertial) -> Self {
		Self {
			transform: value.transform.map(Into::into),
			mass: value.mass,
			ixx: value.ixx,
			ixy: value.ixy,
			ixz: value.ixz,
			iyy: value.iyy,
			iyz: value.iyz,
			izz: value.izz,
		}
	}
}
