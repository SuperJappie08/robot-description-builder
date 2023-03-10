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

pub mod smart_joint_specification {
	pub use crate::joint::smartjointbuilder::smartparams::{
		axis::AxisAllowed, calibration::CalibrationAllowed, dynamics::DynamicsAllowed,
		limit::LimitAllowed, mimic::MimicAllowed, safety_controller::SafetyControllerAllowed,
	};
}

pub(crate) mod smart_joint_datatraits {
	pub trait AxisDataType {}
	pub trait CalibrationDataType {}
	pub trait DynamicsDataType {}
	pub trait LimitDataType {}
	pub trait MimicDataType {}
	pub trait SafetyControllerDataType {}
}
