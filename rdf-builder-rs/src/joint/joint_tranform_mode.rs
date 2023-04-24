use crate::{
	link::LinkShapeData,
	transform_data::{MirrorAxis, TransformData},
};

#[derive(Debug, PartialEq, Clone)]
pub enum JointTransformMode {
	Direct(TransformData),
	FigureItOut(fn(LinkShapeData) -> TransformData),
}

impl From<fn(LinkShapeData) -> TransformData> for JointTransformMode {
	fn from(value: fn(LinkShapeData) -> TransformData) -> Self {
		Self::FigureItOut(value)
	}
}

impl JointTransformMode {
	pub(crate) fn apply(self, parent_link_data: LinkShapeData) -> TransformData {
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
		Self::Direct(TransformData::default())
	}
}
