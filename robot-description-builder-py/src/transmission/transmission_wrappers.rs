use crate::{joint::PyJoint, utils::TryIntoRefPyAny};
use pyo3::{intern, prelude::*, PyTypeInfo};
use robot_description_builder::transmission::{TransmissionActuator, TransmissionJointBuilder};

use super::PyTransmissionHardwareInterface;

#[derive(Debug, Clone, FromPyObject)]
pub(super) struct PyTransmissionActuator(String, Option<f32>);

unsafe impl PyTypeInfo for PyTransmissionActuator {
	const NAME: &'static str = "TransmissionActuator";

	const MODULE: Option<&'static str> = Some("robot_description_builder.transmission");

	type AsRefTarget = PyAny;

	fn type_object_raw(py: Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
		py.import(intern!(py, "robot_description_builder.transmission"))
			.unwrap()
			.getattr(intern!(py, PyTransmissionActuator::NAME))
			.unwrap()
			.get_type_ptr()
	}
}

// impl IntoPy<PyObject> for PyTransmissionActuator {
// 	fn into_py(self, py: Python<'_>) -> PyObject {
// 		// Unwraping should be Ok here
// 		let py_actuator = py
// 			.import(intern!(py, "robot_description_builder.transmission"))
// 			.unwrap()
// 			.getattr(intern!(py, "TransmissionActuator"))
// 			.unwrap();

// 		py_actuator
// 			.call_method1(intern!(py, "__new__"), (py_actuator, self.0, self.1))
// 			.unwrap()
// 			.to_object(py)
// 	}
// }

impl TryIntoRefPyAny for PyTransmissionActuator {
	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<&PyAny> {
		// Unwraping should be Ok here
		let py_actuator = py
			.import(intern!(py, "robot_description_builder.transmission"))?
			.getattr(intern!(py, "TransmissionActuator"))?;

		py_actuator.call_method1(intern!(py, "__new__"), (py_actuator, self.0, self.1))
	}
}

crate::impl_into_py_callback!(PyTransmissionActuator);

impl From<TransmissionActuator> for PyTransmissionActuator {
	fn from(value: TransmissionActuator) -> Self {
		Self(value.name().clone(), value.mechanical_reduction().copied())
	}
}

impl From<PyTransmissionActuator> for TransmissionActuator {
	fn from(value: PyTransmissionActuator) -> Self {
		let mut actuator = Self::new(value.0);

		if let Some(reduction) = value.1 {
			actuator = actuator.mechanically_reduced(reduction);
		}

		actuator
	}
}

// #[derive(Debug, FromPyObject)]
// pub(super) enum PyTransmissionJoint {
// 	UnBuild(
// 		String,
// 		#[pyo3(from_py_with = "crate::utils::one_or_list")] Vec<PyTransmissionHardwareInterface>,
// 	),
// 	Build(
// 		PyJoint,
// 		#[pyo3(from_py_with = "crate::utils::one_or_list")] Vec<PyTransmissionHardwareInterface>,
// 	),
// }

// impl PyTransmissionJoint {
// 	fn name(&self) -> PyResult<String> {
// 		match self {
// 			PyTransmissionJoint::UnBuild(name, _) => Ok(name.clone()),
// 			PyTransmissionJoint::Build(joint, _) => joint.get_name(),
// 		}
// 	}

// 	fn hardware_interfaces(&self) -> &Vec<PyTransmissionHardwareInterface> {
// 		match self {
// 			PyTransmissionJoint::UnBuild(_, interfaces) => interfaces,
// 			PyTransmissionJoint::Build(_, interfaces) => interfaces,
// 		}
// 	}
// }

// unsafe impl PyTypeInfo for PyTransmissionJoint {
// 	const NAME: &'static str = "TransmissionJoint";

// 	const MODULE: Option<&'static str> = Some("robot_description_builder.transmission");

// 	type AsRefTarget = PyAny;

// 	fn type_object_raw(py: Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
// 		py.import(intern!(py, "robot_description_builder.transmission"))
// 			.unwrap()
// 			.getattr(intern!(py, PyTransmissionJoint::NAME))
// 			.unwrap()
// 			.get_type_ptr()
// 	}
// }

