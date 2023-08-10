# TODO: INTERFACE INCOMPLETE
from typing import TYPE_CHECKING, Final, Optional, TypeVar

# need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder import Transform
    from robot_description_builder.material import Material, MaterialDescriptor
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
    def __repr__(self) -> str: ...
    # TODO: EXPAND
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...


class Visual[Geometry]:
    """TODO:"""
    name: Final[Optional[str]]
    geometry: Final[Geometry]
    origin: Final[Optional[Transform]]
    # TODO:
    material: Final[Optional[Material]]

    # TODO: EXPAND
    def __repr__(self) -> str: ...