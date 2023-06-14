#!../venv/bin/python3
import pytest
from robot_description_builder import Transform
from robot_description_builder.joint import JointBuilder, JointType

def test_joint_builder_new_error_wrong_type():
    with pytest.raises(TypeError) as exception:
        JointBuilder("d", JointType.Fixed, transform=[1,2])
    assert exception.type is TypeError
    assert exception.value.args[0] == "'list' object cannot be converted to 'Transform'"

    
def test_joint_builder_new_error_wrong_key():
    with pytest.raises(TypeError) as exception:
        JointBuilder("d", JointType.Fixed, transforms=Transform(x=1))
    assert exception.type is TypeError
    assert exception.value.args[0] == "JointBuilder.__new__() got an unexpected keyword argument 'transforms'"

def test_joint_builder_new_error_extra_unknown_key():
    with pytest.raises(TypeError) as exception:
        JointBuilder("d", JointType.Fixed, transform=Transform(x=1), color="Red")
    assert exception.type is TypeError
    assert exception.value.args[0] == "JointBuilder.__new__() got an unexpected keyword argument 'color'"