// impl TryIntoRefPyAny for PyTransmissionJoint {
// 	fn try_into_py_ref(self, py: Python<'_>) -> PyResult<&PyAny> {
// 		// Unwraping should be Ok here
// 		let py_joint = py // ? .get_type::<PyTransmissionJoint>()
// 			.import(intern!(py, "robot_description_builder.transmission"))?
// 			.getattr(intern!(py, "TransmissionJoint"))?;

// 		py_joint.call_method1(
// 			intern!(py, "__new__"),
// 			(py_joint, self.name()?, self.hardware_interfaces().clone()),
// 		)
// 	}
// }

// impl IntoPy<PyObject> for PyTransmissionJoint {
//     fn into_py(self, py: Python<'_>) -> PyObject {
// 		let py_TransmissionJoint = py.

//         match self {
//             PyTransmissionJoint::UnBuild(name, list) => todo!(),
//             PyTransmissionJoint::Build(_, _) => todo!(),
//         }
//     }
// }

// crate::impl_into_py_callback!(PyTransmissionJoint);

// impl TryFrom<PyTransmissionJoint> for TransmissionJointBuilder {
// 	type Error = PyErr;

// 	fn try_from(val: PyTransmissionJoint) -> Result<Self, PyErr> {
// 		// Because it is enforced that the List should not be empty from conversion from python, unwrapping on the first element is ok.
// 		let (builder, hw_interfaces) = match val {
// 			PyTransmissionJoint::UnBuild(name, hw_interfaces) => {
// 				let mut hw_interfaces = hw_interfaces.into_iter();
// 				let builder =
// 					TransmissionJointBuilder::new(name, hw_interfaces.next().unwrap().into());
// 				Result::<_, PyErr>::Ok((builder, hw_interfaces))
// 			}
// 			PyTransmissionJoint::Build(joint, hw_interfaces) => {
// 				let mut hw_interfaces = hw_interfaces.into_iter();
// 				let builder = TransmissionJointBuilder::new(
// 					joint.get_name()?,
// 					hw_interfaces.next().unwrap().into(),
// 				);

// 				Ok((builder, hw_interfaces))
// 			}
// 		}?;

// 		Ok(hw_interfaces
// 			.map(Into::into)
// 			.fold(builder, |builder, hw_interface| {
// 				builder.with_hw_inteface(hw_interface)
// 			}))
// 	}
// }

// impl From<TransmissionJointBuilder> for PyTransmissionJoint {
// 	fn from(value: TransmissionJointBuilder) -> Self {
// 		Self::UnBuild(
// 			value.name().clone(),
// 			value
// 				.hw_interfaces()
// 				.iter()
// 				.cloned()
// 				.map(TryInto::try_into)
// 				.map(|v| v.unwrap())
// 				.collect(),
// 		)
// 	}
// }

// #[cfg(test)]
// mod tests {
// 	use super::{PyTransmissionHardwareInterface, PyTransmissionJoint, TransmissionJointBuilder};

// 	mod joint {
// 		use robot_description_builder::transmission::TransmissionHardwareInterface;

// 		use super::{
// 			PyTransmissionHardwareInterface, PyTransmissionJoint, TransmissionJointBuilder,
// 		};

// 		#[test]
// 		fn into_joint_builder_from_unbuild() {
// 			// Check if first element is not duplicated.
// 			assert_eq!(
// 				TryInto::<TransmissionJointBuilder>::try_into(PyTransmissionJoint::UnBuild(
// 					"some_joint".to_owned(),
// 					vec![PyTransmissionHardwareInterface::JointCommandInterface]
// 				))
// 				.unwrap(),
// 				TransmissionJointBuilder::new(
// 					"some_joint",
// 					TransmissionHardwareInterface::JointCommandInterface
// 				)
// 			);

// 			assert_eq!(
// 				TryInto::<TransmissionJointBuilder>::try_into(PyTransmissionJoint::UnBuild(
// 					"some_joint".to_owned(),
// 					vec![
// 						PyTransmissionHardwareInterface::JointCommandInterface,
// 						PyTransmissionHardwareInterface::IMUSensorInterface
// 					]
// 				))
// 				.unwrap(),
// 				TransmissionJointBuilder::new(
// 					"some_joint",
// 					TransmissionHardwareInterface::JointCommandInterface
// 				)
// 				.with_hw_inteface(TransmissionHardwareInterface::IMUSensorInterface)
// 			);
// 		}
// 	}
// }
