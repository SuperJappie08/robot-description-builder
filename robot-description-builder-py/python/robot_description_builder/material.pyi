from typing import Final, NamedTuple, Optional, Union

# need to use type names instead of typing.Self (supported since Python 3.11)

class Color(NamedTuple):
    """TODO: Maybe move to actual file in order to support this"""

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

    def __repr__(self) -> str: ...

class MaterialDescriptor:
    name: Optional[str]
    data: Final[Union[Color, TexturePath]]

    def __new__(cls, data: Union[Color, TexturePath], name: Optional[str]=None) -> MaterialDescriptor: ...
    def __repr__(self) -> str: ...
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...

class Material:
    name: Final[Optional[str]]
    """The name of the Material if Any"""
    data: Final[Union[Color, TexturePath]]
    """The Material data, which is either a Color or a Path"""

    def __repr__(self) -> str: ...
    def describe(self) -> MaterialDescriptor: ...
