# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

# [3.1.0] - 2023-06-14
### Changed
- Updated zbus

# [3.1.0] - 2022-12-25
### Changed
- Remove useless `IntoPath` trait and methods
- Cleanup some useless return statements
- Use clippy suggestions
- Bump zbus dep

# [3.0.0]
### Changed
- Remove all wrapping as generated implementations are easily accessible now
- Adjust some serde impls for enums
- Add a few more tests, for both blocking and async
- *Note:* some I/O may still change a little but most things should now be stabilised

# [2.1.0]
### Changed
- Remove `non_blocking` feature as it is horribly broken and untested

# [2.0.0]
### Changed
- Upgrade crate to use zbus 2.0.0
- Enable majority of missing logind properties

# [0.7.0] - 2021-04-7
### Changed
- Revert to zbus 1.9.1 stable

# [0.6.1] - 2021-03-15
### Changed
- Derive PartialEq on more structs

# [0.6.1] - 2021-03-15
### Changed
- Derive PartialEq on more structs

# [0.6.0] - 2021-03-15
### Changed
- derive Debug, PartialEq, Clone, Copy, Serialize, Deserialize on enums
- Add list_active_graphical example
- Stricter types for <Proxy>::new() with a trait for getting interface path

# [0.5.0] - 2021-03-15
### Changed
- Add deref, deref_mut, as_ref, as_mut to all proxy to coeerce to `Proxy`
- Rename all `<Name>Interface` to `<Name>Proxy` as correct

# [0.4.0] - 2021-03-15
### Changed
- Begin using zbus 2.0
- Remove wrapper `disconnect_<name>` functions due to no-longer being generated

# [0.3.0] - 2021-03-15
### Changed
- Pass closures to wrapped signals correctly
- Return wrapped proxy as `<Type>Proxy` instead of trait object `Proxy`
- Enable more signals
- Add basic signal example
