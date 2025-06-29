# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.4] - 2025-06-29

### Changed

- Bump to 1.5.4
- Update changelog

### Fixed

- Remove unwanted error propagation

## [1.5.3] - 2025-06-29

### Changed

- Bump to 1.5.3
- Genetic parser
- Launch check workflow on push
- Use peeking to get delimiter tokens
- Update changelog

### Fixed

- Escaped group delimited
- Relax separated list parameters requirements

## [1.5.2] - 2025-06-01

### Changed

- Bump to 1.5.2
- Update changelog

### Fixed

- Bad peekable implementation for Token

## [1.5.1] - 2025-05-31

### Changed

- Bump version 1.5.1
- Update changelog

### Fixed

- Remove redundant Until modifier

## [1.5.0] - 2025-05-31

### Changed

- Bump version 1.5.0
- Provide a default implementation of Peekable to all Visitor but relax it to allow customization
- Update changelog

## [1.4.0] - 2025-05-30

### Changed

- Bump to 1.4.0
- Introduce get_scanner_without_trailing_separator to cleanup scanner before accepting SeparatedList
- Update changelog

### Fixed

- Bad behavior in case of not foundable pattern but another pattern found previously

## [1.3.0] - 2025-05-30

### Changed

- Bump to 1.3.0
- Add the Last peekable object
- Update changelog

## [1.2.1] - 2025-05-30

### Changed

- Bump to version 1.2.1
- Update changelog

### Fixed

- Relax Peekable Clone requirement

## [1.2.0] - 2025-05-30

### Changed

- Bump to 1.2.0
- Update changelog

### Fixed

- Simplify usage of Until peekable

## [1.1.0] - 2025-05-30

### Changed

- Bump to 1.1.0
- Blanket implementation of Visitor on Defaulted Recognizable
- Update changelog

### Fixed

- Recognizer doesn't propagate the previous state in case of empty scanner after successful match

## [1.0.7] - 2025-05-29

### Changed

- Update changelog

### Fixed

- Fix scanner mutability in the peek method

## [1.0.6] - 2025-05-29

### Changed

- Bump version to v1.0.6
- Update changelog

### Fixed

- Simplify internal perking result type

## [1.0.5] - 2025-05-28

### Changed

- Update changelog

### Fixed

- Merge Match and MatchSize traits

## [1.0.4] - 2025-05-28

### Changed

- Update changelog

### Fixed

- Simplify the recognizer definition

## [1.0.3] - 2025-05-28

### Changed

- Update changelog

### Fixed

- Prevent acceptor to accept an EOF data
- Remove useless into_data method on Scanner

## [1.0.2] - 2025-05-28

### Changed

- Bump version to v1.0.2
- Fix Scanner and Peeking lifetimes
- Update changelog

### Fixed

- Fix peeking lifetime
- Set the Scanner::remaining lifetime return to 'a instead '1

## [1.0.1] - 2025-05-28

### Changed

- Define BSD licence

## [1.0.0] - 2025-05-28

### Changed

- Rename Noa Parser into Elyze and bump to 1.0.0
- Run changelog and publish only on tags
- Split workflows between publish and changelog

### Fixed

- Allow empty scanner on separated list

## [0.7.3] - 2025-05-27

### Fixed

- Relax Peeker lifetimes

## [0.7.2] - 2025-05-27

### Fixed

- Until behavior

## [0.7.1] - 2025-05-27

### Fixed

- UntilEnd behavior

## [0.7.0] - 2025-05-27

### Changed

- Allow to peek until a specific element

## [0.6.0] - 2025-05-27

### Changed

- Add Recognizer

## [0.5.0] - 2025-05-27

### Changed

- Add support to separated list

## [0.4.0] - 2025-05-27

### Changed

- Add support to carriage return and tab recognizer

## [0.3.0] - 2025-05-27

### Changed

- Add whitespaces visitor

## [0.1.1] - 2025-05-27

### Changed

- Use blanket implementation on Recognizable trait

## [0.1.0] - 2025-05-25

### Changed

- Initial commit

[1.5.4]: https://github.com/Elyze-Parser/elyze/compare/v1.5.3..v1.5.4
[1.5.3]: https://github.com/Elyze-Parser/elyze/compare/v1.5.2..v1.5.3
[1.5.2]: https://github.com/Elyze-Parser/elyze/compare/v1.5.1..v1.5.2
[1.5.1]: https://github.com/Elyze-Parser/elyze/compare/v1.5.0..v1.5.1
[1.5.0]: https://github.com/Elyze-Parser/elyze/compare/v1.4.0..v1.5.0
[1.4.0]: https://github.com/Elyze-Parser/elyze/compare/v1.3.0..v1.4.0
[1.3.0]: https://github.com/Elyze-Parser/elyze/compare/v1.2.1..v1.3.0
[1.2.1]: https://github.com/Elyze-Parser/elyze/compare/v1.2.0..v1.2.1
[1.2.0]: https://github.com/Elyze-Parser/elyze/compare/v1.1.0..v1.2.0
[1.1.0]: https://github.com/Elyze-Parser/elyze/compare/v1.0.7..v1.1.0
[1.0.7]: https://github.com/Elyze-Parser/elyze/compare/v1.0.6..v1.0.7
[1.0.6]: https://github.com/Elyze-Parser/elyze/compare/v1.0.5..v1.0.6
[1.0.5]: https://github.com/Elyze-Parser/elyze/compare/v1.0.4..v1.0.5
[1.0.4]: https://github.com/Elyze-Parser/elyze/compare/v1.0.3..v1.0.4
[1.0.3]: https://github.com/Elyze-Parser/elyze/compare/v1.0.2..v1.0.3
[1.0.2]: https://github.com/Elyze-Parser/elyze/compare/v1.0.1..v1.0.2
[1.0.1]: https://github.com/Elyze-Parser/elyze/compare/v1.0.0..v1.0.1
[1.0.0]: https://github.com/Elyze-Parser/elyze/compare/v0.7.3..v1.0.0
[0.7.3]: https://github.com/Elyze-Parser/elyze/compare/v0.7.2..v0.7.3
[0.7.2]: https://github.com/Elyze-Parser/elyze/compare/v0.7.1..v0.7.2
[0.7.1]: https://github.com/Elyze-Parser/elyze/compare/v0.7.0..v0.7.1
[0.7.0]: https://github.com/Elyze-Parser/elyze/compare/v0.6.0..v0.7.0
[0.6.0]: https://github.com/Elyze-Parser/elyze/compare/v0.5.0..v0.6.0
[0.5.0]: https://github.com/Elyze-Parser/elyze/compare/v0.4.0..v0.5.0
[0.4.0]: https://github.com/Elyze-Parser/elyze/compare/v0.3.0..v0.4.0
[0.3.0]: https://github.com/Elyze-Parser/elyze/compare/v0.1.1..v0.3.0
[0.1.1]: https://github.com/Elyze-Parser/elyze/compare/v0.1.0..v0.1.1

<!-- generated by git-cliff -->
