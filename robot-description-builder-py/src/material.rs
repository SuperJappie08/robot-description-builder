use pyo3::{intern, prelude::*};
use robot_description_builder::{
	material::{
		data::{MaterialData, MaterialDataReference},
		Material, MaterialDescriptor,
	},
	prelude::GroupIDChanger,
};

use crate::{
	identifier::GroupIDError,
	impl_into_py_callback,
	utils::{PyReadWriteable, TryIntoRefPyAny},
};

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

#[derive(Debug, Clone, FromPyObject)]
enum PyMaterialData {
	#[pyo3(annotation = "Color")]
	Color(f32, f32, f32, f32),
	#[pyo3(annotation = "TexturePath")]
	TexturePath { path: String },
}

// impl IntoPy<PyObject> for PyMaterialData {
// 	fn into_py(self, py: Python<'_>) -> PyObject {
// 		// Can unwrap, the things should be there
// 		let module = py
// 			.import(intern!(py, "robot_description_builder.material"))
// 			.unwrap();

// 		match self {
// 			PyMaterialData::Color(red, green, blue, alpha) => {
// 				let py_color = module.getattr(intern!(py, "Color")).unwrap();
// 				py_color
// 					.call_method1(intern!(py, "__new__"), (py_color, red, green, blue, alpha))
// 					.unwrap()
// 					.to_object(py)
// 			}
// 			PyMaterialData::TexturePath { path } => {
// 				let py_texture_path = module.getattr(intern!(py, "TexturePath")).unwrap();
// 				py_texture_path
// 					.call_method1(intern!(py, "__new__"), (py_texture_path, path))
// 					.unwrap()
// 					.to_object(py)
// 			}
// 		}
// 	}
// }

impl TryIntoRefPyAny for PyMaterialData {
	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<&PyAny> {
		let module = py.import(intern!(py, "robot_description_builder.material"))?;

		match self {
			PyMaterialData::Color(red, green, blue, alpha) => {
				let py_color = module.getattr(intern!(py, "Color"))?;
				py_color.call_method1(intern!(py, "__new__"), (py_color, red, green, blue, alpha))
			}
			PyMaterialData::TexturePath { path } => {
				let py_texture_path = module.getattr(intern!(py, "TexturePath"))?;
				py_texture_path.call_method1(intern!(py, "__new__"), (py_texture_path, path))
			}
		}
	}
}

// impl IntoPy<PyObject> for PyMaterialData {
// 	fn into_py(self, py: Python<'_>) -> PyObject {
// 		// It should not be able to panic??
// 		self.try_into_py(py).unwrap()
// 	}
// }

impl_into_py_callback!(PyMaterialData);

impl From<MaterialData> for PyMaterialData {
	fn from(value: MaterialData) -> Self {
		match value {
			MaterialData::Color(r, g, b, a) => Self::Color(r, g, b, a),
			MaterialData::Texture(path) => Self::TexturePath { path },
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
#[pyclass(
	name = "MaterialDescriptor",
	module = "robot_description_builder.material"
)]
pub struct PyMaterialDescriptor(MaterialDescriptor);

#[pymethods]
impl PyMaterialDescriptor {
	#[new]
	#[pyo3(signature=(data, name=None, /))]
	fn py_new(data: PyMaterialData, name: Option<String>) -> Self {
		let mut material_description = match data {
			PyMaterialData::Color(red, green, blue, alpha) => {
				MaterialDescriptor::new_color(red, green, blue, alpha)
			}
			PyMaterialData::TexturePath { path } => MaterialDescriptor::new_texture(path),
		};

		if let Some(name) = name {
			material_description = material_description.named(name);
		}

		Self(material_description)
	}

	#[getter]
	fn get_name(&self) -> Option<String> {
		self.0.name().cloned()
	}

	#[getter]
	fn get_data(&self) -> PyResult<PyMaterialData> {
		Ok(self.0.data().clone().into())
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
		data += self
			.get_data()?
			.try_into_py_ref(py)?
			.repr()?
			.extract::<&str>()?;

		Ok(format!("{class_name}({data})"))
	}

	fn change_group_id(&mut self, new_group_id: String, _py: Python<'_>) -> PyResult<()> {
		self.0
			.change_group_id(new_group_id)
			.map_err(GroupIDError::from)
	}

	fn apply_group_id(&mut self, _py: Python<'_>) {
		self.0.apply_group_id()
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
	fn get_data(&self) -> PyResult<PyMaterialData> {
		match self.0.material_data() {
			MaterialDataReference::Direct(data) => Ok(data.clone().into()),
			MaterialDataReference::Global(arc_data) => Ok(arc_data.py_read()?.clone().into()),
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
			MaterialDataReference::Direct(data) => repr_material_data(data),
			MaterialDataReference::Global(arc_data) => repr_material_data(&*arc_data.py_read()?),
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
