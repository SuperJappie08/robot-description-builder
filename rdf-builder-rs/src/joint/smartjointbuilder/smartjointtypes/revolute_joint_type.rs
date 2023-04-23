use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::JointType,
	joint::{
		smartjointbuilder::{
			smartparams::{
				smart_joint_datatraits::{self, LimitDataType},
				smart_joint_specification, WithLimit,
			},
			SmartJointBuilder,
		},
		BuildJoint, Joint, JointBuilder,
	},
	link::Link,
	ArcLock, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct RevoluteType;

impl From<RevoluteType> for JointType {
	fn from(_value: RevoluteType) -> Self {
		JointType::Revolute
	}
}

impl smart_joint_specification::AxisAllowed for RevoluteType {}
impl smart_joint_specification::CalibrationAllowed for RevoluteType {}
impl smart_joint_specification::DynamicsAllowed for RevoluteType {}
impl smart_joint_specification::LimitAllowed for RevoluteType {}
impl smart_joint_specification::MimicAllowed for RevoluteType {}
impl smart_joint_specification::SafetyControllerAllowed for RevoluteType {}

impl<Axis, Calibration, Dynamics, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<RevoluteType, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
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

impl<Axis, Calibration, Dynamics, Mimic, SafetyController>
	From<
		SmartJointBuilder<
			RevoluteType,
			Axis,
			Calibration,
			Dynamics,
			WithLimit,
			Mimic,
			SafetyController,
		>,
	> for JointBuilder
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn from(
		value: SmartJointBuilder<
			RevoluteType,
			Axis,
			Calibration,
			Dynamics,
			WithLimit,
			Mimic,
			SafetyController,
		>,
	) -> Self {
		let mut joint_builder = JointBuilder::new(value.name, value.joint_type.into());

		// if let Some(mode) = self.offset {
		// 	// todo!("BUILD FUNCTIOn")
		// 	joint_builder = match mode {
		// 		OffsetMode::Offset(x, y, z) => joint_builder.add_origin_offset((x, y, z)),
		// 		OffsetMode::FigureItOut(_) => todo!(),
		// 	}
		// }
		//
		// if let Some(rotation) = self.rotation {
		// 	// TODO: MAKE SMARTER
		// 	joint_builder = joint_builder.add_origin_rotation(rotation)
		// }

		joint_builder.with_origin(value.offset);

		value.axis.simplify(&mut joint_builder);
		value.calibration.simplify(&mut joint_builder);
		value.dynamics.simplify(&mut joint_builder);
		value.limit.simplify(&mut joint_builder, false);
		value.mimic.simplify(&mut joint_builder);
		value.safety_controller.simplify(&mut joint_builder);

		// THIS MIGHT BE DONE// todo!("Create a `RevoluteJoint`")

		joint_builder
	}
}
