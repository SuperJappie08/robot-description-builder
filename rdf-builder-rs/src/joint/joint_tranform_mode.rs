use crate::{
	link::LinkShapeData,
	transform_data::{MirrorAxis, Transform},
};

#[derive(Debug, PartialEq, Clone)]
pub enum JointTransformMode {
	Direct(Transform),
	FigureItOut(fn(LinkShapeData) -> Transform),
}

impl From<fn(LinkShapeData) -> Transform> for JointTransformMode {
	fn from(value: fn(LinkShapeData) -> Transform) -> Self {
		Self::FigureItOut(value)
	}
}

impl JointTransformMode {
	pub(crate) fn apply(self, parent_link_data: LinkShapeData) -> Transform {
		match self {
			JointTransformMode::Direct(transform) => transform,
			JointTransformMode::FigureItOut(func) => func(parent_link_data),
		}
	}

	/// TODO: Maybe pub?
	pub(crate) fn mirrored(&self, axis: MirrorAxis) -> Self {
		match self {
			JointTransformMode::Direct(tranform) => tranform.mirrored(axis).into(),
			JointTransformMode::FigureItOut(_) => todo!(),
		}
	}
}

impl Default for JointTransformMode {
	fn default() -> Self {
		Self::Direct(Transform::default())
	}
}
