use pyo3::{exceptions::PyTypeError, intern, prelude::*};
use robot_description_builder::material::{
	data::{MaterialData, MaterialDataReferenceWrapper},
	Material, MaterialDescriptor,
};

use crate::utils::PyReadWriteable;

pub(super) fn init_module(_py: Python<'_>, module: &PyModule) -> PyResult<()> {
	module.add_class::<PyMaterial>()?;
	module.add_class::<PyMaterialDescriptor>()?;
	Ok(())
}

fn repr_material_data(data: &MaterialData) -> String {
	match data {
		MaterialData::Color(r, g, b, a) => format!("rgba=({r}, {g}, {b}, {a})"),
		MaterialData::Texture(path) => format!("filname='{path}'"),
	}
}

fn material_data2py_object<'py>(data: &MaterialData, py: Python<'py>) -> PyResult<&'py PyAny> {
	let material_module = py.import(intern!(py, "robot_description_builder.material"))?;

	match data {
		MaterialData::Color(r, g, b, a) => {
			let color = material_module.getattr(intern!(py, "Color"))?;
			color.call_method1(intern!(py, "__new__"), (color, *r, *g, *b, *a))
		}
		MaterialData::Texture(filename) => {
			let texture_path = material_module.getattr(intern!(py, "TexturePath"))?;
			texture_path.call1((filename,))
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
#[pyclass(
	name = "MaterialDescriptor",
	module = "robot_description_builder.material",
	text_signature = "(data: typing.Union[Color, TexturePath], name = typing.Optional[str], /)"
)]
pub struct PyMaterialDescriptor(MaterialDescriptor);

#[pymethods]
impl PyMaterialDescriptor {
	#[new]
	#[pyo3(signature=(data, name=None, /))]
	fn py_new(data: PyObject, name: Option<String>, py: Python<'_>) -> PyResult<Self> {
		let material_mod = py.import(intern!(py, "robot_description_builder.material"))?;
		let py_color = material_mod.getattr(intern!(py, "Color"))?;
		let py_texture_path = material_mod.getattr(intern!(py, "TexturePath"))?;

		let mut material_description = if data.as_ref(py).is_instance(py_color)? {
			Ok(MaterialDescriptor::new_color(
				data.getattr(py, intern!(py, "r"))?.extract(py)?,
				data.getattr(py, intern!(py, "g"))?.extract(py)?,
				data.getattr(py, intern!(py, "b"))?.extract(py)?,
				data.getattr(py, intern!(py, "a"))?.extract(py)?,
			))
		} else if data.as_ref(py).is_instance(py_texture_path)? {
			/*FOR NewType} else if data.as_ref(py).is_instance_of::<PyString>()? {
			// This excepts all string types, but there is no better way,
			// without changing Python's `TexturePath` to a class which is only possible starting from Python3.10
			Ok(MaterialDescriptor::new_texture(data.extract::<String>(py)?)) */
			Ok(MaterialDescriptor::new_texture(
				data.getattr(py, intern!(py, "path"))?
					.extract::<String>(py)?,
			))
		} else {
			Err(PyTypeError::new_err(
				"'data' is not a Color or a TexturePath",
			))
		}?;

		if let Some(name) = name {
			material_description = material_description.named(name);
		}

		Ok(Self(material_description))
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().cloned()
	}

	#[getter]
	fn get_data<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
		material_data2py_object(self.0.data(), py)
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let mut data = match self.0.name() {
			Some(name) => format!("name='{}', ", name),
			None => String::new(),
		};

		data += "data=";
		data += self.get_data(py)?.repr()?.extract::<&str>()?;

		Ok(format!("{class_name}({data})"))
	}
}

impl From<PyMaterialDescriptor> for MaterialDescriptor {
	fn from(value: PyMaterialDescriptor) -> Self {
		value.0
	}
}

impl From<MaterialDescriptor> for PyMaterialDescriptor {
	fn from(value: MaterialDescriptor) -> Self {
		Self(value)
	}
}

#[derive(Debug, PartialEq, Clone)]
#[pyclass(
	name = "Material",
	module = "robot_description_builder.material",
	weakref,
	frozen
)]
pub struct PyMaterial(Material);

#[pymethods]
impl PyMaterial {
	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().cloned()
	}

	#[getter]
	fn get_data<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
		match self.0.material_data() {
			MaterialDataReferenceWrapper::Direct(data) => material_data2py_object(data, py),
			MaterialDataReferenceWrapper::Global(arc_data) => {
				material_data2py_object(&arc_data.py_read()?.clone(), py)
			}
		}
	}

	fn describe(&self) -> PyMaterialDescriptor {
		self.0.describe().into()
	}

	pub fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
		let class_name = py.get_type::<Self>().getattr(intern!(py, "__qualname__"))?;

		let mut data = match self.0.name() {
			Some(name) => format!("name='{name}', "),
			None => String::new(),
		};

		data += "data=";
		data += match self.0.material_data() {
			MaterialDataReferenceWrapper::Direct(data) => repr_material_data(data),
			MaterialDataReferenceWrapper::Global(arc_data) => {
				repr_material_data(&*arc_data.py_read()?)
			}
		}
		.as_str();

		Ok(format!("{class_name}({data})"))
	}
}

impl From<Material> for PyMaterial {
	fn from(value: Material) -> Self {
		Self(value)
	}
}

impl From<PyMaterial> for Material {
	fn from(value: PyMaterial) -> Self {
		value.0
	}
}
