mod fixed_joint_type;
mod revolute_joint_type;

#[derive(Debug, Default, Clone)]
pub struct NoType;

pub use fixed_joint_type::FixedType;
pub use revolute_joint_type::RevoluteType;
