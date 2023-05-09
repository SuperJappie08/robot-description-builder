use pyo3::{intern, prelude::*};
use robot_description_builder::link_data::InertialData;

use crate::transform::PyTransform;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	// let module = PyModule::new(py, "inertial")?;

	module.add_class::<PyInertial>()?;

	// parent_module.add_submodule(module)?;
	Ok(())
}

/// TODO: Maybe easier to make a python version?
#[derive(Debug, PartialEq, Clone, Default)]
#[pyclass(
	name = "Inertial",
	module = "robot_description_builder.link",
	frozen,
	get_all
)] // Is set_all ok, since we would need to send the data back to the tree which is weird, immutability is also an option, set_all)]
pub struct PyInertial {
	pub origin: Option<PyTransform>,
	pub mass: f32,
	pub ixx: f32, // Not the nicesest way of doing this.
	pub ixy: f32,
	pub ixz: f32,
	pub iyy: f32,
	pub iyz: f32,
	pub izz: f32,
}

#[pymethods]
impl PyInertial {
	#[new]
	#[pyo3(signature = (mass, ixx, iyy, izz, ixy=0., ixz=0., iyz=0., origin=None))]
	fn py_new(
		mass: f32,
		ixx: f32,
		iyy: f32,
		izz: f32,
		ixy: f32,
		ixz: f32,
		iyz: f32,
		origin: Option<PyTransform>,
	) -> Self {
		Self {
			origin,
			mass,
			ixx,
			ixy,
			ixz,
			iyy,
			iyz,
			izz,
		}
	}

	/// TODO: Figure this out propperly
	pub fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
		// let module_name = slf
		// 	.get_type()
		// 	.getattr(intern!(slf.py(), "__module__"))?
		// 	.extract::<&str>()?;
		let class_name = slf
			.get_type()
			.getattr(intern!(slf.py(), "__qualname__"))?
			.extract::<&str>()?;

		let binding = slf.borrow();

		let mut repr = format!(
			// "{}.{}(mass = {}, ixx = {}, ixy = {}, ixz = {}, iyy = {}, iyz = {}, izz = {}",
			"{}(mass = {}, ixx = {}, ixy = {}, ixz = {}, iyy = {}, iyz = {}, izz = {}",
			// module_name,
			class_name,
			binding.mass,
			binding.ixx,
			binding.ixy,
			binding.ixz,
			binding.iyy,
			binding.iyz,
			binding.izz
		);

		if let Some(transform) = binding.origin {
			repr.push_str(
				", Whoops Something is confusion about repr", // format!(", origin = {}", transform.).as_str()
			);
		}

		repr.push(')');

		Ok(repr)
	}

	fn __bool__(&self) -> bool {
		// Origin is not checked since it is meanining less without an mass or an inertia
		self.mass.abs() != 0.
			|| self.ixx.abs() != 0.
			|| self.ixy.abs() != 0.
			|| self.ixz.abs() != 0.
			|| self.iyy.abs() != 0.
			|| self.iyz.abs() != 0.
			|| self.izz.abs() != 0.
	}
}

impl From<InertialData> for PyInertial {
	fn from(value: InertialData) -> Self {
		Self {
			origin: value.origin.map(Into::into),
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

impl From<PyInertial> for InertialData {
	fn from(value: PyInertial) -> Self {
		Self {
			origin: value.origin.map(Into::into),
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
