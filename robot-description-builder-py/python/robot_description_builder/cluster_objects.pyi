from abc import ABC
from types import MappingProxyType
from typing import TYPE_CHECKING, Final

# need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder.joint import Joint, JointBuilderChain
    from robot_description_builder.link import Link, LinkBuilder, LinkBuilderChain
    from robot_description_builder.material import Material

class KinematicBase(ABC):
    links: Final[MappingProxyType[str, Link]]
    joints: Final[MappingProxyType[str, Joint]]
    materials: Final[MappingProxyType[str, Material]]
    # TODO: Transmissions

class KinematicTree(KinematicBase):
    root_link: Final[Link]
    """The Root Link of KinematicTree. All other links are connected to this one"""
    newest_link: Final[Link]

    def to_robot(self, name: str) -> Robot: ...
    def yank_joint(self, name: str) -> JointBuilderChain: ...
    def yank_link(self, name: str) -> LinkBuilderChain: ...
    def yank_root(self) -> LinkBuilderChain: ...

class Robot(KinematicBase):
    name: Final[str]
    root_link: Final[Link]
    """The Root Link of Robot. All other links are connected to this one"""
    newest_link: Final[Link]
