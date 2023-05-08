import pytest
import robot_description_builder as rdb


def test_link_new():
    tree = rdb.Link.new("TestName")

    assert tree.root_link.name == tree.newest_link.name
    assert tree.newest_link.name == "TestName"
    raise NotImplementedError


# In the wrong file, incomplete
def test_jointbuilder():
    tree = rdb.Link.new("root")
    tree.newest_link.try_attach_child(
        rdb.Link.new("child"),
        rdb.JointBuilder(
            "joint-between",
            rdb.JointType.Fixed
        )
    )

    assert tree.get_joint("joint-between").name == "joint-between"
    raise NotImplementedError
