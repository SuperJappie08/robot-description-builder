# Robot-description-builder 🐍
> **A Python Package for create (valid) Robot descriptions**

[![PyPI - Status](https://img.shields.io/pypi/status/robot-description-builder)](https://github.com/SuperJappie08/robot-description-builder/blob/master/robot-description-builder-py/README.md)
[![PyPI](https://img.shields.io/pypi/v/robot-description-builder)](https://pypi.org/project/robot-description-builder/)
[![PyPI - Implementation](https://img.shields.io/pypi/implementation/robot-description-builder)](https://pypi.org/project/robot-description-builder/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/robot-description-builder)](https://pypi.org/project/robot-description-builder/)
[![PyPI - License](https://img.shields.io/pypi/l/robot-description-builder)](https://github.com/SuperJappie08/robot-description-builder/blob/master/robot-description-builder-py/LICENSE)
[![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black)
<!-- [![Imports: isort](https://img.shields.io/badge/%20imports-isort-%231674b1?style=flat&labelColor=ef8336)](https://pycqa.github.io/isort/) -->

<!-- Robot-description-builder -->
<b title="Robot-description-builder for Python 🐍">`robot-description-builder`</b> is a <b title="written in Rust 🦀">Python library</b> for creating **robot descriptions** in multiple formats, like **[URDF](http://wiki.ros.org/urdf)**, for use in various **robotics and simulation** applications such as ROS and Gazebo Simulator.

<!-- ADD SOMETHING ABOUT BEING WRITTEN IN RUST -->
The <span title="robot-description-builder for Python 🐍">Python version of `robot-description-builder`</span> is written in <span title="Rust 🦀">Rust</span> using [PyO3](https://github.com/PyO3/pyo3) by wrapping the <a href="https://crates.io/crates/robot-description-builder" title="robot-description-builder for Rust 🦀">`robot-description-builder`</a> Rust crate. This has been done for the following reasons:
- Using <span title="Rust 🦀">Rust</span> prevented memory leaks, which would occur in a full <span title="Python 🐍">Python</span> implementation. (I had not heard of the [`weakref`](https://docs.python.org/3/library/weakref.html) module, yet.)
- Using <span title="Rust 🦀">Rust</span> also allows for interesting compile time validation, which is only available in the <span title="Rust 🦀">Rust</span> Language. Resulting in the [`SmartJointBuilder`](https://docs.rs/robot-description-builder/latest/robot_description_builder/struct.SmartJointBuilder.html) (Only available in the <a href="https://crates.io/crates/robot-description-builder" title="robot-description-builder for Rust 🦀">Rust version</a>).
- Creating a <span title="Rust 🦀">Rust</span> library and wrapping it in <span title="Python 🐍">Python</span> creates two libraries *with little or no extra work*[^1].
<!-- - Creating a <span title="Rust 🦀">Rust</span> and wrapping it for <span title="Python 🐍">Python</span>, creates two libraries with little to no extra effort[^1]. -->

[^1]: Famous last words.
<!-- ADD SOMETHING ABOUT ABRV> to rdb -->

## Installation
<b title="Robot-description-builder for Python 🐍">`robot-description-builder`</b> can be installed from PyPi using the following command:
```
$ pip install robot-description-builder
```

## Features
- Support for the Full [URDF spec](http://wiki.ros.org/urdf/XML), fully compatible starting at ROS Indigo. (`Transmission`s are different before ROS Indigo, other features should work)
  - Support for all base URDF geometry types: `Box`, `Cylinder`, `Sphere` and `Mesh`.
- Mirroring of Kinematic chains.
- Easy cloning/renaming Kinematic chains by changing the `group_id`.
- ROS independent, can be run on any machine using Python 3.8 and above.
<!-- TODO: EXPAND FEATURE LIST -->
<pre align="center">🚧UNDER CONSTRUCTION: EXPAND FEATURE LIST🚧</pre>

### Compatibility chart
<!-- COMPATIBILTY CHART FORMAT? -->
| Spec | Support | State |
|:----:|:-------:|:-----:|
| [URDF](http://wiki.ros.org/urdf) | ✔/🔩 **WIP** | Fully supported **TRANSMISIONS ARE CURRENTLY INCORRECT** |
| [URDF Gazebo](http://sdformat.org/tutorials?tut=sdformat_urdf_extensions&cat=specification&) | 🔩/❌ | Extension unsupported, Base URDF compatibility avaible |
| [SDF](http://sdformat.org/) | ❌ | Planned |

## Using <b title="robot-description-builder for Python 🐍">`robot-description-builder`</b>
<!--TODO: REWRITE FIRST SENTENCE-->
It is recommended to import only the classes needed from the package or import the module as `rdb`, since the package name (<span title="robot-description-builder for Python 🐍">`robot_description_builder`</span>) is quite long.
<!-- TODO: An example -->
```python
import robot_description_builder as rdb
# TODO: EXPAND
```
<pre align="center">🚧UNDER CONSTRUCTION: EXPAND EXAMPLE🚧</pre>

## Documentation
This <span title="🐍📦">Python Package</span> has typing support and comes fully equiped with `docstrings` and stub files. *Documentation pages comming soon(ish).*
<!-- TODO: Link to docs -->
<pre align="center">🚧UNDER CONSTRUCTION: CREATE DOCUMENTATION PAGES🚧</pre>


## Roadmap
- [ ] Add documentation pages.
- [ ] Add shorthand `Link` constructors.
- [ ] Add (partial) support for [SDFormat](http://sdformat.org/).
- [ ] Add support for the [Gazebo URDF extension](http://sdformat.org/tutorials?tut=sdformat_urdf_extensions&cat=specification&) support.
- [ ] \(Optional\) Add (partial) support for the [MIT Drake URDF extension](https://drake.mit.edu/doxygen_cxx/group__multibody__parsing.html#multibody_parsing_drake_extensions)

### Interesting ideas with questionable feasibility
- [ ] Add `SmartJointBuilder`, similar to the [Rust version](https://github.com/SuperJappie08/robot-description-builder/blob/master/robot-description-builder/README.md)
  > It would need to be a dynamic class with function injection, <b>(ASSUMPTION)</b> which would not work with IntelliSense making.

## Lessons
<!-- TODO: Add Lessons/Design Decisions -->
<pre align="center">🚧UNDER CONSTRUCTION: ADD LESSONS/DESIGN DECISIONS🚧</pre>

## License
<!-- Robot-description-builder <sup>(for Python)</sup> is licensed under the [MIT](./LICENSE) license. -->
<b title="Robot-description-builder for Python 🐍">`robot-description-builder`</b> is licensed under the [MIT](LICENSE) license.