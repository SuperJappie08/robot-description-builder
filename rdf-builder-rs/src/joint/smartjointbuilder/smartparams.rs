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
pub use mimic::{NoMimic};
pub use safety_controller::{NoSafetyController};

pub mod smart_joint_specification {
	pub use super::axis::AxisAllowed;
	pub use super::calibration::CalibrationAllowed;
    pub use super::dynamics::DynamicsAllowed;
    pub use super::limit::LimitAllowed;
    pub use super::mimic::MimicAllowed;
    pub use super::safety_controller::SafetyControllerAllowed;
}
