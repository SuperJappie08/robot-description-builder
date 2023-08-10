# 🐍 Changelog 🐍
The Changes between versions of [<b>`robot-description-builder`</b> for <b>Python</b>](https://github.com/SuperJappie08/robot-description-builder/tree/master/robot-description-builder-py#robot-description-builder-) are recored here.

## DEV-Version 0.0.3
- Convert Collision and VisualBuilder between eachother.

### Added
- Added Changelog
- Added `VisualBuilder.to_collision` to allow for conversion between visual and collision builders
- Added `CollisionBuilder.to_visual` to allow for conversion between collision and visual builders

### Changed/Updated
- Updated examples (URDF tutorial 7 & 8) to use visual/collision conversions in some places.

### Removed
### Misc
- Bumped depency versions:
 - Rust: PyO3 0.19.1 -> 0.19.2
 - Python: Maturin 1.1.0 -> 1.2.0