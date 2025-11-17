# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* `servo data-url parse`: Parse `data:`-urls
* `servo mime parse`: Parse mime-types into structured data
* Crate features to toggle parts of it
* Option to change the backend used (via crate feature flag)
* Added `blitz` backend (not yet useful, but will allow new features in the future)

### Changed

* re-imagined `servo html query`:
  * added `--format <string>`
  * It is now parsed by default (use `--format`)
  * `--inner-html` merged into `--format`
* re-imagined `servo xml query`:
  * added `--format <string>`
  * It is now parsed by default (use `--format`)
  * `--inner-html` merged into `--format`
  * `--from-xml-compat` merged into `--format`

### Removed

* `servo html parse_query`: use `servo html query`
* `servo xml parse_query`: use `servo xmlquery`
* `servo xml query --inner-html`: merged into `--format`
* `servo html query --inner-html`: merged into `--format`

## [0.108.0] - 2025-11-11

### Added

* `servo html parse`
* `servo html parse_query`
* `servo html query`
* `servo xml parse`
* `servo xml parse_query`
* `servo xml query`
