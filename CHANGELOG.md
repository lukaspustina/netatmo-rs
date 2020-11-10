# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2020-10-12

### Change

* `NetatmoClient::get_station_data` returns a modified `StationData` struct.

    The changes occurred in the sub-struct `Administrative`. `pressureunit` is now an `Option<u64>` and `country` has been added.

    *Attention*: This is a breaking change necessary due to different responses from some Netatmo devices.


## [0.3.0] - 2020-01-27

### Add

* API endpoints
    * [home status](https://dev.netatmo.com/apidocumentation/energy#homestatus)
    * [homes data](https://dev.netatmo.com/apidocumentation/energy#homesdata)
    * [set room thermpoint](https://dev.netatmo.com/apidocumentation/energy#setroomthermpoint)

[Unreleased]: https://github.com/centerdevice/ceres/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/centerdevice/ceres/compare/v0.3.0...v0.5.0
[0.3.0]: https://github.com/centerdevice/ceres/compare/v0.2.1...v0.3.0

