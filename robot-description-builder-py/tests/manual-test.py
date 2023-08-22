#!../venv/bin/python3
from robot_description_builder.link import LinkBuilder
from robot_description_builder.link.visual import VisualBuilder, Visual
from robot_description_builder.link.collision import CollisionBuilder
from robot_description_builder.link.geometry import *
from robot_description_builder.joint import JointBuilder, JointType


import robot_description_builder as rdb

link_builder = LinkBuilder("my-link").add_visual(VisualBuilder(BoxGeometry(2,4,6), name="my-vis"))

print(link_builder)

tree = link_builder.build()
tree.root_link.try_attach_child(LinkBuilder("child-link"), JointBuilder("Revolute-Joint", JointType.Revolute))

print(tree)

vis: Visual = tree.root_link.visuals[0]

print(vis.transform)

print(tree.root_link.visuals)
print(tree.root_link.joints)
print(tree.print_refs())

print("Test Complete")