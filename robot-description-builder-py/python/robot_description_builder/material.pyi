from typing import Final, NamedTuple, NewType, Optional, Union

class Color(NamedTuple):
    """TODO: Maybe move to actual file in order to support this"""
    r: float
    g: float
    b: float
    a: float = 1.


# TODO: Maybe move to actual file in order to support this
# Maybe this one could also be a type alias
Path = NewType("Path", str)
""""From Python > 3.10, this is a class and might be able to be used as NamedTuple like a superclass"""


class MaterialDescriptor:
    name: Optional[str]
    data: Final[Union[Color, Path]]

    # TODO: ADD __new__
    def __repr__(self) -> str: ...
