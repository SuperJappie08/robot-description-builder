mod axis;
mod calibration;
mod dynamics;
mod limit;
mod mimic;
mod safety_controller;

pub use axis::{NoAxis, WithAxis};
pub use calibration::{NoCalibration, WithCalibration};
pub use dynamics::{NoDynamics, WithDynamics};
pub use limit::{NoLimit, WithLimit};
pub use mimic::{NoMimic, WithMimic};
pub use safety_controller::{NoSafetyController, WithSafetyController};

/// This module contains all traits to specify a `SmartJointBuilderType`.
pub mod smart_joint_specification {
	pub use crate::joint::smartjointbuilder::smartparams::{
		axis::AxisAllowed, calibration::CalibrationAllowed, dynamics::DynamicsAllowed,
		limit::LimitAllowed, mimic::MimicAllowed, safety_controller::SafetyControllerAllowed,
	};

	pub use super::super::smartjointtypes::{JointTypeTrait, SmartJointTypeTrait};
}

#[allow(unused_variables)]
pub(crate) mod smart_joint_datatraits {
	pub use super::super::smartjointtypes::{JointTypeTrait, SmartJointTypeTrait};

	pub trait AxisDataType {
		fn simplify(&self, joint_builder: &mut crate::joint::jointbuilder::JointBuilder) {}
	}
	pub trait CalibrationDataType {
		fn simplify(&self, joint_builder: &mut crate::joint::jointbuilder::JointBuilder) {}
	}
	pub trait DynamicsDataType {
		fn simplify(&self, joint_builder: &mut crate::joint::jointbuilder::JointBuilder) {}
	}
	pub trait LimitDataType {
		fn simplify(
			&self,
			joint_builder: &mut crate::joint::jointbuilder::JointBuilder,
			is_continous: bool,
		) {
		}
	}
	pub trait MimicDataType {
		fn simplify(&self, joint_builder: &mut crate::joint::jointbuilder::JointBuilder) {}
	}
	pub trait SafetyControllerDataType {
		fn simplify(&self, joint_builder: &mut crate::joint::jointbuilder::JointBuilder) {}
	}
}
