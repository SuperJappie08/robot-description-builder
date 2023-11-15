use pyo3::{exceptions::PyTypeError, intern, prelude::*, types::PyDict};
use robot_description_builder::{prelude::GroupIDChanger, JointBuilder};

use crate::identifier::GroupIDError;

use super::{PyJointBuilderBase, PyJointType, PyLimit};

#[derive(Debug, Clone)]
#[pyclass(
	name = "JointBuilder",
	module = "robot_description_builder.joint",
    extends = PyJointBuilderBase,
	subclass
)]
pub struct PyJointBuilder;

impl PyJointBuilder {
	fn as_jointbuilder(mut slf: PyRefMut<'_, Self>) -> JointBuilder {
		if let Some(py_transform) = slf.as_ref().transform.clone() {
			slf.as_mut()
				.builder
				.set_transform_simple(Python::with_gil(|py| (*py_transform.borrow(py)).into()))
		}

		slf.as_ref().builder.clone()
	}
}

#[pymethods]
impl PyJointBuilder {
	#[new]
	#[pyo3(signature = (name, joint_type, **kwds))]
	fn py_new(
		name: String,
		joint_type: PyJointType,
		kwds: Option<&PyDict>,
		py: Python<'_>,
	) -> PyResult<(Self, PyJointBuilderBase)> {
		let builder = JointBuilder::new(name, joint_type.into());
		let mut base = PyJointBuilderBase::new(py, builder)?;

		if let Some(kwds) = kwds {
			if let Some(transform) = kwds
				.get_item(intern!(py, "transform"))?
				.map(FromPyObject::extract)
			{
				base.transform = transform?;
				kwds.del_item(intern!(py, "transform"))?;
			}

			if let Some(axis) = kwds
				.get_item(intern!(py, "axis"))?
				.map(FromPyObject::extract)
			{
				base.builder.with_axis(axis?);
				kwds.del_item(intern!(py, "axis"))?;
			}

			// TODO: Calibration
			// if let Some(calibration) = kwds.get_item(intern!(py, "calibration")).map(FromPyObject::extract){
			//
			// }

			// if let Some(dynamics) = kwds.get_item(intern!(py, "dynamics")).map()

			if let Some(limit) = kwds
				.get_item(intern!(py, "limit"))?
				.map::<PyResult<PyLimit>, _>(FromPyObject::extract)
			{
				*base.builder.limit_mut() = Some(limit?.into());
				kwds.del_item(intern!(py, "limit"))?;
			}

			// if let Some(mimic) = kwds.get_item(intern!(py, "mimic")).map()

			// if let Some(safety_controller) = kwds.get_item(intern!(py, "safety_controller")).map()

			if !kwds.is_empty() {
				let qual_name = py
					.get_type::<Self>()
					.getattr(intern!(py, "__new__"))?
					.getattr(intern!(py, "__qualname__"))?;
				return Err(PyTypeError::new_err(format!(
					"{}() got an unexpected keyword argument {}",
					qual_name,
					kwds.keys()
						.get_item(0)
						.expect("The dict should not have been empty")
						.repr()?
				)));
			}
		}

		Ok((Self, base))
	}

	// #[setter]
	// fn set_transform(mut slf: PyRefMut<'_, Self>, transform: Option<Py<PyTransform>>) {
	// 	slf.as_mut().transform = transform
	// }
	// // TODO: transform advanced

	// TEMP implementation
	//
	// TODO: Something
	fn add_origin_offset(mut slf: PyRefMut<'_, Self>, x: f32, y: f32, z: f32) {
		slf.as_mut().builder = slf.as_ref().builder.clone().add_origin_offset((x, y, z));
	}

	#[setter]
	fn set_axis(mut slf: PyRefMut<'_, Self>, axis: Option<(f32, f32, f32)>) {
		match (axis, slf.as_ref().builder.axis().is_some()) {
			(Some(axis), _) => slf.as_mut().builder.with_axis(axis),
			(None, true) => {
				// This would be easier
				// self.inner = JointBuilder{
				// 	axis: None,
				// 	..self.inner.clone()
				// }
				// TODO: This is a lot of work, it is easier to change the Rust libary
				todo!()
			}
			(None, false) => (),
		}
	}

	pub fn __repr__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<String> {
		let class_name = py
			.get_type::<Self>()
			.getattr(intern!(py, "__qualname__"))?
			.extract::<&str>()?;

		let super_self = slf.as_ref();

		// TODO: EXPAND
		Ok(format!(
			"{class_name}({}, {}, ...)",
			super_self.get_name(),
			super_self.get_joint_type().__pyo3__repr__()
		))
	}

	fn change_group_id(
		mut slf: PyRefMut<'_, Self>,
		new_group_id: String,
		_py: Python<'_>,
	) -> PyResult<()> {
		slf.as_mut()
			.builder
			.change_group_id(new_group_id)
			.map_err(GroupIDError::from)
	}

	fn apply_group_id(mut slf: PyRefMut<'_, Self>, _py: Python<'_>) {
		slf.as_mut().builder.apply_group_id()
	}
}
