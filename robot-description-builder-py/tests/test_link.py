import pytest
from robot_description_builder.link import Inertial, Link, LinkBuilder

# import robot_description_builder as rdb


def test_link_builder_new():
    link_builder = LinkBuilder("First-Name")
    assert link_builder.name == "First-Name"
    assert len(link_builder.visuals) == 0
    assert len(link_builder.colliders) == 0
    assert link_builder.inertial is None
    assert len(link_builder.joints) == 0
    assert repr(link_builder) == "LinkBuilder(\'First-Name\', joints=[])"

# def test_link_new():
#     tree = rdb.Link.new("TestName")

#     assert tree.root_link.name == tree.newest_link.name
#     assert tree.newest_link.name == "TestName"
#     raise NotImplementedError


# # In the wrong file, incomplete
# def test_jointbuilder():
#     tree = rdb.Link.new("root")
#     tree.newest_link.try_attach_child(
#         rdb.Link.new("child"),
#         rdb.JointBuilder(
#             "joint-between",
#             rdb.JointType.Fixed
#         )
#     )

#     assert tree.get_joint("joint-between").name == "joint-between"
#     raise NotImplementedError
