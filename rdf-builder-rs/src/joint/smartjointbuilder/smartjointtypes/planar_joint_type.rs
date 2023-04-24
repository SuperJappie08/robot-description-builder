use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{
		jointbuilder::{BuildJoint, JointBuilder},
		smartjointbuilder::{
			smartparams::{smart_joint_datatraits, smart_joint_specification},
			SmartJointBuilder,
		},
		Joint, JointType,
	},
	link::Link,
	ArcLock, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct PlanarType;

impl From<PlanarType> for JointType {
	fn from(_value: PlanarType) -> Self {
		JointType::Planar
	}
}

impl smart_joint_specification::AxisAllowed for PlanarType {}
/// TODO: Figure out if this is allowed
impl smart_joint_specification::CalibrationAllowed for PlanarType {}
impl smart_joint_specification::DynamicsAllowed for PlanarType {}
/// TODO: Figure out if this is allowed/if joint is continous
impl smart_joint_specification::LimitAllowed for PlanarType {}
/// TODO: Figure out if this is allowed
impl smart_joint_specification::MimicAllowed for PlanarType {}
/// TODO: Figure out if this is allowed
impl smart_joint_specification::SafetyControllerAllowed for PlanarType {}

impl<Axis, Calibration, Dynamics, Limit, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<PlanarType, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
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

impl<Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
	From<SmartJointBuilder<PlanarType, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>>
	for JointBuilder
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn from(
		value: SmartJointBuilder<
			PlanarType,
			Axis,
			Calibration,
			Dynamics,
			Limit,
			Mimic,
			SafetyController,
		>,
	) -> Self {
		let mut joint_builder = JointBuilder::new(value.name, value.joint_type.into());

		joint_builder.with_origin(value.origin.unwrap_or_default());

		value.axis.simplify(&mut joint_builder);
		value.calibration.simplify(&mut joint_builder);
		value.dynamics.simplify(&mut joint_builder);
		value.limit.simplify(&mut joint_builder, true); // FIXME: Is it contiuos tho?
		value.mimic.simplify(&mut joint_builder);
		value.safety_controller.simplify(&mut joint_builder);

		joint_builder
	}
}
