import pytest
from robot_description_builder import Transform
from robot_description_builder.cluster_objects import KinematicTree
from robot_description_builder.joint import JointBuilder, JointType
from robot_description_builder.link import LinkBuilder
from robot_description_builder.link.geometry import BoxGeometry, SphereGeometry
from robot_description_builder.link.visual import VisualBuilder


def test_build():
    tree = LinkBuilder("some_link_name").build()
    assert type(tree) == KinematicTree

    assert tree.joints == dict()
    assert tree.materials == dict()
    # TODO: Expand with Transmission

    assert set(tree.links.keys()) == {"some_link_name"}

    assert tree.newest_link.name == "some_link_name"
    assert tree.newest_link.parent == tree

    assert tree.root_link.name == "some_link_name"
    assert tree.root_link.parent == tree
    assert tree.root_link.joints == []
    assert tree.root_link.visuals == []
    assert tree.root_link.colliders == []
    assert tree.root_link.inertial == None

    assert tree.root_link == tree.newest_link


def test_update_index_after_attach():
    tree = (
        LinkBuilder("Linky")
        .add_visual(VisualBuilder(SphereGeometry(1), name="test"))
        .build()
    )

    links = tree.links
    joints = tree.joints
    assert set(links.keys()) == {"Linky"}
    assert set(joints.keys()) == set()

    jb = JointBuilder(
        "Some [[Joint]]",
        JointType.Fixed,
        transform=Transform(x=5, roll=1, pitch=2, yaw=3),
    )

    tree.root_link.try_attach_child(
        jb,
        LinkBuilder("This is a [[test]]").add_visual(
            VisualBuilder(
                BoxGeometry(1, 4, 5), name="visual_[[test]]", transform=Transform(1)
            )
        ),
    )

    assert set(links.keys()) == {"Linky", "This is a [[test]]"}
    assert set(joints.keys()) == {"Some [[Joint]]"}

    assert tree.root_link == links["Linky"]
    assert tree.newest_link == links["This is a [[test]]"]
