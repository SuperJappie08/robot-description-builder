use std::sync::{Arc, RwLock, Weak};

use itertools::Itertools;
use nalgebra::{vector, Matrix3};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	identifiers::GroupIDChanger,
	joint::{joint_data, joint_tranform_mode::JointTransformMode, Joint, JointType},
	link::{
		builder::{BuildLink, LinkBuilder},
		Link, LinkShapeData,
	},
	transform::{Mirror, MirrorUpdater, Transform},
	utils::{ArcLock, WeakLock},
};

pub trait BuildJoint: Into<JointBuilder> {
	// Creates the joint ?? and subscribes it to the right right places
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_shape_data: LinkShapeData,
	) -> ArcLock<Joint>;
}

// NOTE: Removed Trait bound due for `Chained<JointBuilder>`
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
	pub(crate) joint_type: JointType,
	// TODO: Maybe add a Figure it out???
	/// The transform from the origin of the parent to the origin of this `JointBuilder`.
	///
	/// In URDF this field is refered to as `<origin>`.
	pub(crate) transform: JointTransformMode,
	pub(crate) child: Option<LinkBuilder>,

	//Consider making everything below pub to remove need for all the functions
	// TODO: MAYBE CHANGE TO Vec3D Or something
	pub(crate) axis: Option<(f32, f32, f32)>,
	pub(crate) calibration: joint_data::CalibrationData,
	pub(crate) dynamics: joint_data::DynamicsData,
	pub(crate) limit: Option<joint_data::LimitData>,
	pub(crate) mimic: Option<joint_data::MimicBuilderData>,
	pub(crate) safety_controller: Option<joint_data::SafetyControllerData>,
}

// TODO: maybe add new_full ? or make fields public
impl JointBuilder {
	pub fn new(name: impl Into<String>, joint_type: JointType) -> Self {
		Self {
			name: name.into(),
			joint_type,
			..Default::default()
		}
	}

	// TODO: rename transform
	pub fn add_origin_offset(mut self, offset: (f32, f32, f32)) -> Self {
		match &mut self.transform {
			JointTransformMode::Direct(transform) => transform.translation = Some(offset),
			JointTransformMode::FigureItOut(_) => todo!("Don't know how to do this"),
		};
		self
	}

	// TODO: rename transform
	pub fn add_origin_rotation(mut self, rotation: (f32, f32, f32)) -> Self {
		match &mut self.transform {
			JointTransformMode::Direct(tranform) => tranform.rotation = Some(rotation),
			JointTransformMode::FigureItOut(_) => todo!("Don't know how to do this yet"),
		}
		self
	}

	pub fn set_transform_simple(&mut self, transform: Transform) {
		self.transform = JointTransformMode::Direct(transform);
	}

	// Nominated for Deprication
	// Maybe Not??
	#[inline]
	pub(crate) fn with_transform<JTM>(&mut self, transform: JTM)
	where
		JTM: Into<JointTransformMode>,
	{
		self.transform = transform.into();
	}

	// TODO: HAS A CONFUSING NAME WITH SmartJointBuilder::with_axis, which consumes
	/// Add the [`axis`](JointBuilder::axis) to the `JointBuillder`.
	#[inline]
	pub fn with_axis(&mut self, axis: (f32, f32, f32)) {
		self.axis = Some(axis);
	}

	/// Add the full [`CalibrationData`](joint_data::CalibrationData) to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_calibration_data(&mut self, calibration_data: joint_data::CalibrationData) {
		self.calibration = calibration_data;
	}

	/// Add the full [`DynamicsData`](joint_data::DynamicsData) to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_dynamics_data(&mut self, dynamics_data: joint_data::DynamicsData) {
		self.dynamics = dynamics_data;
	}

	/// Add the full [`LimitData`](joint_data::LimitData) to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_limit_data(&mut self, limit_data: joint_data::LimitData) {
		self.limit = Some(limit_data);
	}

	/// Add the full [`MimicData`](joint_data::MimicData) to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_mimic_data(&mut self, mimic_data: joint_data::MimicBuilderData) {
		self.mimic = Some(mimic_data);
	}

	/// Add the full [`SafetyControllerData`](joint_data::SafetyControllerData) to the `JointBuillder`.
	#[inline]
	pub(crate) fn with_safety_controller(
		&mut self,
		safety_controller_data: joint_data::SafetyControllerData,
	) {
		self.safety_controller = Some(safety_controller_data);
	}
}

