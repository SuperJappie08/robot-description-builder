from typing import Final, TYPE_CHECKING, TypeVar, Optional 

if TYPE_CHECKING:
    from robot_description_builder import Transform
    from robot_description_builder.link.geometry import GeometryBase

Geometry = TypeVar("Geometry", bound='GeometryBase')

class CollisionBuilder[Geometry]:
    """TODO:
    """
    name: Optional[str]
    geometry: Final[Geometry]
    origin: Optional[Transform]

    def __new__(cls, geometry: Geometry, name: Optional[str] = None,
                origin: Optional[Transform] = None) -> CollisionBuilder[Geometry]: ...
    def __repr__(self) -> str: ...
    # TODO: EXPAND
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...

class Collision[Geometry]:
    """TODO:"""
    name: Final[Optional[str]]
    geometry: Final[Geometry]
    origin: Final[Optional[Transform]]
    
    def __repr__(self) -> str: ...
    # TODO: EXPAND