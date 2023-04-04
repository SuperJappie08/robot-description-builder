use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{joint_data, Joint, JointType},
	link::{
		builder::{BuildLink, LinkBuilder},
		Link,
	},
	transform_data::{MirrorAxis, TransformData},
	ArcLock, WeakLock,
};

pub trait BuildJoint {
	/// Creates the joint ?? and subscribes it to the right right places
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint>;
}

pub(crate) trait BuildJointChain: BuildJoint {
	fn build_chain(
		self,
		tree: &Weak<KinematicDataTree>,
		parent_link: &WeakLock<Link>,
	) -> ArcLock<Joint>;
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct JointBuilder {
	pub(crate) name: String,
	pub(crate) joint_type: JointType, // TODO: FINISH ME
	pub(crate) origin: TransformData,
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
	pub fn new<Name: Into<String>>(name: Name, joint_type: JointType) -> Self {
		Self {
			name: name.into(),
			joint_type,
			..Default::default()
		}
	}

	pub fn add_origin_offset(mut self, offset: (f32, f32, f32)) -> Self {
		self.origin.translation = Some(offset);
		self
	}

	pub fn add_origin_rotation(mut self, rotation: (f32, f32, f32)) -> Self {
		self.origin.rotation = Some(rotation);
		self
	}

	/// Nominated for Deprication
	/// Maybe Not??
	#[inline]
	pub(crate) fn with_origin(&mut self, origin: TransformData) {
		self.origin = origin;
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

	/// TODO: WIP SEE TransfromData::mirrored()
	pub fn mirrored(&self, axis: MirrorAxis) -> Self {
		JointBuilder {
			origin: self.origin.mirrored(axis),
			..self.clone()
		}
	}
}

impl BuildJoint for JointBuilder {
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let joint = Arc::new_cyclic(|me| -> RwLock<Joint> {
			RwLock::new(Joint {
				name: self.name,
				tree: Weak::clone(&tree),
				parent_link,
				child_link,
				joint_type: self.joint_type,
				origin: self.origin,
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
				origin: self.origin,
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
