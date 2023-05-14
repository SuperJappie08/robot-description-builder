# robot-description-builder \[rs\]

This is a temporary read me for the full project.
This will be cleaned up and improved soon.


## Roadmap:
- [x] Implement geometries
- [x] Implement Materials fully
- [x] Implement to URDF some way.
- [x] Implement mirroring
  - ~~`KinematicInterface::Mirror(...)`~~ ```Chainded<T: ChainableBuilder>::mirror(...)``` 
- [x] Implement propper name cloning
  - ~~`change_group_tag`~~ `change_group_id` method
- [ ] Transmissions
  - [x] Implementation
  - [ ] Documentation
- [ ] Wrap to python
- [x] Add LICENSE

### Long term:
- [ ] Learn how to generate python typesubs
- [ ] Support the `"logging"` feature everywhere.
- [ ] Support SDFormat
- [ ] Support URDF Gazebo Extension
- [ ] Support URDF Drake Extension

See Obsidian

## Might be nice to look into
 - [`nalgebra`](https://nalgebra.org/docs/user_guide/getting_started#usage-and-cargo-features) supports `arbitrary` feature which allows for randomized propperty testing using [`quickcheck`](https://crates.io/crates/quickcheck)
   - `quick-xml` also has a feature for it
 - A feature flag for Customizable types for `JointType` `TransmissionType` and `TransmissionHardwareInterface` etc.