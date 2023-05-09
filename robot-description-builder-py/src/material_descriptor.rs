use itertools::Itertools;
use pyo3::{
	prelude::*,
	types::{PyDict, PyTuple},
};
use robot_description_builder::material::{data::MaterialData, MaterialDescriptor};

#[derive(Debug, Clone)]
#[pyclass(name = "MaterialDescriptor")]
pub struct PyMaterialDescriptor {
	inner: MaterialDescriptor,
}

#[pymethods]
impl PyMaterialDescriptor {
	/// TODO: Fix signature so name migth be picked up automagically? or not
	#[new]
	#[pyo3(signature = (*py_args, name=None, **py_kwargs))]
	fn new(
		py_args: &PyTuple,
		name: Option<String>,
		py_kwargs: Option<&PyDict>,
	) -> PyResult<PyMaterialDescriptor> {
		// TODO: Take String and do path
		// TODO: Take Color and do Colours.
		if (py_args.is_empty() || py_args.is_none())
			&& !py_kwargs.map_or(false, |dict| {
				dict.keys()
					.iter()
					.map(|key| {
						// if key.is_instance_of::<PyString>()? {
						if let Ok(key) = key.extract::<String>() {
							// FIXME: Maybe do only lowercase? Check conventions
							// If conventions turn out to be resiliant and take CrAZyCasE than update builder get kwargs step
							let key = key.to_lowercase();
							// TODO: Maybe add others as well
							["path", "color", "colour"].contains(&key.as_str())
						} else {
							false
						}
					})
					.count() > 0
			}) {
			return Err(pyo3::exceptions::PyValueError::new_err(
				"None value was given for MaterialDescriptor construction",
			));
		}

		let mut material_builder: MaterialDescriptor = if py_args.is_empty() || py_args.is_none() {
			// If we got here py_kwargs is not empty
			let kwargs = py_kwargs.unwrap();

			if let Some(Ok(path)) = kwargs.get_item("path").map(PyAny::extract::<String>) {
				MaterialDescriptor::new_texture(path)
			} else if let Some(Ok((red, green, blue, alpha))) = kwargs
				.get_item("rgba")
				.map(PyAny::extract::<(f32, f32, f32, f32)>)
			{
				MaterialDescriptor::new_color(red, green, blue, alpha)
			} else if let Some(Ok((red, green, blue))) = kwargs
				.get_item("rgb")
				.map(PyAny::extract::<(f32, f32, f32)>)
			{
				MaterialDescriptor::new_color(red, green, blue, 1.)
			} else {
				todo!()
			}
		} else {
			let args = py_args.as_slice();

			// Should always be some
			if let Ok(path) = args.first().unwrap().extract::<String>() {
				MaterialDescriptor::new_texture(path)
			} else if let Ok((red, green, blue, alpha)) =
				args.first().unwrap().extract::<(f32, f32, f32, f32)>()
			{
				MaterialDescriptor::new_color(red, green, blue, alpha)
			} else if let Ok((red, green, blue)) =
				args.first().unwrap().extract::<(f32, f32, f32)>()
			{
				MaterialDescriptor::new_color(red, green, blue, 1.)
			} else if let Some((Ok(red), Ok(green), Ok(blue), Ok(alpha))) = args
				.iter()
				.next_tuple::<(_, _, _, _)>()
				.map(|(red, green, blue, alpha)| {
					(
						red.extract::<f32>(),
						green.extract::<f32>(),
						blue.extract::<f32>(),
						alpha.extract::<f32>(),
					)
				}) {
				MaterialDescriptor::new_color(red, green, blue, alpha)
			} else if let Some((Ok(red), Ok(green), Ok(blue))) = args
				.iter()
				.next_tuple::<(_, _, _)>()
				.map(|(red, green, blue)| {
					(
						red.extract::<f32>(),
						green.extract::<f32>(),
						blue.extract::<f32>(),
					)
				}) {
				MaterialDescriptor::new_color(red, green, blue, 1.)
			} else {
				// TODO: Improve error
				return Err(pyo3::exceptions::PyValueError::new_err(
					"Invalid data was given for material construction",
				));
			}
		};

		if let Some(name) = name {
			material_builder = material_builder.named(name);
		}

		Ok(Self {
			inner: material_builder,
		})
	}

	pub fn __repr__(&self) -> String {
		let mut repr: String = match self.inner.data() {
			MaterialData::Color(red, green, blue, alpha) => format!(
				"MaterialDescriptor(rgba = ({}, {}, {}, {})",
				red, green, blue, alpha
			),
			MaterialData::Texture(path) => format!("MaterialDescriptor(path = \"{}\"", path),
		};

		if let Some(name) = self.inner.name() {
			repr.push_str(format!(", name=\"{}\"", name).as_str());
		}

		repr.push(')');
		repr
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.inner.name().map(String::clone)
	}

	#[setter]
	fn set_name(&mut self, name: String) {
		// This is a bit funky, but it needs to be GIL approved.
		self.inner = self.inner.clone().named(name);
	}

	#[getter]
	fn get_data(&self) -> Py<PyAny> {
		Python::with_gil(|py| match self.inner.data() {
			MaterialData::Color(red, green, blue, alpha) => {
				(*red, *green, *blue, *alpha).into_py(py)
			}
			MaterialData::Texture(path) => path.into_py(py),
		})
	}

	// #[setter]
	// /// TODO: DECIDE IF THIS SHOULD BE ALLOWED, since it normally is not
	// fn set_data(&mut self, value: &PyAny) {
	// 	if let Ok(path) = value.extract::<String>() {
	// 		self.inner = MaterialDescriptor {
	// 			data: MaterialData::Texture(path)
	// 			..self.inner.clone()
	// 		}
	// 	}
	// }
}

impl From<MaterialDescriptor> for PyMaterialDescriptor {
	fn from(value: MaterialDescriptor) -> Self {
		Self { inner: value }
	}
}

impl From<PyMaterialDescriptor> for MaterialDescriptor {
	fn from(value: PyMaterialDescriptor) -> Self {
		value.inner
	}
}
