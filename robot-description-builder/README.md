# Robot-description-builder 🦀
> **A Rust Crate for create (valid) Robot descriptions**

[![stability-unstable](https://img.shields.io/badge/stability-unstable-yellow.svg)](https://github.com/emersion/stability-badges#unstable)
[![Crates.io](https://img.shields.io/crates/v/robot-description-builder)](https://crates.io/crates/robot-description-builder)
[![Crates.io](https://img.shields.io/crates/d/robot-description-builder)](https://crates.io/crates/robot-description-builder)
[![Docs.rs](https://img.shields.io/docsrs/robot-description-builder)](https://docs.rs/robot-description-builder)
[![License: MIT](https://img.shields.io/crates/l/robot-description-builder)](https://github.com/SuperJappie08/robot-description-builder/blob/master/robot-description-builder/LICENSE)

<!-- Robot-description-builder -->
<b title="Robot-description-builder for Rust 🦀">`robot-description-builder`</b> is a <b title="Rust 🦀">Rust crate</b> for creating **robot descriptions** in multiple formats, like **[URDF](http://wiki.ros.org/urdf)**, for use in various **robotics and simulation** applications such as ROS and Gazebo Simulator.

<!-- TODO:ADD SOMETHING ABOUT BEING WRITTEN IN RUST -->
<!-- TODO:MENTION PYTHON PACKAGE -->

## Installation
<!-- TODO: MAYBE MOVE BECAUSE OF CRATES.IO LAYOUT-->
<b title="Robot-description-builder for Rust 🦀">`robot-description-builder`</b> can be installed from [Crates.io](https://crates.io/crates/robot-description-builder) using the following command:
```shell
$ carge add robot-description-builder
```

<!-- TODO: Add line to cargo.toml-->

## Features
- Support for the Full [URDF spec](http://wiki.ros.org/urdf/XML), fully compatible starting at ROS Indigo. (`Transmission`s are different before ROS Indigo, other features should work)
  - Support for all base URDF geometry types: `Box`, `Cylinder`, `Sphere` and `Mesh`.
- Mirroring of Kinematic chains.
- Easy cloning/renaming Kinematic chains by changing the `group_id`.
<!-- TODO: EXPAND FEATURE LIST -->
<pre align="center">🚧UNDER CONSTRUCTION: EXPAND FEATURE LIST🚧</pre>

### Compatibility chart
<!-- COMPATIBILTY CHART FORMAT? -->
| Spec | Support | State |
|:----:|:-------:|:-----:|
| [URDF](http://wiki.ros.org/urdf) | ✔/🔩 | Fully supported **TRANSMISIONS ARE CURRENTLY INCORRECT** |
| [URDF Gazebo](http://sdformat.org/tutorials?tut=sdformat_urdf_extensions&cat=specification&) | 🔩/❌ | Extension unsupported, Base URDF compatibility avaible |
| [SDF](http://sdformat.org/) | ❌ | Planned |

## Using <b title="robot-description-builder for Rust 🦀">`robot-description-builder`</b>
It is recommended to import only the items needed from the function or import the crate as `rdb`, since the crate name (<span title="robot-description-builder for Rust 🦀">`robot_description_builder`</span>) is quite long.
```rust
use robot_description_builder as rdb;
use rdb::prelude::*;
// TODO: EXPAND
```
<pre align="center">🚧UNDER CONSTRUCTION: EXAMPLE🚧</pre>


## Documentation
The documentation for this <span title="🦀📦">Rust Crate</span> can be found on [docs.rs](https://docs.rs/robot-description-builder/latest/robot_description_builder/index.html).
<pre align="center">🚧UNDER CONSTRUCTION: DOCUMENTATION IS UNFINISHED🚧</pre>


## Roadmap
- [ ] Improve documentation.
- [ ] Add shorthand [`Link`](https://docs.rs/robot-description-builder/latest/robot_description_builder/struct.Link.html) constructors.
- [ ] Add (partial) support for [SDFormat](http://sdformat.org/).
- [ ] Add support for the [Gazebo URDF extension](http://sdformat.org/tutorials?tut=sdformat_urdf_extensions&cat=specification&) support.
- [ ] \(Optional\) Add (partial) support for the [MIT Drake URDF extension](https://drake.mit.edu/doxygen_cxx/group__multibody__parsing.html#multibody_parsing_drake_extensions)

## Lessons
<!-- TODO: Add Lessons/Design Decision -->
<pre align="center">🚧UNDER CONSTRUCTION: ADD LESSONS/DESIGN DECISIONS🚧</pre>

## License
<b title="Robot-description-builder for Rust 🦀">`robot-description-builder`</b> is licensed under the [MIT](https://github.com/SuperJappie08/robot-description-builder/blob/master/robot-description-builder/LICENSE) license.