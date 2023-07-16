#!../venv/bin/python3
from robot_description_builder import MirrorAxis, Transform, to_urdf_string
from robot_description_builder.joint import JointBuilder, JointType
from robot_description_builder.link import LinkBuilder, geometry
from robot_description_builder.link.visual import VisualBuilder

if __name__ == "__main__":
    tree = (
        LinkBuilder("root_link")
        .add_visual(VisualBuilder(geometry.SphereGeometry(1), name="root_viz"))
        .build()
    )

    jb = JointBuilder(
        "joint_[[1]]",
        JointType.Continuous,
        transform=Transform(x=5, roll=1, pitch=2, yaw=3),
    )

    tree.root_link.try_attach_child(
        LinkBuilder("inked_[[1]]").add_visual(
            VisualBuilder(geometry.BoxGeometry(1, 2, 3))
        ),
        jb,
    )

    chain = tree.joints["joint_[[1]]"].rebuild_branch().mirror(MirrorAxis.X)
    chain.change_group_id("2")

    tree.root_link.attach_joint_chain(chain)

    print(to_urdf_string(tree.to_robot("test")))
