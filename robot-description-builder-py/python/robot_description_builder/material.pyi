from typing import Final, NamedTuple, Optional, Union


class Color(NamedTuple):
    """TODO: Maybe move to actual file in order to support this"""
    r: float
    g: float
    b: float
    a: float = 1.


# TODO: Maybe move to actual file in order to support this
# Maybe this one could also be a type alias
# TexturePath = NewType("TexturePath", str)
# """"From Python > 3.10, this is a class and might be able to be used as NamedTuple like a superclass"""
class TexturePath(NamedTuple):
    """TODO: is this better? than NewType"""
    path: str

    def __repr__(self) -> str: ...


class MaterialDescriptor:
    name: Optional[str]
    data: Final[Union[Color, TexturePath]]

    # TODO: ADD __new__
    def __repr__(self) -> str: ...


class Material:
    name: Final[Optional[str]]
    """The name of the Material if Any"""
    data: Final[Union[Color, TexturePath]]
    """The Material data, which is either a Color or a Path"""

    def __repr__(self) -> str: ...
    def describe(self) -> MaterialDescriptor: ...
