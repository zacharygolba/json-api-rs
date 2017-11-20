# Change Log

All notable changes to this project will be documented in this file.
See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## [v0.3.2](https://github.com/zacharygolba/json-api-rs/compare/v0.3.1...v0.3.2) (2017-11-20)

#### :bug: Bug Fix
* [#25](https://github.com/zacharygolba/json-api-rs/pull/25) Only use absolute paths in resource macro output.

#### Committers: 1
* Zachary Golba ([zacharygolba](https://github.com/zacharygolba))

## [v0.3.1](https://github.com/zacharygolba/json-api-rs/compare/v0.3.0...v0.3.1) (2017-11-20)

#### :bug: Bug Fix
* [#20](https://github.com/zacharygolba/json-api-rs/pull/20) Keys should be converted to kebab case when resources are rendered.
* [#21](https://github.com/zacharygolba/json-api-rs/pull/21) Disambiguate method calls in the resource macro output.

#### :memo: Documentation
* [#22](https://github.com/zacharygolba/json-api-rs/pull/22) Typo in context docs.
* [#23](https://github.com/zacharygolba/json-api-rs/pull/23) Update resource macro examples in the readme.

#### Committers: 1
* Zachary Golba ([zacharygolba](https://github.com/zacharygolba))

## [v0.3.0](https://github.com/zacharygolba/json-api-rs/compare/v0.2.0...v0.3.0) (2017-11-19)

#### :rocket: Enhancement
* [#14](https://github.com/zacharygolba/json-api-rs/pull/14) Efficiently decode types from a document.
* [#15](https://github.com/zacharygolba/json-api-rs/pull/15) Document and stabalize apis in the value module.

#### :bug: Bug Fix
* [#18](https://github.com/zacharygolba/json-api-rs/pull/18) Member names can start with reserved chars.

#### :nail_care: Polish
* [#17](https://github.com/zacharygolba/json-api-rs/pull/17) Add max width to rustfmt.toml.

#### :house: Internal
* [#16](https://github.com/zacharygolba/json-api-rs/pull/16) Improve internal and public builder apis.

#### Committers: 1
* Zachary Golba ([zacharygolba](https://github.com/zacharygolba))

## [v0.2.0](https://github.com/zacharygolba/json-api-rs/compare/v0.1.0...v0.2.0) (2017-11-5)

#### :rocket: Enhancement
* [#11](https://github.com/zacharygolba/json-api-rs/pull/11) Add tests for the query module.
* [#10](https://github.com/zacharygolba/json-api-rs/pull/10) Add negation operator and reverse method to sort and direction.
* [#6](https://github.com/zacharygolba/json-api-rs/pull/6) Report test coverage to codecov.
* [#1](https://github.com/zacharygolba/json-api-rs/pull/1) Add support for encoding and decoding query params.

#### :bug: Bug Fix
* [#8](https://github.com/zacharygolba/json-api-rs/pull/8) Ignore unknown fields.

#### :nail_care: Polish
* [#2](https://github.com/zacharygolba/json-api-rs/pull/2) Update rustfmt config.

#### :memo: Documentation
* [#4](https://github.com/zacharygolba/json-api-rs/pull/4) Add ci badges to readme and crates.io.

#### :house: Internal
* [#13](https://github.com/zacharygolba/json-api-rs/pull/13) Update version format when passed to --vers in test.sh.
* [#9](https://github.com/zacharygolba/json-api-rs/pull/9) Do not show private _ext field when debugging.
* [#7](https://github.com/zacharygolba/json-api-rs/pull/7) Improve rocket crate structure.
* [#5](https://github.com/zacharygolba/json-api-rs/pull/5) Invalidate appveyor cache when appveyor.yml is changed.
* [#3](https://github.com/zacharygolba/json-api-rs/pull/3) Setup appveyor and circle ci.

#### Committers: 1
* Zachary Golba ([zacharygolba](https://github.com/zacharygolba))
