#!../venv/bin/python3
import robot_description_builder as rdb
from robot_description_builder.link import LinkBuilder
from robot_description_builder.link.geometry import BoxGeometry, CylinderGeometry
from robot_description_builder.link.visual import VisualBuilder
from robot_description_builder.material import Color, MaterialDescriptor


def main():
    # ==== Material Descriptions ==== #
    blue_material = MaterialDescriptor(Color(0, 0, 0.8), "blue")
    black_material = MaterialDescriptor(name="black", data=Color(0, 0, 0))
    white_material = MaterialDescriptor(Color(1, 1, 1))
    white_material.name = "white"

    # =========== Step 1 ============ #
    base_link = LinkBuilder("base_link").add_visual(
        VisualBuilder(
            CylinderGeometry(0.2, 0.6),
            material=blue_material
        )
    )

    # model = base_link.build().to_robot("visual")

    print(blue_material)


if __name__ == "__main__":
    main()
