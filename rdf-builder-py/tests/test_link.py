import pytest
import rdf_builder_py


def test_link_new():
    tree = rdf_builder_py.Link.new("TestName")

    assert tree.root_link.name == tree.newest_link.name
    assert tree.newest_link.name == "TestName"
    raise NotImplementedError


# In the wrong file, incomplete
def test_jointbuilder():
    tree = rdf_builder_py.Link.new("root")
    tree.newest_link.try_attach_child(
        rdf_builder_py.Link.new("child"),
        rdf_builder_py.JointBuilder(
            "joint-between",
            rdf_builder_py.JointType.Fixed
        )
    )

    assert tree.get_joint("joint-between").name == "joint-between"
    raise NotImplementedError
