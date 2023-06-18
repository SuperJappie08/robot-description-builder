from typing import NamedTuple, Optional

from ._internal import (Joint, JointBuilder, JointBuilderBase,
                        JointBuilderChain, JointType)


class Limit(NamedTuple):
    """A representation of Limit data. NOTE: Maybe not final.
    """
    effort: float
    velocity: float
    lower: Optional[float] = None
    upper: Optional[float] = None

del NamedTuple, Optional

__all__ = [
    "Joint",
    "JointBuilder",
    "JointBuilderBase",
    "JointBuilderChain",
    "JointType",
    "Limit",
]
