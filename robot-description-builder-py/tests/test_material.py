import sys

from robot_description_builder.material import Color, MaterialDescriptor, TexturePath


def test_color_descriptor_simple():
    color = Color(0.3, 0.3, 0.3)
    descriptor = MaterialDescriptor(color)

    assert isinstance(descriptor.data, Color)
    assert abs(descriptor.data.r - descriptor.data.r) < sys.float_info.epsilon
    assert abs(descriptor.data.g - descriptor.data.g) < sys.float_info.epsilon
    assert abs(descriptor.data.b - descriptor.data.b) < sys.float_info.epsilon
    assert abs(descriptor.data.a - descriptor.data.a) < sys.float_info.epsilon
    assert descriptor.name is None


def test_texture_descriptor_simple():
    texture = TexturePath("Some Path")
    descriptor = MaterialDescriptor(texture)

    assert isinstance(descriptor.data, TexturePath)
    assert descriptor.data == texture
    assert descriptor.name is None


# TODO: Add more tests
