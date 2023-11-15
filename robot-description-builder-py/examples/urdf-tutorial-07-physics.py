#!../venv/bin/python3
from math import pi

from robot_description_builder import MirrorAxis, Transform, to_urdf_string
from robot_description_builder.joint import JointBuilder, JointType, Limit
from robot_description_builder.link import Inertial, LinkBuilder
from robot_description_builder.link.collision import CollisionBuilder
from robot_description_builder.link.geometry import (BoxGeometry,
                                                     CylinderGeometry,
                                                     MeshGeometry,
                                                     SphereGeometry)
from robot_description_builder.link.visual import VisualBuilder
from robot_description_builder.material import Color, MaterialDescriptor


# [Adding Physical and Collision Properties to a URDF Model](http://wiki.ros.org/urdf/Tutorials/Adding%20Physical%20and%20Collision%20Properties%20to%20a%20URDF%20Model)
# Using ROS 2 as reference, since it is the most [up-to-date](https://github.com/ros/urdf_tutorial/blob/ros2/urdf/07-physics.urdf).
def main():
    # ==== Material Descriptions ==== #
    blue_material = MaterialDescriptor(Color(0, 0, 0.8), "blue")
    black_material = MaterialDescriptor(Color(0, 0, 0), "black")
    white_material = MaterialDescriptor(Color(1, 1, 1), "white")

    # =========== Step 1 ============ #
    base_link = (
        LinkBuilder("base_link")
        .add_visual(VisualBuilder(CylinderGeometry(0.2, 0.6), material=blue_material))
        .add_collider(CollisionBuilder(CylinderGeometry(0.2, 0.6)))
        .add_inertial(Inertial(10, 1e-3, 1e-3, 1e-3))
    )

    model = base_link.build().to_robot("physics")

    # ======= Start rigth leg ======= #
    right_leg_link_vis = VisualBuilder(
        BoxGeometry(0.6, 0.1, 0.2),
        material=white_material,
        transform=Transform(z=-0.3, pitch=pi / 2),
    )

    right_leg_link = (
        LinkBuilder(r"[\[right]\]_leg")
        .add_visual(right_leg_link_vis)
        .add_collider(right_leg_link_vis.as_collision())
        .add_inertial(Inertial(10, 1e-3, 1e-3, 1e-3))
    )

    right_leg = right_leg_link.build()

    right_base_link = (
        LinkBuilder(r"[\[right]\]_base")
        .add_visual(VisualBuilder(BoxGeometry(0.4, 0.1, 0.1), material=white_material))
        .add_collider(CollisionBuilder(BoxGeometry(0.4, 0.1, 0.1)))
        .add_inertial(Inertial(10, 1e-3, 1e-3, 1e-3))
    )

    right_base_joint = JointBuilder(
        r"[\[right]\]_base_joint", JointType.Fixed, transform=Transform(z=-0.6)
    )

    right_leg.root_link.try_attach_child(right_base_joint, right_base_link)

    right_front_wheel_link = (
        LinkBuilder(r"[\[right]\]_[[front]]_wheel")
        .add_visual(
            VisualBuilder(
                CylinderGeometry(0.035, 0.1),
                transform=Transform(roll=pi / 2),
                material=black_material,
            )
        )
        .add_collider(
            CollisionBuilder(
                CylinderGeometry(0.035, 0.1),
                transform=Transform(roll=pi / 2),
            )
        )
        .add_inertial(Inertial(1, 1e-3, 1e-3, 1e-3))
    )

    right_front_wheel_joint = JointBuilder(
        r"[\[right]\]_[[front]]_wheel_joint",
        JointType.Continuous,
        transform=Transform(x=0.133333333333, z=-0.085),
    )
    right_front_wheel_joint.axis = (0, 1, 0)

    right_leg.newest_link.try_attach_child(
        right_front_wheel_joint, right_front_wheel_link
    )

    right_back_wheel = (
        right_leg.joints[r"[\[right]\]_[[front]]_wheel_joint"]
        .rebuild_branch()
        .mirror(MirrorAxis.X)
    )
    right_back_wheel.change_group_id("back")
    right_back_wheel.axis = (0, 1, 0)

    right_leg.links[r"[\[right]\]_base"].attach_joint_chain(right_back_wheel)

    right_leg = right_leg.yank_root()
    right_leg.apply_group_id()

    base_right_leg_joint = JointBuilder("base_to_[[right]]_leg", JointType.Fixed)
    base_right_leg_joint.transform = Transform(y=-0.22, z=0.25)

    # ======= Attach right leg ====== #

    model.root_link.try_attach_child(base_right_leg_joint, right_leg)

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
                CylinderGeometry(0.01, 0.2), transform=Transform(0.1, pitch=pi / 2)
            )
        )
        .add_collider(
            CollisionBuilder(
                CylinderGeometry(0.01, 0.2), transform=Transform(0.1, pitch=pi / 2)
            )
        )
        .add_inertial(Inertial(0.05, 1e-3, 1e-3, 1e-3))
        .build()
    )

    left_gripper_geometry = MeshGeometry(
        "package://urdf_tutorial/meshes/l_finger.dae", (0.1, 0.05, 0.06)
    )

    left_gripper = (
        LinkBuilder("[[left]]_gripper")
        .add_visual(VisualBuilder(left_gripper_geometry))
        .add_collider(CollisionBuilder(left_gripper_geometry))
        .add_inertial(Inertial(0.05, 1e-3, 1e-3, 1e-3))
        .build()
    )

    left_tip_geometry = MeshGeometry(
        "package://urdf_tutorial/meshes/l_finger_tip.dae",
        (0.06, 0.04, 0.02),
    )

    left_tip_collider = CollisionBuilder(
        left_tip_geometry, transform=Transform(0.09137, 0.00495)
    )

    left_gripper.root_link.try_attach_child(
        JointBuilder("[[left]]_tip_joint", JointType.Fixed),
        LinkBuilder("[[left]]_tip")
        .add_visual(left_tip_collider.as_visual())
        .add_collider(left_tip_collider)
        .add_inertial(Inertial(0.05, 1e-3, 1e-3, 1e-3)),
    )

    gripper_pole.root_link.try_attach_child(
        JointBuilder(
            # TODO: Change to smartbuilder
            "[[left]]_gripper_joint",
            JointType.Revolute,
            transform=Transform(0.2, 0.01),
            axis=(0, 0, 1),
            limit=Limit(1000, 0.5, 0, 0.548),
        ),
        left_gripper.yank_root(),
    )

    right_gripper = (
        gripper_pole.joints["[[left]]_gripper_joint"]
        .rebuild_branch()
        .mirror(MirrorAxis.Y)
    )

    right_gripper.change_group_id("right")

    gripper_pole.root_link.attach_joint_chain(right_gripper)

    model.root_link.try_attach_child(
        JointBuilder(
            "gripper_extension",
            JointType.Prismatic,
            transform=Transform(0.19, 0.0, 0.2),
            limit=Limit(1000, 0.5, -0.38, 0),
        ),
        gripper_pole.yank_root(),
    )

    # ====== Defining the HEAD ====== #
    head_link = (
        LinkBuilder("head")
        .add_visual(
            VisualBuilder(
                SphereGeometry(0.2),
                material=MaterialDescriptor(Color(1.0, 1.0, 1.0), "white"),
            )
        )
        .add_collider(CollisionBuilder(SphereGeometry(0.2)))
        .add_inertial(Inertial(2, 1e-3, 1e-3, 1e-3))
    )

    head_swivel_joint = JointBuilder(
        "head_swivel", JointType.Continuous, transform=Transform(z=0.3), axis=(0, 0, 1)
    )

    model.root_link.try_attach_child(head_swivel_joint, head_link)

    #  The URDF tutorial is inconsistent here, out of nowhere translates visual, but not collision.
    box_link = (
        LinkBuilder("box")
        .add_visual(
            VisualBuilder(
                BoxGeometry(0.08, 0.08, 0.08),
                material=blue_material,
                transform=Transform(-0.04),
            )
        )
        .add_collider(
            CollisionBuilder(BoxGeometry(0.08, 0.08, 0.08), transform=Transform(-0.04))
        )
        .add_inertial(Inertial(1, 1e-3, 1e-3, 1e-3))
    )

    to_box_joint = JointBuilder(
        "tobox", JointType.Fixed, transform=Transform(0.1814, 0, 0.1414)
    )

    model.newest_link.try_attach_child(to_box_joint, box_link)

    result = to_urdf_string(model, indent=(" ", 2))

    print(result)


if __name__ == "__main__":
    main()
