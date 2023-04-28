use std::sync::{Arc, RwLock, Weak};

use itertools::Itertools;
use nalgebra::{vector, Matrix3};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{joint_data, joint_tranform_mode::JointTransformMode, Joint, JointType},
	link::{
		builder::{BuildLink, LinkBuilder},
		Link, LinkShapeData,
	},
	ArcLock, WeakLock,
};

pub trait BuildJoint: Into<JointBuilder> {
	/// Creates the joint ?? and subscribes it to the right right places
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_shape_data: LinkShapeData,
	) -> ArcLock<Joint>;
}

/// NOTE: Removed Trait bound due for `Chained<JointBuilder>`
pub(crate) trait BuildJointChain {
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_link: &WeakLock<Link>,
		parent_shape_data: LinkShapeData,
	) -> ArcLock<Joint>;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct JointBuilder {
	pub(crate) name: String,
	pub(crate) joint_type: JointType, // TODO: FINISH ME
	/// TODO: Maybe add a Figure it out???
	pub(crate) origin: JointTransformMode,
	pub(crate) child: Option<LinkBuilder>,

	/// TODO: DO SOMETHING WITH THIS
	/// TODO: MAYBE CHANGE TO Vec3D Or something
	pub(crate) axis: Option<(f32, f32, f32)>,
	pub(crate) calibration: joint_data::CalibrationData,
	pub(crate) dynamics: joint_data::DynamicsData,
	pub(crate) limit: Option<joint_data::LimitData>,
	pub(crate) mimic: Option<joint_data::MimicBuilderData>,
	pub(crate) safety_controller: Option<joint_data::SafetyControllerData>,
}

impl JointBuilder {
	pub fn new(name: impl Into<String>, joint_type: JointType) -> Self {
		Self {
			name: name.into(),
			joint_type,
			..Default::default()
		}
	}

	pub fn add_origin_offset(mut self, offset: (f32, f32, f32)) -> Self {
		match &mut self.origin {
			JointTransformMode::Direct(transform) => transform.translation = Some(offset),
			JointTransformMode::FigureItOut(_) => todo!("Don't know how to do this"),
		};
		self
	}

	pub fn add_origin_rotation(mut self, rotation: (f32, f32, f32)) -> Self {
		match &mut self.origin {
			JointTransformMode::Direct(tranform) => tranform.rotation = Some(rotation),
			JointTransformMode::FigureItOut(_) => todo!("Don't know how to do this yet"),
		}
		self
	}

	/// Nominated for Deprication
	/// Maybe Not??
	#[inline]
	pub(crate) fn with_origin<JTM>(&mut self, origin: JTM)
	where
		JTM: Into<JointTransformMode>,
	{
		self.origin = origin.into();
	}

	#[inline]
	pub fn with_axis(&mut self, axis: (f32, f32, f32)) {
		self.axis = Some(axis);
	}

	/// Add the full `CalibrationData` to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_calibration_data(&mut self, calibration_data: joint_data::CalibrationData) {
		self.calibration = calibration_data;
	}

	#[inline]
	pub(crate) fn with_dynamics_data(&mut self, dynamics_data: joint_data::DynamicsData) {
		self.dynamics = dynamics_data;
	}

	#[inline]
	pub(crate) fn with_limit_data(&mut self, limit_data: joint_data::LimitData) {
		self.limit = Some(limit_data);
	}

	#[inline]
	pub(crate) fn with_mimic_data(&mut self, mimic_data: joint_data::MimicBuilderData) {
		self.mimic = Some(mimic_data);
	}

	#[inline]
	pub(crate) fn with_safety_controller(
		&mut self,
		safety_controller_data: joint_data::SafetyControllerData,
	) {
		self.safety_controller = Some(safety_controller_data);
	}

	pub(crate) fn mirror(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		let (origin, new_mirror_matrix) = self.origin.mirror(mirror_matrix);
		Self {
			name: self.name.clone(), // FIXME: Rename
			joint_type: self.joint_type,
			origin,
			child: self
				.child
				.as_ref()
				.map(|link_builder| link_builder.mirror(&new_mirror_matrix)),
			axis: match (self.joint_type, self.axis) {
				(JointType::Fixed | JointType::Floating, _) => None, // TODO: Figure out if this clause should be moved down to allow for Fixed and Floating with axis if desired?
				(_, Some((x, y, z))) => Some(
					(new_mirror_matrix * vector![x, y, z])
						.normalize() // Theoretically not necessary, but float rounding errors are a thing | TODO: Figure out if this improves the situation or makes it worse
						.iter()
						.copied()
						.collect_tuple()
						.unwrap(), // Unwrapping here to ensure that we collect to a Tuple3 | TODO: Change to expect? or remove
				),
				(
					JointType::Revolute
					| JointType::Continuous
					| JointType::Prismatic
					| JointType::Planar,
					None,
				) => Some(
					(new_mirror_matrix * vector![1., 0., 0.])
						.normalize() // Theoretically not necessary, but float rounding errors are a thing | TODO: Figure out if this improves the situation or makes it worse
						.iter()
						.copied()
						.collect_tuple()
						.unwrap(), // Unwrapping here to ensure that we collect to a Tuple3 | TODO: Change to expect? or remove
				),
			},
			calibration: self.calibration,                // TODO: Is this Correct?
			dynamics: self.dynamics,                      // TODO: Is this Correct?
			limit: self.limit,                            // TODO: Is this Correct?
			mimic: self.mimic.as_ref().map(Clone::clone), // TODO: Is this Correct?
			safety_controller: self.safety_controller,    // TODO: Is this Correct?
		}
	}
}

impl BuildJoint for JointBuilder {
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_link_size_data: LinkShapeData,
	) -> ArcLock<Joint> {
		let joint = Arc::new_cyclic(|me| -> RwLock<Joint> {
			RwLock::new(Joint {
				name: self.name,
				tree: Weak::clone(&tree),
				parent_link,
				child_link,
				joint_type: self.joint_type,
				origin: self.origin.apply(parent_link_size_data),
				axis: self.axis,
				calibration: self.calibration,
				dynamics: self.dynamics,
				limit: self.limit,
				mimic: self.mimic.map(|mimic| mimic.to_mimic_data(&tree)),
				safety_controller: self.safety_controller,
				me: Weak::clone(me),
			})
		});

		tree.upgrade().unwrap().try_add_joint(&joint).unwrap(); // FIXME: Figure out if Unwrap is Ok here?
		joint
	}
}

impl BuildJointChain for JointBuilder {
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_link: &WeakLock<Link>,
		parent_shape_data: LinkShapeData,
	) -> ArcLock<Joint> {
		#[cfg(any(feature = "logging", test))]
		log::trace!("Building a Joint[name ='{}']", self.name);

		Arc::new_cyclic(|me| {
			RwLock::new(Joint {
				name: self.name,
				tree: Weak::clone(tree),
				parent_link: Weak::clone(parent_link),
				// This is Ok, since the Joint can only be attached with specific functions.
				child_link: self.child.expect("When Building Kinematic Branches Joints should have a child link, since a Joint only makes sense when attachted to a Parent and a Child").build_chain(tree, me),
				joint_type: self.joint_type,
				origin: self.origin.apply(parent_shape_data),
				axis: self.axis,
				calibration: self.calibration,
				dynamics: self.dynamics,
				limit: self.limit,
				mimic: self
					.mimic
					.map(|mimic| mimic.to_mimic_data(tree)),
				safety_controller: self.safety_controller,
				me: Weak::clone(me),
			})
		})
	}
}
