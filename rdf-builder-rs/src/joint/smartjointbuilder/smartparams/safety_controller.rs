use crate::joint::smartjointbuilder::SmartJointBuilder;

pub trait SafetyControllerAllowed {}

#[derive(Debug, Default, Clone)]
pub struct NoSafetyController;