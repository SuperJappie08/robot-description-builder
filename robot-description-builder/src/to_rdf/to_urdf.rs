//! The infrastructure to describe a `Robot` in the Universal Robot Description Format (URDF).
// TODO: EXPAND Module doc?, Matbe not

use std::io::Cursor;

use quick_xml::{
	events::{BytesDecl, Event},
	Writer,
};

use super::{make_xml_writer, XMLMode};
use crate::cluster_objects::KinematicInterface;

// FIXME: FIX CONFIG, MAYBE MAKE AN INTERNAL CONFIG TYPE
/// A Configuration for the exporting of the description in the [URDF](http://wiki.ros.org/urdf) format.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct URDFConfig {
	/// Determines the way all `Material`s are displayed.
	///
	/// For example either referenced or inline under specific conditions.
	pub material_references: URDFMaterialReferences,
	/// Determines how the current `Material` is displayed, internal use only.
	///
	/// This field is overwritten by the logic of [`material_references`](URDFConfig).
	pub direct_material_ref: URDFMaterialMode,
	/// Determines the Target URDF-parser specification variant.
	pub urdf_target: URDFTarget,
	/// Determines the XML style.
	pub xml_mode: XMLMode,
}

/// Determines how Referencable/Named `Material`s should be written.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum URDFMaterialReferences {
	/// Write all named [`Material`s](crate::material::Material) at the top of the document and refer to them when used even if they are only used once.
	/// This is the default.
	///
	/// This mode is recommened to be used, since it makes it easy to change materials slightly by hand after the fact.
	#[default]
	AllNamedMaterialOnTop,
	/// Only [`Material`s](crate::material::Material), that are used more then once, will be displayed at the top of the description.
	/// These `Material`s are written as references where used.
	///
	/// Other [`Material`s](crate::material::Material) (Unnamed and Named, but used once) will be written fully inside of the [`Link`](crate::link::Link).
	OnlyMultiUseMaterials,
	/// Always writes the full [`Material`](crate::material::Material) in the [`Link`](crate::link::Link). No matter if it is referenceable/named or not.
	///
	/// Therefor no materials will be written at the top.
	AlwaysInline,
}

/// Determines how the current `Material` is displayed. Can only be used internally.
///
/// This field is overwritten by logic controlled by [`URDFMaterialReferences`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum URDFMaterialMode {
	/// Display the `Material` fully, with content. This is the default.
	#[default]
	FullMaterial,
	/// Display the `Material` as a reference.
	Referenced,
}

/// A way to specify a target URDF reader.
///
/// This is needed, since not all URDF-Parser are created equally.
/// They can have minor structuring preferences, which can be respected by the use of this Enum.
///
/// Currently, this only changes Transmission styles.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum URDFTarget {
	/// The standard URDF configuration as specified by the [URDF specification](http://wiki.ros.org/urdf/XML). This is the default.
	///
	/// This is compatible with [ROS Control](http://wiki.ros.org/ros_control)'s RobotHW way of [Hardware interfaces](crate::transmission::TransmissionHardwareInterface) for a [`Transmission`](crate::transmission::Transmission).
	#[default]
	Standard,
	// Not fully implemented yet
	/// A URDF configuration for [Gazebo Simulator](https://gazebosim.org/home).
	///
	/// This specifies [Hardware interfaces](crate::transmission::TransmissionHardwareInterface) for a [`Transmission`](crate::transmission::Transmission) in the Gazebo style.
	Gazebo,
}

/// A trait to allow parts of a `Robot` to be described in the URDF format.
pub trait ToURDF {
	/// Represents the element as in URDF format.
	fn to_urdf(
		&self,
		writer: &mut Writer<Cursor<Vec<u8>>>,
		urdf_config: &URDFConfig,
	) -> Result<(), quick_xml::Error>;
}

/// A function to represent a `KinematicInterface` implementor in the URDF format.
///
/// This function should be used to generate the descriptions.
///
/// # Example
/// Reads and writes are hidden for brevity.
/// ```
/// # use robot_description_builder::{
/// #     link_data::{geometry::*, Visual},
/// #     material::MaterialDescriptor,
/// #     prelude::*,
/// #     to_rdf::{
/// #       to_urdf::{to_urdf, URDFConfig},
/// #       xml_writer_to_string, XMLMode
/// #     },
/// #     Link, Robot, SmartJointBuilder, Transform,
/// # };
/// #
/// let white_material = MaterialDescriptor::new_rgb(1., 1., 1.).named("white");
///
/// let right_leg_link = Link::builder("[\\[right]\\]_leg").add_visual(
///     Visual::builder(BoxGeometry::new(0.6, 0.1, 0.2))
///     .materialized(white_material.clone())
///     .transformed(Transform::new_translation(0., 0., -0.3)),
/// );
///
/// let right_leg: Robot = right_leg_link.build_tree().to_robot("Right_Leg_bot");
///
/// let right_base_link = Link::builder("[\\[right]\\]_base")
///     .add_visual(Visual::builder(BoxGeometry::new(0.4, 0.1, 0.1)).materialized(white_material));
///
/// let right_base_joint = SmartJointBuilder::new_fixed("[\\[right]\\]_base_joint")
///     .add_transform(Transform::new_translation(0., 0., -0.6));
///
/// right_leg
///     .get_root_link()
///     .write()
///     .unwrap()
///     .try_attach_child(right_base_joint, right_base_link)
///     .unwrap();
///
/// assert_eq!(
/// xml_writer_to_string(
///     to_urdf(
///         &right_leg,
///         URDFConfig{
///             xml_mode: XMLMode::Indent(' ', 2),
///             ..Default::default()
///     }).unwrap()),
/// r#"﻿<?xml version="1.0"?>
/// <robot name="Right_Leg_bot">
///   <material name="white">
///     <color rgba="1 1 1 1"/>
///   </material>
///   <link name="[[right]]_leg">
///     <visual>
///       <origin xyz="0 0 -0.3"/>
///       <geometry>
///         <box size="0.6 0.1 0.2"/>
///       </geometry>
///       <material name="white"/>
///     </visual>
///   </link>
///   <joint name="[[right]]_base_joint" type="fixed">
///     <origin xyz="0 0 -0.6"/>
///     <parent link="[[right]]_leg"/>
///     <child link="[[right]]_base"/>
///   </joint>
///   <link name="[[right]]_base">
///     <visual>
///       <geometry>
///         <box size="0.4 0.1 0.1"/>
///       </geometry>
///       <material name="white"/>
///     </visual>
///   </link>
/// </robot>"#
/// )
/// ```
pub fn to_urdf(
	tree: &(impl KinematicInterface + ToURDF),
	urdf_config: URDFConfig,
) -> Result<Writer<Cursor<Vec<u8>>>, quick_xml::Error> {
	let mut writer = make_xml_writer(urdf_config.xml_mode);

	writer.write_bom()?;
	writer.write_event(&Event::Decl(BytesDecl::new("1.0", None, None)))?;
	tree.to_urdf(&mut writer, &urdf_config)?;
	Ok(writer)
}

// This does not work due to ElementWriter.write_inner() expecting a closure that returns `quick_xml::Error`
// /// TODO DOCS
// /// TODO DOES THIS COMPLY WITH THE NAMING CONVENTION
// /// THIS DOES NOT WORK DO TO CLOSURES
// #[derive(Debug, Error)]
// pub enum ToURDFError {
// 	#[error(transparent)]
// 	XML(#[from] quick_xml::Error),
// }
