"""
The transmission system is not fully correct. No Checking available as of NOW.
"""
from enum import Enum, auto
from typing import TYPE_CHECKING, Final, List, NamedTuple, Optional, Union

# need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder.joint import Joint

class TransmissionActuator(NamedTuple):
    name: str
    mechanical_reduction: Optional[float]

class TransmissionJoint(NamedTuple):
    joint: Union[Joint, str]
    """Only union for construction,"""
    hardware_interfaces: Union[TransmissionHardwareInterface, List[TransmissionHardwareInterface]]

class TransmissionBuilder:
    """TODO"""

    name: str
    type: TransmissionType
    pass

class Transmission:
    name: Final[str]
    transmission_type: Final[TransmissionType]
    joints: Final[List[TransmissionJoint]]
    actuators: Final[List[TransmissionActuator]]

    def __repr__(self) -> str: ...

class TransmissionType(Enum):
    """FIXME: This does not work like build in enum but is is clear for the typechecking"""

    SimpleTransmission = auto()
    DifferentialTransmission = auto()
    FourBarLinkageTransmission = auto()

class TransmissionHardwareInterface(Enum):
    """FIXME: This does not work like build in enum but is is clear for the typechecking"""

    JointCommandInterface = auto()
    EffortJointInterface = auto()
    VelocityJointInterface = auto()
    PositionJointInterface = auto()
    JointStateInterface = auto()
    ActuatorStateInterface = auto()
    EffortActuatorInterface = auto()
    VelocityActuatorInterface = auto()
    PositionActuatorInterface = auto()
    PosVelJointInterface = auto()
    PosVelAccJointInterface = auto()
    ForceTorqueSensorInterface = auto()
    IMUSensorInterface = auto()
