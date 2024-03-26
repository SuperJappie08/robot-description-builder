use pyo3::{prelude::*, sync::GILOnceCell, types::PyType};
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

// Sometimes it needed to be defined directly in macro
const SUBMODULE_PATH: &'static str = "robot_description_builder.material";

static PY_COLOR_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static PY_TEXTUREPATH_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();

fn repr_material_data(data: &MaterialData) -> String {
	match data {
		MaterialData::Color(r, g, b, a) => format!("rgba=({r}, {g}, {b}, {a})"),
		MaterialData::Texture(path) => format!("filename='{path}'"),
	}
}

#[derive(Debug, Clone, FromPyObject)]
enum PyMaterialData {
	#[pyo3(annotation = "Color")]
	Color(f32, f32, f32, f32),
	#[pyo3(annotation = "TexturePath")]
	TexturePath { path: String },
}

// impl IntoPy<PyResult<Py<PyAny>>> for PyMaterialData {
// 	fn into_py(self, py: Python<'_>) -> PyResult<Py<PyAny>> {
// 		match self {
// 			PyMaterialData::Color(red, green, blue, alpha) => {
// 				let py_color = PY_COLOR_TYPE.get_or_try_init(py, || -> PyResult<Py<PyType>> {
// 					Ok(py
// 						.import_bound(SUBMODULE_PATH)?
// 						.getattr("Color")?
// 						.downcast_into_exact()?
// 						.unbind())
// 				})?;
// 				py_color.call1(py, (red, green, blue, alpha))
// 			}
// 			PyMaterialData::TexturePath { path } => {
// 				let py_texture_path =
// 					PY_TEXTUREPATH_TYPE.get_or_try_init(py, || -> PyResult<Py<PyType>> {
// 						Ok(py
// 							.import_bound(SUBMODULE_PATH)?
// 							.getattr("TexturePath")?
// 							.downcast_into_exact()?
// 							.unbind())
// 					})?;
// 				py_texture_path.call1(py, (path,))
// 			}
// 		}
// 	}
// }

impl TryIntoRefPyAny for PyMaterialData {
	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<Bound<'_, PyAny>> {
		match self {
			PyMaterialData::Color(red, green, blue, alpha) => {
				let py_color = PY_COLOR_TYPE
					.get_or_try_init(py, || -> PyResult<Py<PyType>> {
						Ok(py
							.import_bound(SUBMODULE_PATH)?
							.getattr("Color")?
							.downcast_into_exact()?
							.unbind())
					})?
					.bind(py);
				py_color.call1((red, green, blue, alpha))
			}
			PyMaterialData::TexturePath { path } => {
				let py_texture_path = PY_TEXTUREPATH_TYPE
					.get_or_try_init(py, || -> PyResult<Py<PyType>> {
						Ok(py
							.import_bound(SUBMODULE_PATH)?
							.getattr("TexturePath")?
							.downcast_into_exact()?
							.unbind())
					})?
					.bind(py);
				py_texture_path.call1((path,))
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
		let class_name = py.get_type::<Self>().qualname()?;

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
		let class_name = py.get_type::<Self>().qualname()?;

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
