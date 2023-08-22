from enum import Enum, auto
from typing import TYPE_CHECKING, Final, NamedTuple, Optional, Tuple

# Need to use typing.Tuple instead of tuple because Python 3.8 is supported (Supported since Python 3.9)
# Need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder import Transform
    from robot_description_builder.link import Link, LinkBuilder

class Limit(NamedTuple):
    effort: float
    velocity: float
    lower: Optional[float] = None
    upper: Optional[float] = None

class JointBuilderBase:
    @property
    def name(self) -> str: ...
    joint_type: Final[JointType]
    transform: Optional[Transform]
    child: Final[Optional[LinkBuilder]]

    @property
    def axis(self) -> Optional[Tuple[float, float, float]]: ...
    @property
    def calibration(self) -> Optional[Tuple[Optional[float], Optional[float]]]:
        """TODO: Bettertype"""
        ...
    @property
    def dynamics(self) -> Optional[Tuple[Optional[float], Optional[float]]]:
        """TODO: Bettertype"""
        ...
    @property
    def limit(self) -> Optional[Limit]: ...
    @property
    def mimic(self) -> Optional[dict]:
        """TODO: Bettertype"""
        ...
    @property
    def safety_controller(self) -> Optional[dict]:
        """TODO: BetterType"""
        ...

class JointBuilder(JointBuilderBase):
    name: Final[str]
    """TODO: Final for now as a result of design"""
    axis: Optional[Tuple[float, float, float]]
    # TODO: calibration: Any # TODO: IMPLEMENT
    # TODO: dynamics: Any # TODO: IMPLEMENT
    # TODO: limit: Any # TODO: IMPLEMENT
    # TODO: mimic: Any # TODO: IMPLEMENT
    # TODO: safety_controller: Any # TODO: IMPLEMENT

    def __new__(cls, name: str, type: JointType, **kwargs) -> JointBuilder: ...
    """
    kwargs: transform, axis
    """
    def __repr__(self) -> str: ...
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...

# TODO: mark as frozen

class JointBuilderChain(JointBuilder):
    def __repr__(self) -> str: ...
    def mirror(self, axis) -> JointBuilderChain: ...

class Joint:
    name: Final[str]
    joint_type: Final[JointType]
    # TODO: ??? Tree?
    parent_link: Final[Link]
    child_link: Final[Link]
    transform: Final[Optional[Transform]]
    axis: Final[Optional[Tuple[float, float, float]]]
    # TODO: calibration: Any # TODO: IMPLEMENT
    # TODO: dynamics: Any # TODO: IMPLEMENT
    # TODO: limit: Any # TODO: IMPLEMENT
    # TODO: mimic: Any # TODO: IMPLEMENT
    # TODO: safety_controller: Any # TODO: IMPLEMENT

    def __repr__(self) -> str: ...
    def rebuild(self) -> JointBuilder: ...
    def rebuild_branch(self) -> JointBuilderChain: ...

class JointType(Enum):
    """FIXME: This does not work like build in enum but is is clear for the typechecking"""

    Fixed = auto()
    Revolute = auto()
    Continuous = auto()
    Prismatic = auto()
    Floating = auto()
    Planar = auto()
