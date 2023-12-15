import importlib
import subprocess
import sys
from pathlib import Path
import os

import pytest

IN_GITHUB_ACTIONS = os.getenv("GITHUB_ACTIONS") == "true"

# Import the examples as modules relative to this module
EXAMPLES_PATH = Path(__file__).parents[1].joinpath("examples")
EXECUTE_PATH = Path(__file__).parents[2]

if EXAMPLES_PATH not in sys.path:
    sys.path.insert(0, str(EXAMPLES_PATH.absolute()))

urdf_tutorials = {
    tutorial: importlib.import_module(tutorial)
    for tutorial in sorted(
        tutorial.name[:-3]
        for tutorial in EXAMPLES_PATH.absolute().iterdir()
        if tutorial.is_file() and tutorial.name[-3:] == ".py"
    )
}


# Test tutorial 5-7
@pytest.mark.skipif(IN_GITHUB_ACTIONS, reason="Test doesn't work in Github Actions.")
@pytest.mark.parametrize("name", sorted(urdf_tutorials.keys())[:-1])
def test_tutorials(capsys, name):
    result = urdf_tutorials[name]
    result.main()
    cap_py = capsys.readouterr()

    out_rs = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "--example",
            name,
        ],
        cwd=EXECUTE_PATH,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    ).stdout.decode()

    assert cap_py.out == out_rs


@pytest.mark.skipif(IN_GITHUB_ACTIONS, reason="Test doesn't work in Github Actions.")
def test_tutorial_08():
    name = "urdf-tutorial-08-macroed"
    out_py = subprocess.run(
        [sys.executable, EXAMPLES_PATH.joinpath(name + ".py"), "--width", "0.5"],
        cwd=EXAMPLES_PATH,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    ).stdout.decode()

    out_rs = subprocess.run(
        ["cargo", "run", "-q", "--example", name, "--", "0.5"],
        cwd=EXECUTE_PATH,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    ).stdout.decode()

    assert out_py == out_rs
