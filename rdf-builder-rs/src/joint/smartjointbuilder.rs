mod smartjointtypes;
pub mod smartparams;

use smartparams::{NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController};

pub use smartjointtypes::{FixedType, NoType, RevoluteType};

use crate::link::link_data::ConnectionPoint;

use self::{
	smartjointtypes::{ContinuousType, FloatingType, PlanarType, PrismaticType},
	smartparams::smart_joint_datatraits,
};

#[derive(Debug, PartialEq, Clone)]
pub enum OffsetMode {
	Offset(f32, f32, f32),
	FigureItOut(ConnectionPoint),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	name: String,
	joint_type: Type,
	offset: Option<OffsetMode>,
	rotation: Option<(f32, f32, f32)>,
	axis: Axis,
	calibration: Calibration,
	dynamics: Dynamics,
	limit: Limit,
	mimic: Mimic,
	safety_controller: SafetyController,
}

impl<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	pub fn add_offset(mut self, offset_mode: OffsetMode) -> Self {
		self.offset = Some(offset_mode);
		self
	}

	pub fn add_rotation(mut self, rotation: (f32, f32, f32)) -> Self {
		self.rotation = Some(rotation);
		self
	}
}

impl
	SmartJointBuilder<NoType, NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController>
{
	pub fn new<Name: Into<String>>(
		name: Name,
	) -> SmartJointBuilder<
		NoType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: name.into(),
			joint_type: NoType,
			..SmartJointBuilder::default()
		}
	}

	/// TODO: Maybe do it like this
	/// I Like it...
	pub fn new_revolute<Name: Into<String>>(
		name: Name,
	) -> SmartJointBuilder<
		RevoluteType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).revolute()
	}

	pub fn new_continuous<Name: Into<String>>(
		name: Name,
	) -> SmartJointBuilder<
		ContinuousType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).continuous()
	}

	pub fn new_fixed<Name: Into<String>>(
		name: Name,
	) -> SmartJointBuilder<
		FixedType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).fixed()
	}

	/// TODO: Maybe move these to a more generic implementation
	pub fn revolute(
		self,
	) -> SmartJointBuilder<
		RevoluteType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: RevoluteType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	pub fn continuous(
		self,
	) -> SmartJointBuilder<
		ContinuousType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: ContinuousType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	pub fn prismatic(
		self,
	) -> SmartJointBuilder<
		PrismaticType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: PrismaticType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	/// TODO: Maybe move these to a more generic implementation
	pub fn fixed(
		self,
	) -> SmartJointBuilder<
		FixedType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: FixedType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	pub fn floating(
		self,
	) -> SmartJointBuilder<
		FloatingType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: FloatingType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	pub fn planar(
		self,
	) -> SmartJointBuilder<
		PlanarType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		SmartJointBuilder {
			name: self.name,
			joint_type: PlanarType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

// Convert to JointBuilder
// impl<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
// 	TryFrom<SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>>
// 	for JointBuilder
// 	where Axis: smart_joint_datatraits::AxisDataType,
// 	Calibration: smart_joint_datatraits::CalibrationDataType,
// 	Dynamics: smart_joint_datatraits::DynamicsDataType,
// 	Limit: smart_joint_datatraits::LimitDataType,
// 	Mimic: smart_joint_datatraits::MimicDataType,
// 	SafetyController: smart_joint_datatraits::SafetyControllerDataType
// {
// 	type Error = JointBuilder;

// 	fn try_from(
// 		value: SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>,
// 	) -> Result<Self, Self::Error> {
// 		todo!()
// 	}
// }
