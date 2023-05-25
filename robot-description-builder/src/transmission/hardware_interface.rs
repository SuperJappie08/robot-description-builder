#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFTarget};
#[cfg(feature = "xml")]
use quick_xml::events::BytesText;

/// <http://wiki.ros.org/ros_control#Hardware_Interfaces>
///
/// This is really vague and confusion
///
/// <https://docs.ros.org/en/noetic/api/transmission_interface/html/c++/classtransmission__interface_1_1TransmissionInterfaceLoader.html#details>
/// <https://github.com/ros-controls/ros_control/wiki/hardware_interface#hardware-interfaces>
///
/// TODO: Maybe add other variant with argument
///
/// <https://github.com/ros-controls/ros_control/wiki/hardware_interface>
///
/// `gazebo_ros_control` does not support multiple HardwareInterfaces for a Joint in one transmission <https://answers.ros.org/question/235040/gazebo_ros_control-lwa4p/>
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransmissionHardwareInterface {
	/// TODO: THIS MIGHT BE A CATEGORY
	JointCommandInterface,
	/// TODO: DOCS
	/// Supported in Gazebo ROS by gazebo_ros_control [Source](https://classic.gazebosim.org/tutorials?tut=ros_control&cat=connect_ros#Defaultgazebo_ros_controlBehavior)
	EffortJointInterface,
	/// TODO: DOCS
	/// Supported in Gazebo ROS by gazebo_ros_control. It states it is not fully implemented [Source](https://classic.gazebosim.org/tutorials?tut=ros_control&cat=connect_ros#Defaultgazebo_ros_controlBehavior)
	VelocityJointInterface,
	PositionJointInterface,
	/// [**Joint State Interfaces**](http://docs.ros.org/melodic/api/hardware_interface/html/c++/classhardware__interface_1_1JointStateInterface.html): Hardware interface to support reading the state of an array of named joints, each of which has some position, velocity, and effort (force or torque).
	/// Supported in Gazebo ROS by gazebo_ros_control [Source](https://classic.gazebosim.org/tutorials?tut=ros_control&cat=connect_ros#Defaultgazebo_ros_controlBehavior)
	JointStateInterface,
	/// [**Actuator State Interfaces**](http://docs.ros.org/melodic/api/hardware_interface/html/c++/classhardware__interface_1_1ActuatorStateInterface.html): Hardware interface to support reading the state of an array of named actuators, each of which has some position, velocity, and effort (force or torque).
	/// TODO: THIS MIGHT BE A CATEGORY
	ActuatorStateInterface,
	/// Actuator Command Interfaces
	EffortActuatorInterface,
	/// Actuator Command Interfaces
	VelocityActuatorInterface,
	/// Actuator Command Interfaces
	PositionActuatorInterface,
	/// <https://github.com/ros-controls/ros_control/blob/noetic-devel/hardware_interface/include/hardware_interface/posvel_command_interface.h>
	PosVelJointInterface,
	/// <https://github.com/ros-controls/ros_control/blob/noetic-devel/hardware_interface/include/hardware_interface/posvelacc_command_interface.h>
	PosVelAccJointInterface,
	/// <https://github.com/ros-controls/ros_control/blob/noetic-devel/hardware_interface/include/hardware_interface/force_torque_sensor_interface.h>
	ForceTorqueSensorInterface,
	/// <https://github.com/ros-controls/ros_control/blob/noetic-devel/hardware_interface/include/hardware_interface/imu_sensor_interface.h>
	IMUSensorInterface,
}

impl TransmissionHardwareInterface {
	/// Gets the URDF String identifier for this `TransmissionHardwareInterface`
	///
	/// TODO:DOC: Explain about URDF Target selection
	///
	/// TODO: Might not be named inline with [convention](https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv)
	#[cfg(feature = "urdf")]
	fn as_urdf_hardware_interface_type(&self, urdf_target: URDFTarget) -> String {
		// This is because of http://wiki.ros.org/urdf/XML/Transmission#A.3Ctransmission.3E_Elements
		// However it could be possible that other `hardware_interface` providers need a different root
		let mut result = String::from(match urdf_target {
			URDFTarget::Standard => "hardware_interface/",
			URDFTarget::Gazebo => "",
		});
		match self {
			Self::JointCommandInterface => result.push_str("JointCommandInterface"),
			Self::EffortJointInterface => result.push_str("EffortJointInterface"),
			Self::VelocityJointInterface => result.push_str("VelocityJointInterface"),
			Self::PositionJointInterface => result.push_str("PositionJointInterface"),
			Self::JointStateInterface => result.push_str("JointStateInterface"),
			Self::ActuatorStateInterface => result.push_str("ActuatorStateInterface"),
			Self::EffortActuatorInterface => result.push_str("EffortActuatorInterface"),
			Self::VelocityActuatorInterface => result.push_str("VelocityActuatorInterface"),
			Self::PositionActuatorInterface => result.push_str("PositionActuatorInterface"),
			Self::PosVelJointInterface => result.push_str("PosVelJointInterface"),
			Self::PosVelAccJointInterface => result.push_str("PosVelAccJointInterface"),
			Self::ForceTorqueSensorInterface => result.push_str("ForceTorqueSensorInterface"),
			Self::IMUSensorInterface => result.push_str("IMUSensorInterface"),
		};
		result
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for TransmissionHardwareInterface {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("hardwareInterface")
			.write_text_content(BytesText::new(
				self.as_urdf_hardware_interface_type(urdf_config.urdf_target)
					.as_str(),
			))?;
		Ok(())
	}
}
