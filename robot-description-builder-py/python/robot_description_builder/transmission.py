"""
The transmission system is not fully correct. No Checking available as of NOW.
"""
from typing import List, NamedTuple, Optional, Union

from robot_description_builder.joint import Joint

from ._internal import (Transmission, TransmissionBuilder,
                        TransmissionHardwareInterface, TransmissionType)


class TransmissionActuator(NamedTuple):
    name: str
    mechanical_reduction: Optional[float] = None


class TransmissionJoint(NamedTuple):
    """TODO: Fine for now"""

    joint: Union[Joint, str]
    hardware_interfaces: Union[TransmissionHardwareInterface, List[TransmissionHardwareInterface]]


del (Joint, List, NamedTuple, Optional, Union)

__all__ = [
    "Transmission",
    "TransmissionBuilder",
    "TransmissionHardwareInterface",
    "TransmissionType",
    "TransmissionActuator",
    "TransmissionJoint",
]
