from enum import Enum, auto
from typing import TYPE_CHECKING, Optional

# need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder.cluster_objects import Robot

class Transform:
    x: Optional[float]
    y: Optional[float]
    z: Optional[float]
    roll: Optional[float]
    pitch: Optional[float]
    yaw: Optional[float]

    def __new__(
        cls,
        x: Optional[float] = None,
        y: Optional[float] = None,
        z: Optional[float] = None,
        roll: Optional[float] = None,
        pitch: Optional[float] = None,
        yaw: Optional[float] = None,
    ) -> Transform: ...
    def __repr__(self) -> str: ...
    def __bool__(self) -> bool: ...
    def __eq__(self, other: Transform) -> bool: ...
    def __neq__(self, other: Transform) -> bool: ...

class MirrorAxis(Enum):
    X = auto()
    Y = auto()
    Z = auto()

def to_urdf_string(description: Robot, **kwargs) -> str: ...
