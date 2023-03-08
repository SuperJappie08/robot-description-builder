use crate::joint::smartjointbuilder::SmartJointBuilder;

pub trait MimicAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoMimic;


impl<Type, Axis, Calibration, Dynamics, Limit, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, NoMimic, SafetyController>
where
	Type: MimicAllowed,
{

}