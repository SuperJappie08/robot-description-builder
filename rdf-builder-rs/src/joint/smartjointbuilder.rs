mod smartjointtypes;
pub mod smartparams;

use crate::JointBuilder;
use smartparams::{NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController};

pub use smartjointtypes::{FixedType, NoType, RevoluteType};

#[derive(Debug)]
pub enum OffsetMode {
	Offset(f32, f32, f32),
	FigureItOut,
}

#[derive(Debug, Default)]
pub struct SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController> {
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
	pub fn new(
		name: String,
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
			name: name,
			joint_type: NoType,
			..SmartJointBuilder::default()
		}
	}

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
}

// TODO: Not sure if this is how i want it
impl
	From<
		SmartJointBuilder<
			FixedType,
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
			FixedType,
			NoAxis,
			NoCalibration,
			NoDynamics,
			NoLimit,
			NoMimic,
			NoSafetyController,
		>,
	) -> Self {
		JointBuilder::new(value.name, crate::JointType::Fixed)
	}
}
