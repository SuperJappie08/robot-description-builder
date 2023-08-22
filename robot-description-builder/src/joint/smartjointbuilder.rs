mod smartjointtypes;

pub mod smartparams;
pub use smartjointtypes::{
	ContinuousType, FixedType, FloatingType, NoType, PlanarType, PrismaticType, RevoluteType,
};

use super::joint_tranform_mode::JointTransformMode;
use crate::{link::LinkShapeData, transform::Transform};
use smartparams::{NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController};

use self::smartparams::smart_joint_datatraits;

#[cfg(feature = "wrapper")]
use super::jointbuilder::JointBuilder;

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
	/// The transform from the origin of the parent to the origin of this `Joint`.
	///
	/// In URDF this field is refered to as `<origin>`.
	transform: Option<JointTransformMode>,
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
	/// Renames the current `SmartJointBuilder`.
	pub fn rename(mut self, name: impl Into<String>) -> Self {
		self.name = name.into();
		self
	}

	pub fn add_transform(mut self, transform: Transform) -> Self {
		self.transform = Some(transform.into());
		self
	}

	pub fn add_dynamic_transform(mut self, func: fn(LinkShapeData) -> Transform) -> Self {
		self.transform = Some(func.into());
		self
	}
}

impl
	SmartJointBuilder<NoType, NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController>
{
	/// Created a new `JointType`-less `SmartJointBuilder`.
	pub fn new(
		name: impl Into<String>,
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

	/// Creates a new `SmartJointBuilder` with `JointType::Revolute`.
	pub fn new_revolute(
		name: impl Into<String>,
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

	/// Creates a new `SmartJointBuilder` of type `Continuous`.
	pub fn new_continuous(
		name: impl Into<String>,
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

	/// Creates a new `SmartJointBuilder` of type `Prismatic`.
	pub fn new_prismatic(
		name: impl Into<String>,
	) -> SmartJointBuilder<
		PrismaticType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).prismatic()
	}

	/// Creates a new `SmartJointBuilder` of type `Fixed`.
	pub fn new_fixed(
		name: impl Into<String>,
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

	/// Creates a new `SmartJointBuilder` of type `Floating`.
	pub fn new_floating(
		name: impl Into<String>,
	) -> SmartJointBuilder<
		FloatingType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).floating()
	}

	/// Creates a new `SmartJointBuilder` of type `Planar`.
	pub fn new_planar(
		name: impl Into<String>,
	) -> SmartJointBuilder<
		PlanarType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	> {
		Self::new(name).planar()
	}

	// TODO: Maybe move these to a more generic implementation
	/// Converts this `SmartJointBuilder` to the `Revolute` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	/// Converts this `SmartJointBuilder` to the `Continuous` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	/// Converts this `SmartJointBuilder` to the `Prismatic` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	// TODO: Maybe move these to a more generic implementation
	/// Converts this `SmartJointBuilder` to the `Fixed` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	// TODO: ADD WARNING
	/// Converts this `SmartJointBuilder` to the `Floating` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}

	// TODO: ADD WARNING
	/// Converts this `SmartJointBuilder` to the `Planar` type.
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
			transform: self.transform,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
			limit: self.limit,
			mimic: self.mimic,
			safety_controller: self.safety_controller,
		}
	}
}

#[cfg(feature = "wrapper")]
impl<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
	SmartJointBuilder<Type, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Type: smart_joint_datatraits::JointTypeTrait,
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	/// Convert the `SmartJointBuilder` to a normal `JointBuilder`
	///
	/// # Safety
	/// A normal [`JointBuilder`] is not checked for the required fields of its [`JointType`](super::JointType).
	/// This could result in invalid descriptions.
	pub unsafe fn as_simple(&self) -> JointBuilder {
		let mut joint_builder = JointBuilder::new(self.name.clone(), self.joint_type.as_type());

		self.axis.simplify(&mut joint_builder);
		self.calibration.simplify(&mut joint_builder);
		self.dynamics.simplify(&mut joint_builder);
		self.limit.simplify(&mut joint_builder, Type::IS_CONTINOUS);
		self.mimic.simplify(&mut joint_builder);
		self.safety_controller.simplify(&mut joint_builder);

		joint_builder
	}
}
