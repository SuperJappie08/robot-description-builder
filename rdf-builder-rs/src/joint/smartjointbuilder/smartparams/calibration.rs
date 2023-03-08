use crate::joint::smartjointbuilder::SmartJointBuilder;

pub trait CalibrationAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoCalibration;

#[derive(Debug, Default, Clone)]
pub struct WithCalibration {
	rising: String,
	falling: String,
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: CalibrationAllowed,
{
	pub fn set_calibration(
		self,
		_calibration: String,
	) -> SmartJointBuilder<Type, Axis, WithCalibration, Dynamics, Limit, Mimic, SafetyController> {
		todo!()
	}
}
