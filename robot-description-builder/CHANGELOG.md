# 🦀 Changelog 🦀
The Changes between versions of [<b>`robot-description-builder`</b> for <b>Rust</b>](https://github.com/SuperJappie08/robot-description-builder/tree/master/robot-description-builder#robot-description-builder-) are recored here.

## DEV-Version 0.0.3
- Convert Collision and VisualBuilder between eachother.


### Added
- Added Changelog
- Added `VisualBuilder::to_collision` to allow for conversion between visual and collision builders
- Added `CollisionBuilder::to_visual` to allow for conversion between collision and visual builders

### Changed/Updated
- Updated examples (URDF tutorial 7 & 8) to use `VisualBuilder::to_collision` and `CollisionBuilder::to_visual` in some places.
- Changed Errortype of `TryFrom<MaterialDataReferenceWrapper<'a>>` impl of `MaterialData` from `PoisonError<ArcLock<MaterialData>>` to `PoisonError<ErroredRead<ArcLock<MaterialData>>>`
- Added Errors to `yank` methods

### Removed
### Misc
- Bumped depency versions:
 - `quick-xml` 0.29.0 -> 0.30.0