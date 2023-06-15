from typing import TYPE_CHECKING, Final, List, Optional, Union

# Need to use typing.List instead of list because Python 3.8 is supported (Supported since Python 3.9)
# Need to use typing.Union instead `|` because `|` (Supported since Python 3.10)

if TYPE_CHECKING:
    from robot_description_builder.cluster_objects import KinematicTree
    from robot_description_builder.joint import Joint, JointBuilder, JointBuilderChain
    from robot_description_builder.link.collision import (Collision,
                                                          CollisionBuilder)
    from robot_description_builder.link.visual import Visual, VisualBuilder


class Inertial:
    """TODO"""
    pass


class LinkBuilder:
    """TODO"""
    name: Final[str]  # Final for now
    """The name of the `Link(Builder)` must be unique, checked when attaching to a `KinematicTree`"""
    visuals: Final[List[VisualBuilder]]
    colliders: Final[List[CollisionBuilder]]
    # Fix Joint loss when setting to None
    inertial: Optional[Inertial]
    joints: Final[List[JointBuilder]]

    def __new__(cls, name: str) -> LinkBuilder: ...
    def __repr__(self) -> str: ...
    # NOTE: Both inplace and chainable
    def add_visual(self, visual_builder: VisualBuilder) -> LinkBuilder: ...
    def add_collider(self, collision_builder: CollisionBuilder) -> LinkBuilder: ...
    def add_inertial(self, inertial: Inertial) -> LinkBuilder: ...

    def build(self) -> KinematicTree: ...
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...

class LinkBuilderChain(LinkBuilder):
    def __repr__(self) -> str: ...
    def mirror(self, axis) -> LinkBuilderChain: ...

class Link:
    name: Final[str]
    parent: Final[Union[Joint, KinematicTree]]
    visuals: Final[List[Visual]]
    colliders: Final[List[Collision]]
    inertial: Final[Optional[Inertial]]

    def __repr__(self) -> str: ...
    def try_attach_child(self, link_builder: LinkBuilder,
                         joint_builder: JointBuilder) -> None: ...
    def attach_joint_chain(self, joint_chain: JointBuilderChain) -> None: ...
    """
    TODO: Maybe merge with attach_joint_chain_at
    """
    def rebuild(self) -> LinkBuilder: ...
    def rebuild_branch(self) -> LinkBuilderChain: ...
    
