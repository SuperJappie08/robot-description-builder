from enum import Enum, auto
from typing import TYPE_CHECKING, Final, List, Optional, Tuple

if TYPE_CHECKING:
    from robot_description_builder.joint import Joint


class TransmissionBuilder:
    """TODO"""
    name: str
    pass


class Transmission:
    name: Final[str]
    transmission_type: Final[TransmissionType]
    # TODO:
    joints: Final[List[Tuple[Joint, List[TransmissionHardwareInterface]]]]
    actuators: Final[List[Tuple[str, Optional[float]]]]

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
