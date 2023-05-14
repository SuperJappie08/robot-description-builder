# Interface completish
# TODO: DOCS incomplete
from typing import Final, Tuple
# Need to use typing.Tuple instead of tuple because Python 3.8 is supported (Supported since Python 3.9)


class GeometryBase:
    """TODO: DOC"""

    def volume(self) -> float: ...
    """TODO: DOC"""

    def surface_area(self) -> float: ...
    """TODO: DOC"""

    def bounding_box(self) -> Tuple[float, float, float]: ...
    """TODO: DOC"""

    def __repr__(self) -> str: ...
    def __eq__(self, other: GeometryBase) -> bool: ...
    def __neq__(self, other: GeometryBase) -> bool: ...


class BoxGeometry(GeometryBase):
    """TODO: DOC"""
    size: Tuple[float, float, float]
    """TODO: DOC"""

    def __new__(cls, side0: float, side1: float,
                side2: float) -> BoxGeometry: ...
    """TODO: DOC"""

    def __repr__(self) -> str: ...
    # Might be excessive
    def __eq__(self, other: BoxGeometry) -> bool: ...
    def __neq__(self, other: BoxGeometry) -> bool: ...



class CylinderGeometry(GeometryBase):
    """TODO: DOC"""
    radius: float
    """TODO: DOC"""
    length: float
    """TODO: DOC"""
    size: Final[Tuple[float, float]]
    """TODO: DOC"""

    def __new__(cls, radius: float, length: float) -> CylinderGeometry: ...
    """TODO: DOC"""

    def __repr__(self) -> str: ...
    # Might be excessive
    def __eq__(self, other: BoxGeometry) -> bool: ...
    def __neq__(self, other: BoxGeometry) -> bool: ...



class SphereGeometry(GeometryBase):
    """TODO: DOC"""
    radius: float
    """TODO: DOC"""

    def __new__(cls, radius: float) -> SphereGeometry: ...
    """TODO: DOC"""

    def __repr__(self) -> str: ...
    # Might be excessive
    def __eq__(self, other: BoxGeometry) -> bool: ...
    def __neq__(self, other: BoxGeometry) -> bool: ...

