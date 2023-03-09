from enum import Enum

class KinematicTree:
    """
    TODO: KinematicTree Doc
    """

    @property
    def root_link(self) -> Link: ...
    """The root Link of the KinematicTree"""

    @property
    def newest_link(self) -> Link: ...
    """The most recently added Link"""


class Link:
    """
    Link Class documentation
    """
    @staticmethod
    def new(name: str) -> KinematicTree: ...

    @property
    def name(self) -> str: ...
    """
    Gets the Link's name
    
    :returns: the link's name
    """

    def try_attach_child(self, tree: KinematicTree, joint_builder: JointBuilder) -> None: ...
    """
    Try to attach a child tree

    :param tree: A kinemematic tree to attach to current tree
    :param joint_builder: A `JointBuilder` to connect to
    """

class JointType(Enum):
    """
    An Enum for all possible `JointTypes`
    """
    Fixed: "JointType.Fixed"
    Revolute: "JointType.Fixed"
    Continuous: "JointType.Fixed"
    Prismatic: "JointType.Fixed"
    Floating: "JointType.Fixed"
    Planar: "JointType.Fixed"


class JointBuilder:
    """
    TODO: Joint Builder Class

    :param joint_name: the name of the joint, must be unique
    :param joint_type: the type of the joint as a `JointType` 
    """
    def __init__(self, joint_name: str, joint_type: JointType) -> None: ...
