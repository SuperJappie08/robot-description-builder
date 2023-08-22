use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{
		jointbuilder::{BuildJoint, JointBuilder},
		smartjointbuilder::{
			smartparams::{smart_joint_datatraits::*, *},
			SmartJointBuilder,
		},
		Joint, JointType,
	},
	link::Link,
	utils::{ArcLock, WeakLock},
};

// TODO: Maybe flip the JointType Doc order
/// A representation of a floating joint (`JointType::Floating`) for the `SmartJointBuilder`.
///
/// See [`JointType::Floating`] for more details.
///
/// # DevNotes
///
/// Floating Joints are weird, as they do not behave like the other joint types.
/// This causes problems with lots of different tooling for ROS/URDF, like  `sensor_msgs/JointState` messages from for example [`joint_state_publisher`](https://github.com/ros/robot_model/issues/188) which ignores it.
///
/// It also does not help, that `kdl_parser` [only supports single DOF joints](https://github.com/ros/kdl_parser/blob/74d4ee3bc6938de8ae40a700997baef06114ea1b/kdl_parser/src/kdl_parser.cpp#L103).
///
/// There are also some problems with the definition `URDF` fields, when trying to apply them to Floating Joints. [(Example)](https://answers.ros.org/question/359863/whats-the-lower-and-upper-limit-for-a-floating-joint-in-urdf/)
///
/// There for I have decided to do the following:
///  - The `SmartJointBuilder` allows the creation of a `Joint` with `JointType::Floating`, however a warning will be shown if the `logging` feature is enabled. This will explain that "*It is very likely it won't work in your simulator, so use at you own risk*".
///  - The following `Joint` fields are **not** allowed, because they don't make sense to me. (I am willing to allow it if someone can explain their meaning):
///    - `axis`, no physical equivalent.
///    - `calibration`, this type of joint is free-floating to my understanding. If something is uncontrollable, it does not have to be calibrated.
///    - `dynamics`, because friction and damping are specified to be either axial or rotational units, but floating as either no friction due to the lack of a physical connection or both which is not an option.
///    - `limit`, because the joint position can not be specified with one (real) number, so the fields can not be filled with a correct physical representation.
///    - `mimic`, because the state can not be represented with one (real) number, so the equation does not work.
///    - `safetycontroller`, since it depends on `limit` for the soft limits.
///
/// However it is to be noted, that all these things are still possible to add using `JointBuilder`.
///
/// ## Extra sources:
/// - [GitHub:URDF/issue/Support for other joint types](https://github.com/ros/urdf/issues/3)
/// - [Gazebo uses KDL, so doesn't support Floating](https://get-help.robotigniteacademy.com/t/urdfs-floating-joint-not-working-in-gazebo/8864)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct FloatingType;

impl_jointtype_traits!(FloatingType, false);

impl From<FloatingType> for JointType {
	fn from(_value: FloatingType) -> Self {
		JointType::Floating
	}
}

// Axis is not allowed
// Calibration is not allowed
// Dynamics is not allowed, because it does not make sense.
// Limit is not allowed, because it does not make sense.
// Mimic is not allowed, because the joint state can not be represented with one (real) number.
// SafetyController is not allowed, because Limit is not allowed

impl BuildJoint
	for SmartJointBuilder<
		FloatingType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	>
{
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_shape_data: crate::link::LinkShapeData,
	) -> ArcLock<Joint> {
		Into::<JointBuilder>::into(self).build(tree, parent_link, child_link, parent_shape_data)
	}
}

impl
	From<
		SmartJointBuilder<
			FloatingType,
			NoAxis,
			NoCalibration,
			NoDynamics,
			NoLimit,
			NoMimic,
			NoSafetyController,
		>,
	> for JointBuilder
{
	fn from(
		value: SmartJointBuilder<
			FloatingType,
			NoAxis,
			NoCalibration,
			NoDynamics,
			NoLimit,
			NoMimic,
			NoSafetyController,
		>,
	) -> Self {
		let mut joint_builder = JointBuilder::new(value.name, value.joint_type.into());

		#[cfg(any(feature = "logging", test))]
		log::warn!("Floating Joints are kind of broken, si it is very likely it won't work in your simulator. Use at you own risk!");

		joint_builder.with_transform(value.transform.unwrap_or_default());

		// Probably unneccessary
		value.axis.simplify(&mut joint_builder);
		value.calibration.simplify(&mut joint_builder);
		value.dynamics.simplify(&mut joint_builder);
		value
			.limit
			.simplify(&mut joint_builder, FloatingType::IS_CONTINOUS);
		value.mimic.simplify(&mut joint_builder);
		value.safety_controller.simplify(&mut joint_builder);

		joint_builder
	}
}
