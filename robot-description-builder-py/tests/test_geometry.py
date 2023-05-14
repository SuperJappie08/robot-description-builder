from robot_description_builder.link.geometry import BoxGeometry, CylinderGeometry, SphereGeometry

# TODO: Add more tests

def test_box_geometry_repr():
    assert repr(BoxGeometry(3, 4, 6)) == "BoxGeometry(3, 4, 6)"
    assert repr(BoxGeometry(3, 4, 6.0000)) == "BoxGeometry(3, 4, 6)"
    assert repr(BoxGeometry(3, 4, 6.0009)) == "BoxGeometry(3, 4, 6.0009)"
    # TODO: EXPAND


def test_cylinder_geometry_repr():
    assert repr(CylinderGeometry(3, 4)) == "CylinderGeometry(3, 4)"
    assert repr(CylinderGeometry(3, 6.0000)) == "CylinderGeometry(3, 6)"
    assert repr(CylinderGeometry(3.33, 6.0009)
                ) == "CylinderGeometry(3.33, 6.0009)"
    # TODO: EXPAND


def test_sphere_geometry_repr():
    assert repr(SphereGeometry(3)) == "SphereGeometry(3)"
    assert repr(SphereGeometry(6.0000)) == "SphereGeometry(6)"
    assert repr(SphereGeometry(6.0009)
                ) == "SphereGeometry(6.0009)"
    # TODO: EXPAND

# Dont now how tot test easily
# def test_geometry_base_repr():
#     assert repr(BoxGeometry(9,8,7.33242)) == "BoxGeometry(9, 8, 7.33242)"