// Mostly for Python Wrapper
/// Non Builder functions
impl JointBuilder {
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn joint_type(&self) -> &JointType {
		&self.joint_type
	}

	pub fn transform(&self) -> Option<&Transform> {
		// TODO: Maybe add a advanced mode for figure it out?
		match &self.transform {
			JointTransformMode::Direct(transform) => match transform.contains_some() {
				true => Some(transform),
				false => None,
			},
			JointTransformMode::FigureItOut(_) => todo!("?"),
		}
	}

	pub fn transform_mut(&mut self) -> Option<&mut Transform> {
		// TODO: Maybe add a advanced mode for figure it out?
		match &mut self.transform {
			JointTransformMode::Direct(transform) => match transform.contains_some() {
				true => Some(transform),
				false => None,
			},
			JointTransformMode::FigureItOut(_) => todo!("?"),
		}
	}

	// TODO: Transform

	pub fn child(&self) -> Option<&LinkBuilder> {
		self.child.as_ref()
	}

	pub fn axis(&self) -> Option<(f32, f32, f32)> {
		self.axis
	}

	pub fn axis_mut(&mut self) -> Option<&mut (f32, f32, f32)> {
		self.axis.as_mut()
	}

	pub fn calibration(&self) -> &joint_data::CalibrationData {
		&self.calibration
	}

	pub fn calibration_mut(&mut self) -> &mut joint_data::CalibrationData {
		&mut self.calibration
	}

	pub fn dynamics(&self) -> &joint_data::DynamicsData {
		&self.dynamics
	}

	pub fn dynamics_mut(&mut self) -> &mut joint_data::DynamicsData {
		&mut self.dynamics
	}

	pub fn limit(&self) -> Option<&joint_data::LimitData> {
		self.limit.as_ref()
	}

	pub fn limit_mut(&mut self) -> &mut Option<joint_data::LimitData> {
		&mut self.limit
	}

	pub fn mimic(&self) -> Option<&joint_data::MimicBuilderData> {
		self.mimic.as_ref()
	}

	pub fn mimic_mut(&mut self) -> &mut Option<joint_data::MimicBuilderData> {
		&mut self.mimic
	}

	pub fn safety_controller(&self) -> Option<&joint_data::SafetyControllerData> {
		self.safety_controller.as_ref()
	}

	pub fn safety_controller_mut(&mut self) -> &mut Option<joint_data::SafetyControllerData> {
		&mut self.safety_controller
	}
}

