use nalgebra::Matrix3;

use crate::{
	link::LinkShapeData,
	transform::{Mirror, MirrorUpdater, Transform},
};

#[derive(Debug, PartialEq, Clone)]
pub enum JointTransformMode {
	Direct(Transform),
	FigureItOut(fn(LinkShapeData) -> Transform),
}

impl JointTransformMode {
	pub(crate) fn apply(self, parent_link_data: LinkShapeData) -> Transform {
		match self {
			JointTransformMode::Direct(transform) => transform,
			JointTransformMode::FigureItOut(func) => func(parent_link_data),
		}
	}
}

impl Mirror for JointTransformMode {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		match self {
			JointTransformMode::Direct(transform) => {
				Self::Direct(transform.mirrored(mirror_matrix))
			}
			JointTransformMode::FigureItOut(_) => todo!("I do not know how to do this yet."),
		}
	}
}

impl MirrorUpdater for JointTransformMode {
	fn update_mirror_matrix(&self, mirror_matrix: &Matrix3<f32>) -> Matrix3<f32> {
		match self {
			JointTransformMode::Direct(transform) => transform.update_mirror_matrix(mirror_matrix),
			JointTransformMode::FigureItOut(_) => todo!("I do not know how to do this yet."),
		}
	}

	fn mirrored_update_matrix(&self, mirror_matrix: &Matrix3<f32>) -> (Self, Matrix3<f32>) {
		match self {
			JointTransformMode::Direct(transform) => {
				let (new_transform, new_mirror_matrix) =
					transform.mirrored_update_matrix(mirror_matrix);
				(Self::Direct(new_transform), new_mirror_matrix)
			}
			JointTransformMode::FigureItOut(_) => todo!("I do not know how to do this yet."),
		}
	}
}

impl From<fn(LinkShapeData) -> Transform> for JointTransformMode {
	fn from(value: fn(LinkShapeData) -> Transform) -> Self {
		Self::FigureItOut(value)
	}
}

impl Default for JointTransformMode {
	fn default() -> Self {
		Self::Direct(Transform::default())
	}
}
