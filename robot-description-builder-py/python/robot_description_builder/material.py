from typing import NamedTuple

from ._internal import Material, MaterialDescriptor


class Color(NamedTuple):
    """TODO: DOCS"""

    r: float
    g: float
    b: float
    a: float = 1.0


# TODO: Maybe move to actual file in order to support this
# Maybe this one could also be a type alias
# TexturePath = NewType("TexturePath", str)
# """"From Python > 3.10, this is a class and might be able to be used as NamedTuple like a superclass"""


class TexturePath(NamedTuple):
    """TODO: is this better? than NewType"""

    path: str

    def __repr__(self) -> str:
        return self.__class__.__name__ + f"('{self.path}')"


del NamedTuple

__all__ = ["Material", "MaterialDescriptor", "Color", "TexturePath"]
