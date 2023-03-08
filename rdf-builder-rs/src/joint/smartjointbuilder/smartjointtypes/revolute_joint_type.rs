use crate::joint::smartjointbuilder::smartparams::smart_joint_specification;

#[derive(Debug, Default, Clone)]
pub struct RevoluteType;

impl smart_joint_specification::AxisAllowed for RevoluteType {}
impl smart_joint_specification::CalibrationAllowed for RevoluteType {}
impl smart_joint_specification::DynamicsAllowed for RevoluteType {}
impl smart_joint_specification::LimitAllowed for RevoluteType {}
impl smart_joint_specification::MimicAllowed for RevoluteType {}
impl smart_joint_specification::SafetyControllerAllowed for RevoluteType {}
