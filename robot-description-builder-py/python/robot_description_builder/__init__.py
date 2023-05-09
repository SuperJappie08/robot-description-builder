from ._internal import KinematicTree, Joint, JointBuilder, JointType, Transform, Robot, MaterialDescriptor
from . import link

__doc__ = _internal.__doc__
if hasattr(_internal, "__all__"):
    __all__ = _internal.__all__

# from . import _robot_description_builder

# import _robot_description_builder.link.LinkBuilder

# del _robot_description_builder
# # from robot_description_builder._robot_description_builder import *

# # __doc__ = robot_description_builder.__doc__
# # if hasattr(robot_description_builder, "__all__"):
# #     __all__ = robot_description_builder.__all__