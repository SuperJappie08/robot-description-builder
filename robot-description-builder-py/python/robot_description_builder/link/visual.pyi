# TODO: INTERFACE INCOMPLETE
from typing import TYPE_CHECKING, Any, Final, Optional, TypeVar

if TYPE_CHECKING:
    from robot_description_builder import MaterialDescriptor, Transform
    from robot_description_builder.link.geometry import GeometryBase

Geometry = TypeVar("Geometry", bound='GeometryBase')


class VisualBuilder[Geometry]:
    """TODO:
    """
    name: Optional[str]
    geometry: Final[Geometry]
    origin: Optional[Transform]
    material: Optional[MaterialDescriptor]

    def __new__(cls, geometry: Geometry,
                name: Optional[str] = None,
                origin: Optional[Transform] = None,
                material: Optional[MaterialDescriptor] = None) -> VisualBuilder[Geometry]: ...
    
    # TODO: EXPAND


class Visual[Geometry]:
    """TODO:"""
    name: Final[Optional[str]]
    geometry: Final[Geometry]
    origin: Final[Optional[Transform]]
    # TODO:
    material: Final[Optional[Any]]

    # TODO: EXPAND