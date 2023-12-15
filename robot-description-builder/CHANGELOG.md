# 🦀 Changelog 🦀
The Changes between versions of [<b>`robot-description-builder`</b> for <b>Rust</b>](https://github.com/SuperJappie08/robot-description-builder/tree/master/robot-description-builder#robot-description-builder-) are recored here.

## Version 0.0.3
- Convert Collision and VisualBuilder between eachother.
- Renamed all `origin` fields to `transform` to make the naming less URDF specific.

### Added
- Added Changelog
- Added `VisualBuilder::to_collision` to allow for conversion between visual and collision builders
- Added `CollisionBuilder::to_visual` to allow for conversion between collision and visual builders
- Added `to_rdf::xml_writer_to_string` function, the use is self-explanitory.
- Added getter functions for `smartjointbuilder::smartparam` fields

### Changed/Updated
- Renamed all `origin` fields to `transform` to make the naming less URDF specific.
- Renamed `MaterialDataReferenceWrapper` to `MaterialDataReference`.
- Renamed `InertialData` to `Inertial`.
- Updated examples (URDF tutorial 7 & 8) to use `VisualBuilder::to_collision` and `CollisionBuilder::to_visual` in some places.
- Changed Errortype of `TryFrom<MaterialDataReference<'a>>` impl of `MaterialData` from `PoisonError<ArcLock<MaterialData>>` to `PoisonError<ErroredRead<ArcLock<MaterialData>>>`
- Added Errors to `yank` methods.
- Updated `AddLinkError` and `AddJointError` to be used with `AttachChainError`.
- `Joint::rebuild_branch` now returns a result type with error `RebuildBranchError`.
- The error type of `Link::{try_attach_child, attach_joint_chain, attach_joint_chain_at}` were changed (from `AddJointError`) to `AttachChainError`.
- Created a new alternative `new_quick_link` method renamed (the old one). 
- Renamed `MaterialDataReferenceWrapper` to `MaterialDataReference`.
- `KinematicInterface::{purge_links, purge_joints}` now have an `except` statement, since the error is unrecoverable until mutex_unpoison #96469 gets stabilized.
- Fixed spelling mistake in method name of `VisualBuilder` and `CollisionBuilder` (`*::tranformed` -> `*::transformed`)
- Flipped arguments of `Link::try_attach_child` to make more sense geometrically (Link -> Joint -> Link)
- Improved documentation.

#### Internal
- Renamed `KinematicDataTree::newer_link` to `KinematicDataTree::new` 

### Removed
- Hidden all transmission related methods, functions and types until they are implemented propperly.

### Misc
- Moved internal typealiasses `ArcLock` and `WeakLock` to `utils` module.
- Moved all generic error helpers to `utils` module.
- Created `ArcRW` trait, which has functions to read and write the internal lock of `ArcLock` with a better error. 
- `Joint::rebuild_branch_continued` now returns a result type with error `RebuildBranchError`
- The `visual_builders` field of `LinkBuilder` has been renamed to `visuals`.

- Bumped depency versions:
    - `itertools` 0.10.5 -> 0.12.0
    - `log` 0.4.18 -> 0.4.19 
    - `quick-xml` 0.29.0 -> 0.31.0
    - `thiserror` 1.0.40 -> 1.0
    - `nalgebra` 0.32.2 -> 0.32.3
    - `DEV` `test-log` 0.2.11 -> 0.2.12