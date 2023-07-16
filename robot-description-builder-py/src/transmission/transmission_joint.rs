use itertools::Itertools;
use pyo3::{prelude::*, types::PyString};
use robot_description_builder::transmission::{
	TransmissionHardwareInterface, TransmissionJointBuilder,
};

use crate::joint::{PyJoint, PyJointBuilderBase};

use super::PyTransmissionHardwareInterface;

fn get_name_from_joint_or_name(obj: &PyAny) -> PyResult<String> {
	if obj.is_instance_of::<PyJoint>() {
		obj.extract::<PyJoint>()?.get_name()
	} else if obj.is_instance_of::<PyJointBuilderBase>() {
		Ok(obj.extract::<PyJointBuilderBase>()?.get_name())
	} else if obj.is_instance_of::<PyString>() {
		obj.extract()
	} else {
		Err(pyo3::exceptions::PyTypeError::new_err(format!(
			"Expected a Joint, JointBuilder or str, got a {}",
			obj.get_type()
		)))
	}
}

// This needs a rewrite anyway, so it will be a bit weird for now
#[pyclass(
	name = "TransmissionJointBuilder",
	module = "robot_description_builder.transmission"
)]
#[derive(Debug, Clone)]
pub struct PyTransmissionJointBuilder {
	name: String,
	/// FUTURE-FIXME: Make Py<PyList> for better python integration
	hardware_interfaces: Vec<PyTransmissionHardwareInterface>,
}

#[pymethods]
impl PyTransmissionJointBuilder {
	#[new]
	fn py_new(
		#[pyo3(from_py_with = "get_name_from_joint_or_name")] name: String,
		#[pyo3(from_py_with = "crate::utils::one_or_list")] hardware_interfaces: Vec<
			PyTransmissionHardwareInterface,
		>,
	) -> Self {
		Self {
			name,
			hardware_interfaces,
		}
	}

	#[getter]
	fn get_name(&self) -> String {
		self.name.clone()
	}

	#[setter]
	fn set_name(&mut self, new_name: String) {
		self.name = new_name;
	}

	#[getter]
	fn get_hardware_interfaces(&self) -> Vec<PyTransmissionHardwareInterface> {
		self.hardware_interfaces.to_vec()
	}
}

impl From<PyTransmissionJointBuilder> for TransmissionJointBuilder {
	fn from(value: PyTransmissionJointBuilder) -> Self {
		let mut hw_interfaces = value.hardware_interfaces.into_iter().map(Into::into);
		// First unwrap is Ok
		let result = Self::new(value.name, hw_interfaces.next().unwrap());

		hw_interfaces.fold(result, |builder, hw_interface| {
			builder.with_hw_inteface(hw_interface)
		})
	}
}

impl TryFrom<TransmissionJointBuilder> for PyTransmissionJointBuilder {
	type Error = PyErr;

	fn try_from(value: TransmissionJointBuilder) -> Result<Self, PyErr> {
		Ok(Self::py_new(
			value.name().clone(),
			value
				.hw_interfaces()
				.iter()
				.copied()
				.map(TryInto::try_into)
				.process_results(|iter| iter.collect())?,
		))
	}
}
