use pyo3::{basic::CompareOp, intern, prelude::*};
use robot_description_builder::Transform;

const NONE_STR: &str = "None";

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyTransform>()?;
	Ok(())
}

#[pyclass(
	name = "Transform",
	get_all,
	set_all,
	module = "robot_description_builder"
)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct PyTransform {
	x: Option<f32>,
	y: Option<f32>,
	z: Option<f32>,
	roll: Option<f32>,
	pitch: Option<f32>,
	yaw: Option<f32>,
}

#[pymethods]
impl PyTransform {
	#[new]
	fn py_new(
		x: Option<f32>,
		y: Option<f32>,
		z: Option<f32>,
		roll: Option<f32>,
		pitch: Option<f32>,
		yaw: Option<f32>,
	) -> Self {
		Self {
			x,
			y,
			z,
			roll,
			pitch,
			yaw,
		}
	}

	// pub fn __repr__(&self) -> PyResult<String> {
	// 	// This is a valid repr since it would recreate the object if the thing is imported to root
	// 	let translation: Option<String> = match self.is_some_translation() {
	// 		true => Some(format!(
	// 			"x={}, y={}, z={}",
	// 			self.x.map_or(NONE_STR.into(), |x| x.to_string()),
	// 			self.y.map_or(NONE_STR.into(), |y| y.to_string()),
	// 			self.z.map_or(NONE_STR.into(), |z| z.to_string())
	// 		)),
	// 		false => None,
	// 	};

	// 	let rotation: Option<String> = match self.is_some_rotation() {
	// 		true => Some(format!(
	// 			"roll={}, pitch={}, yaw={}",
	// 			self.roll.map_or(NONE_STR.into(), |r| r.to_string()),
	// 			self.pitch.map_or(NONE_STR.into(), |p| p.to_string()),
	// 			self.yaw.map_or(NONE_STR.into(), |y| y.to_string())
	// 		)),
	// 		false => None,
	// 	};

	// 	let total = match (translation, rotation) {
	// 		(Some(translation), Some(rotation)) => format!("{}, {}", translation, rotation),
	// 		(None, Some(rotation)) => rotation,
	// 		(Some(translation), None) => translation,
	// 		(None, None) => String::new(),
	// 	};

	// 	Ok(format!("Transform({})", total))
	// }

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		// let module_name = py
		// 	.get_type()
		// 	.getattr(intern!(py, "__module__"))?
		// 	.extract::<&str>()?;
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let translation: Option<String> = match self.is_some_translation() {
			true => Some(format!(
				"x={}, y={}, z={}",
				self.x.map_or(NONE_STR.into(), |x| x.to_string()),
				self.y.map_or(NONE_STR.into(), |y| y.to_string()),
				self.z.map_or(NONE_STR.into(), |z| z.to_string())
			)),
			false => None,
		};

		let rotation: Option<String> = match self.is_some_rotation() {
			true => Some(format!(
				"roll={}, pitch={}, yaw={}",
				self.roll.map_or(NONE_STR.into(), |r| r.to_string()),
				self.pitch.map_or(NONE_STR.into(), |p| p.to_string()),
				self.yaw.map_or(NONE_STR.into(), |y| y.to_string())
			)),
			false => None,
		};

		let total = match (translation, rotation) {
			(Some(translation), Some(rotation)) => format!("{}, {}", translation, rotation),
			(None, Some(rotation)) => rotation,
			(Some(translation), None) => translation,
			(None, None) => String::new(),
		};

		// Ok(format!("{}.{}({})", module_name, class_name, total))
		Ok(format!("{}({})", class_name, total))
	}

	fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
		match op {
			CompareOp::Eq => (self == other).into_py(py),
			CompareOp::Ne => (self != other).into_py(py),
			_ => py.NotImplemented(),
		}
	}

	fn __bool__(&self) -> bool {
		self.x.map(|x| x.abs() != 0.).unwrap_or_default()
			|| self.y.map(|y| y.abs() != 0.).unwrap_or_default()
			|| self.z.map(|z| z.abs() != 0.).unwrap_or_default()
			|| self.roll.map(|r| r.abs() != 0.).unwrap_or_default()
			|| self.pitch.map(|p| p.abs() != 0.).unwrap_or_default()
			|| self.yaw.map(|y| y.abs() != 0.).unwrap_or_default()
	}
}

impl PyTransform {
	fn is_some_translation(&self) -> bool {
		self.x.is_some() || self.y.is_some() || self.z.is_some()
	}

	fn is_some_rotation(&self) -> bool {
		self.roll.is_some() || self.pitch.is_some() || self.yaw.is_some()
	}
}

impl From<PyTransform> for Transform {
	fn from(value: PyTransform) -> Self {
		let translation: Option<(f32, f32, f32)> = match (value.x, value.y, value.z) {
			(None, None, None) => None,
			(x, y, z) => Some((
				x.unwrap_or_default(),
				y.unwrap_or_default(),
				z.unwrap_or_default(),
			)),
		};

		let rotation: Option<(f32, f32, f32)> = match (value.roll, value.pitch, value.yaw) {
			(None, None, None) => None,
			(r, p, y) => Some((
				r.unwrap_or_default(),
				p.unwrap_or_default(),
				y.unwrap_or_default(),
			)),
		};

		Self {
			translation,
			rotation,
		}
	}
}

impl From<Transform> for PyTransform {
	fn from(value: Transform) -> Self {
		Self {
			x: value.translation.map(|tranlation| tranlation.0),
			y: value.translation.map(|tranlation| tranlation.1),
			z: value.translation.map(|tranlation| tranlation.2),
			roll: value.rotation.map(|rotation| rotation.0),
			pitch: value.rotation.map(|rotation| rotation.1),
			yaw: value.rotation.map(|rotation| rotation.2),
		}
	}
}
