#!../venv/bin/python3
import argparse
from math import pi, sqrt

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


# [Using Xacro to Clean Up a URDF File](http://wiki.ros.org/urdf/Tutorials/Using%20Xacro%20to%20Clean%20Up%20a%20URDF%20File)
# Using ROS 2 as reference, since it is the most [up-to-date](https://github.com/ros/urdf_tutorial/blob/ros2/urdf/08-macroed.urdf.xacro).
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--width", type=float, default=0.2)
    parser.add_argument("--leglen", type=float, default=0.6)
    parser.add_argument("--polelen", type=float, default=0.2)
    parser.add_argument("--bodylen", type=float, default=0.6)
    parser.add_argument("--baselen", type=float, default=0.4)
    parser.add_argument("--wheeldiam", type=float, default=0.07)

    args = parser.parse_args()

    # ==== Material Descriptions ==== #
    blue_material = MaterialDescriptor(Color(0, 0, 0.8), "blue")
    black_material = MaterialDescriptor(Color(0, 0, 0), "black")
    white_material = MaterialDescriptor(Color(1, 1, 1), "white")

    def default_inertial(mass):
        return Inertial(mass, 1e-3, 1e-3, 1e-3)

    # =========== Step 1 ============ #
    base_link = (
        LinkBuilder("base_link")
        .add_visual(
            VisualBuilder(
                CylinderGeometry(args.width, args.bodylen), material=blue_material
            )
        )
        .add_collider(CollisionBuilder(CylinderGeometry(args.width, args.bodylen)))
        .add_inertial(default_inertial(10))
    )

    model = base_link.build().to_robot("macroed")

    # ======= Start rigth leg ======= #
    right_leg_link_vis = VisualBuilder(
        BoxGeometry(args.leglen, 0.1, 0.2),
        material=white_material,
        transform=Transform(z=-(args.leglen / 2), pitch=pi / 2),
    )

    right_leg_link = (
        LinkBuilder(r"[\[right]\]_leg")
        .add_visual(right_leg_link_vis)
        .add_collider(right_leg_link_vis.as_collision())
        .add_inertial(default_inertial(10))
    )

    right_leg = right_leg_link.build()

    right_base_link = (
        LinkBuilder(r"[\[right]\]_base")
        .add_visual(
            VisualBuilder(BoxGeometry(args.baselen, 0.1, 0.1), material=white_material)
        )
        .add_collider(CollisionBuilder(BoxGeometry(args.baselen, 0.1, 0.1)))
        .add_inertial(default_inertial(10))
    )

    right_base_joint = JointBuilder(
        r"[\[right]\]_base_joint", JointType.Fixed, transform=Transform(z=-args.leglen)
    )

    right_leg.root_link.try_attach_child(right_base_joint, right_base_link)

    right_front_wheel_link = (
        LinkBuilder(r"[\[right]\]_[[front]]_wheel")
        .add_visual(
            VisualBuilder(
                CylinderGeometry(args.wheeldiam / 2, 0.1),
                transform=Transform(roll=pi / 2),
                material=black_material,
            )
        )
        .add_collider(
            CollisionBuilder(
                CylinderGeometry(args.wheeldiam / 2, 0.1),
                transform=Transform(roll=pi / 2),
            )
        )
        .add_inertial(default_inertial(1))
    )

    right_front_wheel_joint = JointBuilder(
        r"[\[right]\]_[[front]]_wheel_joint",
        JointType.Continuous,
        transform=Transform(x=args.baselen / 3, z=-(args.wheeldiam / 2 + 0.05)),
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
    base_right_leg_joint.transform = Transform(y=-(args.width + 0.02), z=0.25)

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
                CylinderGeometry(0.01, args.polelen),
                transform=Transform(args.polelen / 2, pitch=pi / 2),
            )
        )
        .add_collider(
            CollisionBuilder(
                CylinderGeometry(0.01, args.polelen),
                transform=Transform(args.polelen / 2, pitch=pi / 2),
            )
        )
        .add_inertial(default_inertial(0.05))
        .build()
    )

    left_gripper_geometry = MeshGeometry(
        "package://urdf_tutorial/meshes/l_finger.dae", (0.1, 0.05, 0.06)
    )

    left_gripper = (
        LinkBuilder("[[left]]_gripper")
        .add_visual(VisualBuilder(left_gripper_geometry))
        .add_collider(CollisionBuilder(left_gripper_geometry))
        .add_inertial(default_inertial(0.05))
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
        .add_inertial(default_inertial(0.05)),
    )

    gripper_pole.root_link.try_attach_child(
        JointBuilder(
            # TODO: Change to smartbuilder
            "[[left]]_gripper_joint",
            JointType.Revolute,
            transform=Transform(args.polelen, 0.01),
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
            transform=Transform(args.width - 0.01, 0.0, 0.2),
            limit=Limit(1000, 0.5, -(args.width * 2 - 0.02), 0),
        ),
        gripper_pole.yank_root(),
    )

    # ====== Defining the HEAD ====== #
    head_link = (
        LinkBuilder("head")
        .add_visual(
            VisualBuilder(
                SphereGeometry(args.width),
                material=MaterialDescriptor(Color(1.0, 1.0, 1.0), "white"),
            )
        )
        .add_collider(CollisionBuilder(SphereGeometry(args.width)))
        .add_inertial(default_inertial(2))
    )

    head_swivel_joint = JointBuilder(
        "head_swivel",
        JointType.Continuous,
        transform=Transform(z=args.bodylen / 2),
        axis=(0, 0, 1),
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
        .add_inertial(default_inertial(1))
    )

    to_box_joint = JointBuilder(
        "tobox",
        JointType.Fixed,
        transform=Transform((args.width / sqrt(2)) + 0.04, 0, args.width / sqrt(2)),
    )

    model.newest_link.try_attach_child(to_box_joint, box_link)

    result = to_urdf_string(model, indent=(" ", 2))

    print(result)


if __name__ == "__main__":
    main()
