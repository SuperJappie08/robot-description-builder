#[derive(Debug, Default, Clone)]
pub struct NoType;

#[derive(Debug, Default, Clone)]
pub struct FixedType;

#[derive(Debug, Default, Clone)]
pub struct RevoluteType;

pub trait AxisAllowed {}
impl AxisAllowed for RevoluteType {}
pub trait CalibrationAllowed {}
impl CalibrationAllowed for RevoluteType {}
pub trait DynamicsAllowed {}
impl DynamicsAllowed for RevoluteType {}

#[derive(Debug, Default, Clone)]
pub struct NoAxis;

#[derive(Debug, Default, Clone)]
pub struct WithAxis(f32, f32, f32);

#[derive(Debug, Default, Clone)]
pub struct NoCalibration;

#[derive(Debug, Default, Clone)]
pub struct NoDynamics;
#[derive(Debug, Default, Clone)]
pub struct WithDynamics {
	damping: Option<f32>,
	friction: Option<f32>,
}

#[derive(Debug, Default, Clone)]
pub struct WithCalibration {
	rising: String,
	falling: String,
}

#[derive(Debug)]
pub enum OffsetMode {
	Offset(f32, f32, f32),
	FigureItOut,
}

#[derive(Debug, Default)]
pub struct SmartJointBuilder<Type, Axis, Calibration, Dynamics> {
	name: String,
	joint_type: Type,
	offset: Option<OffsetMode>,
	rotation: Option<(f32, f32, f32)>,
	axis: Axis,
	calibration: Calibration,
	dynamics: Dynamics,
}

impl<Type, Axis, Calibration, Dynamics> SmartJointBuilder<Type, Axis, Calibration, Dynamics> {
	pub fn add_offset(mut self, offset_mode: OffsetMode) -> Self {
		self.offset = Some(offset_mode);
		self
	}

	pub fn add_rotation(mut self, rotation: (f32, f32, f32)) -> Self {
		self.rotation = Some(rotation);
		self
	}
}

impl SmartJointBuilder<NoType, NoAxis, NoCalibration, NoDynamics> {
	pub fn new(name: String) -> SmartJointBuilder<NoType, NoAxis, NoCalibration, NoDynamics> {
		SmartJointBuilder {
			name: name,
			joint_type: NoType,
			..SmartJointBuilder::default()
		}
	}

	pub fn revolute(self) -> SmartJointBuilder<RevoluteType, NoAxis, NoCalibration, NoDynamics> {
		SmartJointBuilder {
			name: self.name,
			joint_type: RevoluteType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
		}
	}

	pub fn fixed(self) -> SmartJointBuilder<FixedType, NoAxis, NoCalibration, NoDynamics> {
		SmartJointBuilder {
			name: self.name,
			joint_type: FixedType,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: self.dynamics,
		}
	}
}

impl<Type, Axis, Calibration, Dynamics> SmartJointBuilder<Type, Axis, Calibration, Dynamics>
where
	Type: AxisAllowed,
{
	pub fn set_axis(
		self,
		axis: (f32, f32, f32),
	) -> SmartJointBuilder<Type, WithAxis, Calibration, Dynamics> {
		let length = f32::sqrt(axis.0 * axis.0 + axis.1 * axis.1 + axis.2 * axis.2);
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
			axis: WithAxis(axis.0 / length, axis.1 / length, axis.2 / length),
			calibration: self.calibration,
			dynamics: self.dynamics,
		}
	}
}

impl<Type, Axis, Calibration, Dynamics> SmartJointBuilder<Type, Axis, Calibration, Dynamics>
where
	Type: CalibrationAllowed,
{
	pub fn set_calibration(
		self,
		_calibration: String,
	) -> SmartJointBuilder<Type, Axis, WithCalibration, Dynamics> {
		todo!()
	}
}

impl<Type, Axis, Calibration> SmartJointBuilder<Type, Axis, Calibration, NoDynamics>
where
	Type: DynamicsAllowed,
{
	pub fn set_damping(
		self,
		damping: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, WithDynamics> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: WithDynamics {
				damping: Some(damping),
				friction: None,
			},
		}
	}

	pub fn set_friction(
		self,
		friction: f32,
	) -> SmartJointBuilder<Type, Axis, Calibration, WithDynamics> {
		SmartJointBuilder {
			name: self.name,
			joint_type: self.joint_type,
			offset: self.offset,
			rotation: self.rotation,
			axis: self.axis,
			calibration: self.calibration,
			dynamics: WithDynamics {
				damping: None,
				friction: Some(friction),
			},
		}
	}
}

impl<Type, Axis, Calibration> SmartJointBuilder<Type, Axis, Calibration, WithDynamics>
where
	Type: DynamicsAllowed,
{
	pub fn set_damping(mut self, damping: f32) -> Self {
		self.dynamics.damping = Some(damping);
		self
	}

	pub fn set_friction(mut self, friction: f32) -> Self {
		self.dynamics.friction = Some(friction);
		self
	}
}
