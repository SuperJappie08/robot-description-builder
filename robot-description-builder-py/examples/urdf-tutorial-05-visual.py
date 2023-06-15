#!../venv/bin/python3
from math import pi

import robot_description_builder as rdb
from robot_description_builder import Transform, MirrorAxis
from robot_description_builder.joint import JointBuilder, JointType
from robot_description_builder.link import LinkBuilder
from robot_description_builder.link.geometry import BoxGeometry, CylinderGeometry
from robot_description_builder.link.visual import VisualBuilder
from robot_description_builder.material import Color, MaterialDescriptor


def main():
    # ==== Material Descriptions ==== #
    blue_material = MaterialDescriptor(Color(0, 0, 0.8), "blue")
    black_material = MaterialDescriptor(Color(0, 0, 0), "black")
    white_material = MaterialDescriptor(Color(1, 1, 1), "white")

    # =========== Step 1 ============ #
    base_link = LinkBuilder("base_link").add_visual(
        VisualBuilder(CylinderGeometry(0.2, 0.6), material=blue_material)
    )

    model = base_link.build().to_robot("visual")

    # ======= Start rigth leg ======= #
    right_leg_link = LinkBuilder("[\\[right]\\]_leg").add_visual(
        VisualBuilder(
            BoxGeometry(0.6, 0.1, 0.2),
            material=white_material,
            origin=Transform(z=-0.3, pitch=pi / 2),
        )
    )

    right_leg = right_leg_link.build()

    right_base_link = LinkBuilder("[\\[right]\\]_base").add_visual(
        VisualBuilder(BoxGeometry(0.4, 0.1, 0.1), material=white_material)
    )

    right_base_joint = JointBuilder(
        "[\\[right]\\]_base_joint", JointType.Fixed, transform=Transform(z=-0.6)
    )

    right_leg.root_link.try_attach_child(right_base_link, right_base_joint)

    right_front_wheel_link = LinkBuilder("[\\[right]\\]_[[front]]_wheel").add_visual(
        VisualBuilder(
            CylinderGeometry(0.035, 0.1),
            origin=Transform(roll=pi / 2),
            material=black_material,
        )
    )

    right_front_wheel_joint = JointBuilder(
        r"[\[right]\]_[[front]]_wheel_joint",
        JointType.Fixed,
        transform=Transform(x=0.133333333333, z=-0.085),
    )

    right_leg.newest_link.try_attach_child(
        right_front_wheel_link, right_front_wheel_joint
    )

    right_back_wheel = (
        right_leg.joints["[\\[right]\\]_[[front]]_wheel_joint"]
        .rebuild_branch()
        .mirror(MirrorAxis.X)
    )
    right_back_wheel.change_group_id("back")

    right_leg.links[r"[\[right]\]_base"].attach_joint_chain(right_back_wheel)

    right_leg = right_leg.yank_root()
    right_leg.apply_group_id()

    base_right_leg_joint = JointBuilder("base_to_[[right]]_leg", JointType.Fixed)
    base_right_leg_joint.transform = Transform(y=-0.22, z=0.25)

    # ======= Attach right leg ====== #

    model.root_link.try_attach_child(right_leg, base_right_leg_joint)

    # ====== Attaching left leg ===== #

    left_leg = (
        model.joints["base_to_[[right]]_leg"].rebuild_branch().mirror(MirrorAxis.Y)
    )
    left_leg.change_group_id("left")

    model.root_link.attach_joint_chain(left_leg)

    # ====== Defining the gripper ===== #

    gripper_pole = (
        LinkBuilder("gripper_pole")
        .add_visual(
            VisualBuilder(
                CylinderGeometry(0.01, 0.2), origin=Transform(0.1, pitch=pi / 2)
            )
        )
        .build()
    )

    # left_gripper = LinkBuilder("[[left]]_gripper").add_visual(VisualBuilder(
    #     MeshGeometry()
    # ))

    # TODO:

    print(blue_material)


if __name__ == "__main__":
    main()
