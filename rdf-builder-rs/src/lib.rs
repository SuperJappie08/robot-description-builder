mod cluster_objects;
mod joint;
mod link;
mod material;
pub mod to_rdf;
mod transform_data;

type ArcLock<T> = std::sync::Arc<std::sync::RwLock<T>>;
type WeakLock<T> = std::sync::Weak<std::sync::RwLock<T>>;

pub use cluster_objects::{KinematicInterface, KinematicTree, Robot};
pub use joint::{Joint, JointBuilder, JointInterface, JointType, OffsetMode, SmartJointBuilder};
pub use link::{helper_functions, link_data, Link};
pub use material::{Material, MaterialData};
use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};
use to_rdf::to_urdf::ToURDF;
pub use transform_data::TransformData;

#[derive(Debug, PartialEq, Eq)]
/// TODO: IMPLEMENT PROPPERLY, THIS IS TEMPORARY
pub struct Transmission {
	pub name: String,
}

impl ToURDF for Transmission {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("transmission")
			.with_attribute(Attribute {
				key: QName(b"name"),
				value: self.name.clone().as_bytes().into(),
			})
			.write_text_content(BytesText::new("<!-- TODO: TRANSMISSIONS -->"))?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	// use super::*;

	// #[test]
	// fn it_works() {
	// 	let result = add(2, 2);
	// 	assert_eq!(result, 4);
	// }
}
