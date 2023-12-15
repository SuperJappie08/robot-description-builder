# 🐍 Changelog 🐍
The Changes between versions of [<b>`robot-description-builder`</b> for <b>Python</b>](https://github.com/SuperJappie08/robot-description-builder/tree/master/robot-description-builder-py#robot-description-builder-) are recored here.

## Version 0.0.3
- Convert Collision and VisualBuilder between eachother.
- Renamed all `origin` fields to `transform` to make the naming less URDF specific.

### Added
- Added Changelog
- Added `VisualBuilder.to_collision` to allow for conversion between visual and collision builders
- Added `CollisionBuilder.to_visual` to allow for conversion between collision and visual builders
- Added `exceptions.AttachChainError`, which replaces `AddJointError` and `AddLinkError`
- Added `exceptions.RebuildBranchError` to reflect the changes of the Rust version.
- Implemented the `__eq__` & `__neq__`  methods to `Link`
- Added manual test `test/manual-2.py`
- Added tests for the `cluster_objects` module.

### Changed/Updated
- Renamed all `origin` fields to `transform` to make the naming less URDF specific.
- Updated examples (URDF tutorial 7 & 8) to use visual/collision conversions in some places.
- Fixed Spelling mistake.
- Flipped arguments of `Link::try_attach_child` to make more sense geometrically (Link -> Joint -> Link)
- Updated `pyproject.toml` to contain links to the repository.
- Improved stubfile coverage.

### Removed
- Removed Python's `AddJointError` and `AddLinkError` infavor of `AttachChainError`

### Misc
- Bumped depency versions:
    - Rust: PyO3 0.19.1 -> 0.20.0
    - Rust: itertools 0.10.5 -> 0.12.0
    - Rust: thiserror 0.10.40 -> 0.10 (Less specific versioning)
    - Python: Maturin 1.0.1 -> 1.3.2
    - Python: pytest 7.3.1 -> 7.4.3