impl Mirror for JointBuilder {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		let (transform, new_mirror_matrix) = self.transform.mirrored_update_matrix(mirror_matrix);
		Self {
			name: self.name.clone(), // FIXME: Rename
			joint_type: self.joint_type,
			transform,
			child: self
				.child
				.as_ref()
				.map(|link_builder| link_builder.mirrored(&new_mirror_matrix)),
			axis: match (self.joint_type, self.axis) {
				(JointType::Fixed | JointType::Floating, _) => None, // TODO: Figure out if this clause should be moved down to allow for Fixed and Floating with axis if desired?
				(_, Some((x, y, z))) => Some(
					(new_mirror_matrix * vector![x, y, z] * -1.)
						// Theoretically not necessary, but float rounding errors are a thing | TODO: Figure out if this improves the situation or makes it worse
						.normalize()
						.iter()
						.copied()
						.collect_tuple()
						// Unwrapping here to ensure that we collect to a Tuple3 | TODO: Change to expect? or remove
						.unwrap(),
				),
				(
					JointType::Revolute
					| JointType::Continuous
					| JointType::Prismatic
					| JointType::Planar,
					None,
				) => Some(
					(new_mirror_matrix * vector![1., 0., 0.] * -1.)
						.normalize() // Theoretically not necessary, but float rounding errors are a thing | TODO: Figure out if this improves the situation or makes it worse
						.iter()
						.copied()
						.collect_tuple()
						.unwrap(), // Unwrapping here to ensure that we collect to a Tuple3 | TODO: Change to expect? or remove
				),
			},
			calibration: self.calibration,             // TODO: Is this Correct?
			dynamics: self.dynamics,                   // TODO: Is this Correct?
			limit: self.limit,                         // TODO: Is this Correct?
			mimic: self.mimic.as_ref().cloned(),       // TODO: Is this Correct?
			safety_controller: self.safety_controller, // TODO: Is this Correct?
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
				transform: self.transform.apply(parent_link_size_data),
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
				transform: self.transform.apply(parent_shape_data),
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

impl GroupIDChanger for JointBuilder {
	unsafe fn change_group_id_unchecked(&mut self, new_group_id: &str) {
		self.name.change_group_id_unchecked(new_group_id);

		if let Some(link_builder) = self.child.as_mut() {
			link_builder.change_group_id_unchecked(new_group_id);
		}
	}

	fn apply_group_id(&mut self) {
		self.name.apply_group_id();

		if let Some(link_builder) = self.child.as_mut() {
			link_builder.apply_group_id();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{JointBuilder, JointType};
	use test_log::test;

	mod group_id_changer {
		use super::{test, JointBuilder, JointType};
		use crate::identifiers::{GroupIDChanger, GroupIDError};

		#[test]
		fn change_group_id_unchecked_simple() {
			#[inline]
			fn test(
				name: impl Into<String>,
				joint_type: JointType,
				new_group_id: &str,
				result: &str,
			) {
				let mut joint_builder = JointBuilder::new(name, joint_type);
				unsafe {
					joint_builder.change_group_id_unchecked(new_group_id);
				}
				assert_eq!(joint_builder.name, result)
			}

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"C10df",
				"leg_[[C10df]]_joint_1",
			);

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"",
				"leg_[[]]_joint_1",
			);

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"[[tsst",
				"leg_[[[[tsst]]_joint_1",
			);

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"tsst]]",
				"leg_[[tsst]]]]_joint_1",
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn test_change_group_id_unchecked_advanced() {
			todo!("Chained things")
		}

		#[test]
		fn change_group_id_simple() {
			#[inline]
			fn test(
				name: impl Into<String>,
				joint_type: JointType,
				group_id: &str,
				change_result: Result<(), GroupIDError>,
				final_name: &str,
			) {
				let mut joint_builder = JointBuilder::new(name, joint_type);
				assert_eq!(joint_builder.change_group_id(group_id), change_result);
				assert_eq!(joint_builder.name, final_name)
			}

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"C10df",
				Ok(()),
				"leg_[[C10df]]_joint_1",
			);
			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"",
				Err(GroupIDError::new_empty()),
				"leg_[[M09da]]_joint_1",
			);
			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"[[tsst",
				Err(GroupIDError::new_open("[[tsst")),
				"leg_[[M09da]]_joint_1",
			);
			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"tsst]]",
				Err(GroupIDError::new_close("tsst]]")),
				"leg_[[M09da]]_joint_1",
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn change_group_id_advanced() {
			todo!()
		}

		#[test]
		fn apply_group_id_simple() {
			#[inline]
			fn test(name: impl Into<String>, joint_type: JointType, final_name: &str) {
				let mut joint_builder = JointBuilder::new(name, joint_type);
				joint_builder.apply_group_id();
				assert_eq!(joint_builder.name, final_name)
			}

			test(
				"leg_[[M09da]]_joint_1",
				JointType::Fixed,
				"leg_M09da_joint_1",
			);
			test(
				"leg_[[M09daf_joint_1",
				JointType::Fixed,
				"leg_[[M09daf_joint_1",
			);
			test(
				"leg_sM09da]]_joint_1",
				JointType::Fixed,
				"leg_sM09da]]_joint_1",
			);
			test(
				"leg_[\\[M09da]\\]_joint_1",
				JointType::Fixed,
				"leg_[[M09da]]_joint_1",
			);
		}

		#[test]
		#[ignore = "TODO"]
		fn apply_group_id_advanced() {
			todo!()
		}
	}
}
