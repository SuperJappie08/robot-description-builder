from typing import Final, TYPE_CHECKING, TypeVar, Optional 

# need to use type names instead of typing.Self (supported since Python 3.11)

if TYPE_CHECKING:
    from robot_description_builder import Transform
    from robot_description_builder.link.geometry import GeometryBase
    from robot_description_builder.link.visual import VisualBuilder

Geometry = TypeVar("Geometry", bound='GeometryBase')

class CollisionBuilder[Geometry]:
    """TODO:
    """
    name: Optional[str]
    geometry: Final[Geometry]
    transform: Optional[Transform]

    def __new__(cls, geometry: Geometry, name: Optional[str] = None,
                transform: Optional[Transform] = None) -> CollisionBuilder[Geometry]: ...
    def __repr__(self) -> str: ...
    # TODO: EXPAND
    def as_visual(self) -> VisualBuilder[Geometry]:
        """Creates a :class:`robot_description_builder.link.visual.VisualBuilder` from this ``CollisionBuilder``.

        :return: A :class:`robot_description_builder.link.visual.VisualBuilder` with the data from this ``CollisionBuilder``
        :rtype: :class:`robot_description_builder.link.visual.VisualBuilder`
        """
        ...
    def change_group_id(self, new_group_id: str) -> None: ...
    def apply_group_id(self) -> None: ...

class Collision[Geometry]:
    """TODO:"""
    name: Final[Optional[str]]
    geometry: Final[Geometry]
    transform: Final[Optional[Transform]]
    
    def __repr__(self) -> str: ...
    # TODO: EXPAND