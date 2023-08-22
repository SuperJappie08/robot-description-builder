import pytest
from robot_description_builder import Transform
from robot_description_builder.link import geometry
from robot_description_builder.link.visual import Visual, VisualBuilder


def test_visual_builder():
    builder = VisualBuilder(geometry.BoxGeometry(1, 2+3, 3))
    assert builder.name == None
    assert builder.geometry == geometry.BoxGeometry(1, 2+3, 3)
    assert builder.transform == None
    assert builder.material == None


def test_visual_builder_full():
    # TODO: Material
    builder = VisualBuilder(geometry.BoxGeometry(
        1, 2+3, 3), name="Visual-Thing", transform=Transform(roll=3.1, pitch=0.987, yaw=10.2))
    assert builder.name == "Visual-Thing"
    assert builder.geometry == geometry.BoxGeometry(1, 2+3, 3)
    assert builder.transform == Transform(roll=3.1, pitch=0.987, yaw=10.2)
    assert builder.material == None
