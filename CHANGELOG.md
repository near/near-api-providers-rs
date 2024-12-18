# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.15.2](https://github.com/near/near-jsonrpc-client-rs/compare/v0.15.1...v0.15.2) - 2024-12-18

### Other

- added CODEOWNERS (#167)

## [0.15.1](https://github.com/near/near-jsonrpc-client-rs/compare/v0.15.0...v0.15.1) - 2024-12-13

### Other

- fixed test compilation (#165)

## [0.15.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.14.0...v0.15.0) - 2024-12-10

### Other

- [**breaking**] updates near-* dependencies to 0.28 release (#163)

## [0.14.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.13.0...v0.14.0) - 2024-11-12

### Added

- added http errors 400, 408, 503, 500 ([#160](https://github.com/near/near-jsonrpc-client-rs/pull/160))

### Other

- [**breaking**] updates near-* dependencies to 0.27 release ([#161](https://github.com/near/near-jsonrpc-client-rs/pull/161))

## [0.13.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.12.0...v0.13.0) - 2024-09-11

### Other

- [**breaking**] updates near-* dependencies to 0.26 release ([#157](https://github.com/near/near-jsonrpc-client-rs/pull/157))

## [0.12.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.11.0...v0.12.0) - 2024-08-21

### Other
- updated near-* to 0.25.0 ([#154](https://github.com/near/near-jsonrpc-client-rs/pull/154))

## [0.11.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.10.1...v0.11.0) - 2024-08-09

### Other
- updated near-* crates to allow 0.24.0 in addition to all the previously supported versions ([#151](https://github.com/near/near-jsonrpc-client-rs/pull/151))

## [0.10.1](https://github.com/near/near-jsonrpc-client-rs/compare/v0.10.0...v0.10.1) - 2024-06-18

### Other
- Updated near-deps to 0.23.0 ([#148](https://github.com/near/near-jsonrpc-client-rs/pull/148))

## [0.10.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.9.0...v0.10.0) - 2024-06-07

### Other
- expose `ChunkReference` type used in chunk call ([#144](https://github.com/near/near-jsonrpc-client-rs/pull/144))
- [**breaking**] Upgraded libraries to the latest versions: near-* 0.22 and reqwest 0.12 ([#145](https://github.com/near/near-jsonrpc-client-rs/pull/145))

## [0.9.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.8.0...v0.9.0) - 2024-04-22

### Added
- Upgrade near-primitives to 0.21.x and refactor the API adding `wait_until` flag, drop `check_tx` method ([#136](https://github.com/near/near-jsonrpc-client-rs/pull/136))

### Other
- removed array creation for parameters in fn params() implementation for send_tx method ([#142](https://github.com/near/near-jsonrpc-client-rs/pull/142))

## [0.8.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.7.0...v0.8.0) - 2024-01-21

### Other
- [**breaking**] Upgraded NEAR crates to 0.20.0 release ([#137](https://github.com/near/near-jsonrpc-client-rs/pull/137))

## [0.7.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.6.0...v0.7.0) - 2024-01-07

### Added
- Added a new example to view contract state ([#129](https://github.com/near/near-jsonrpc-client-rs/pull/129))

### Fixed
- Fixed doc tests after the recent crate updates ([#135](https://github.com/near/near-jsonrpc-client-rs/pull/135))

### Other
- *(docs)* revise crate-level docs ([#88](https://github.com/near/near-jsonrpc-client-rs/pull/88))
- *(docs)* revise README docs ([#91](https://github.com/near/near-jsonrpc-client-rs/pull/91))
- *(docs)* revise module-level docs for RPC methods ([#87](https://github.com/near/near-jsonrpc-client-rs/pull/87))
- *(docs)* document the experimental `tx_status` RPC method ([#63](https://github.com/near/near-jsonrpc-client-rs/pull/63))
- *(docs)* document the experimental `genesis_config` RPC method ([#64](https://github.com/near/near-jsonrpc-client-rs/pull/64))
- *(docs)* document the `EXPERIMENTAL_protocol_config` RPC method ([#65](https://github.com/near/near-jsonrpc-client-rs/pull/65))
- *(docs)* document the `EXPERIMENTAL_changes_in_block` RPC method ([#66](https://github.com/near/near-jsonrpc-client-rs/pull/66))
- *(docs)* document the `EXPERIMENTAL_receipt` RPC method ([#67](https://github.com/near/near-jsonrpc-client-rs/pull/67))
- *(docs)* document the `query` RPC method ([#68](https://github.com/near/near-jsonrpc-client-rs/pull/68))
- *(docs)* document the experimental `validators_ordered` RPC method ([#69](https://github.com/near/near-jsonrpc-client-rs/pull/69))
- *(docs)* document the `gas_price` RPC method ([#70](https://github.com/near/near-jsonrpc-client-rs/pull/70))
- *(docs)* document the `EXPERIMENTAL_changes` RPC method ([#71](https://github.com/near/near-jsonrpc-client-rs/pull/71))
- *(docs)* document the `EXPERIMENTAL_check_tx` RPC method ([#72](https://github.com/near/near-jsonrpc-client-rs/pull/72))
- *(docs)* document the `sandbox_fast_forward` RPC method ([#75](https://github.com/near/near-jsonrpc-client-rs/pull/75))
- *(docs)* document the `validators` RPC method ([#76](https://github.com/near/near-jsonrpc-client-rs/pull/76))
- *(docs)* document the `next_light_client_block` RPC method ([#78](https://github.com/near/near-jsonrpc-client-rs/pull/78))
- *(docs)* document the `broadcast_tx_async` RPC method ([#79](https://github.com/near/near-jsonrpc-client-rs/pull/79))
- *(docs)* document the `broadcast_tx_commit` RPC method ([#80](https://github.com/near/near-jsonrpc-client-rs/pull/80))
- *(docs)* document the `sandbox_patch_state` RPC method ([#81](https://github.com/near/near-jsonrpc-client-rs/pull/81))
- *(docs)* document the error types and variants in the `errors` module ([#84](https://github.com/near/near-jsonrpc-client-rs/pull/84))
- *(docs)* document the generic `methods::any()` constructor ([#89](https://github.com/near/near-jsonrpc-client-rs/pull/89))
- [**breaking**] Bump dependencies ([#134](https://github.com/near/near-jsonrpc-client-rs/pull/134))

## [0.6.0](https://github.com/near/near-jsonrpc-client-rs/compare/v0.5.1...v0.6.0) - 2023-06-02

### Other
- [**breaking**] Upgrade near primitive crates version to 0.17.0 ([#126](https://github.com/near/near-jsonrpc-client-rs/pull/126))

## [0.5.1] - 2023-03-22

- Updated `borsh` to `0.10.2`. <https://github.com/near/near-jsonrpc-client-rs/pull/124>

## [0.5.0] - 2023-02-24

### Added

- `ApiKey::new` now accepts byte arrays and byte slices. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- `Authorization::bearer` method for token-authenticated requests. <https://github.com/near/near-jsonrpc-client-rs/pull/121>
- `ApiKey::as_bytes` returns a byte slice of the key without utf-8 validation. <https://github.com/near/near-jsonrpc-client-rs/pull/119>

### Changed

- Updated nearcore dependencies to `0.16.0`, which now requires a MSRV of `1.67.1`. <https://github.com/near/near-jsonrpc-client-rs/pull/122>
- `ApiKey::new` no longer requres the input of a valid UUID. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- `Debug` on `ApiKey` doesn't reveal the key anymore. <https://github.com/near/near-jsonrpc-client-rs/pull/120>
- The `auth` module is no longer feature gated. <https://github.com/near/near-jsonrpc-client-rs/pull/119>

### Breaking

- Removed the `auth::IntoApiKey` trait, any thing you can get a byte slice from is now a valid `ApiKey` input. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Replaced the `ApiKey::as_str` method with `ApiKey::to_str`, now returning a `Result`. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Replaced the `InvalidApiKey` error with `InvalidHeaderValue` re-exported from `http`. <https://github.com/near/near-jsonrpc-client-rs/pull/119>
- Removed `Display` on `ApiKey`. <https://github.com/near/near-jsonrpc-client-rs/pull/117>

## [0.4.1] - 2022-11-11

- Fixed an issue where an `&RpcMethod`'s response was being parsed differently from an `RpcMethod`. <https://github.com/near/near-jsonrpc-client-rs/pull/114>

## [0.4.0] - 2022-10-04

- Updated nearcore dependencies, which now requires a MSRV of `1.64.0`. <https://github.com/near/near-jsonrpc-client-rs/pull/100>, <https://github.com/near/near-jsonrpc-client-rs/pull/110>
- Updated other dependencies, with some general improvements. <https://github.com/near/near-jsonrpc-client-rs/pull/111>
- Added `rustls-tls` feature flag to enable `rustls` as an alternative to `native-tls`. <https://github.com/near/near-jsonrpc-client-rs/pull/103>
- Switched to using `log::debug!` instead of `log::info!` for debug logging. <https://github.com/near/near-jsonrpc-client-rs/pull/107>
- Fixed `gas_price` RPC method serialization. <https://github.com/near/near-jsonrpc-client-rs/pull/73>
- Fixed `query` method error deserialization. <https://github.com/near/near-jsonrpc-client-rs/pull/82>
- Reworked the `JsonRpcError`::`handler_error` method. <https://github.com/near/near-jsonrpc-client-rs/pull/99>
- Moved auth specific logic behind a feature flag. <https://github.com/near/near-jsonrpc-client-rs/pull/55>
- Added the `methods::to_json()` helper method for visualizing the serialization of the RPC methods. <https://github.com/near/near-jsonrpc-client-rs/pull/49>

## [0.4.0-beta.0] - 2022-05-31

<details>
<summary>
  <em>
    Superseded by <a href="https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.0-beta.0...v0.4.0">
      <code> 0.4.0 </code>
    </a>
  </em>
</summary>

> - Updated nearcore dependencies, fixing a previous breaking change. <https://github.com/near/near-jsonrpc-client-rs/pull/100>
> - Fixed `gas_price` RPC method serialization. <https://github.com/near/near-jsonrpc-client-rs/pull/73>
> - Fixed `query` method error deserialization. <https://github.com/near/near-jsonrpc-client-rs/pull/82>
> - Reworked the `JsonRpcError`::`handler_error` method. <https://github.com/near/near-jsonrpc-client-rs/pull/99>
> - Moved auth specific logic behind a feature flag. <https://github.com/near/near-jsonrpc-client-rs/pull/55>
> - Added the `methods::to_json()` helper method for visualizing the serialization of the RPC methods. <https://github.com/near/near-jsonrpc-client-rs/pull/49>

</details>

## [0.3.0] - 2022-02-09

- Dropped generic authentication and added support for custom headers. <https://github.com/near/near-jsonrpc-client-rs/pull/47>
- Added the `sandbox_fast_forward` RPC Method. <https://github.com/near/near-jsonrpc-client-rs/pull/38>
- Upgraded `nearcore` crates to `v0.12.0` <https://github.com/near/near-jsonrpc-client-rs/pull/48>
- Executing the [examples](https://github.com/near/near-jsonrpc-client-rs/tree/master/examples) now allows custom RPC addr specification with interactive server selection. <https://github.com/near/near-jsonrpc-client-rs/commit/b130118d0de806bd9950be306f563559f07c77e6> <https://github.com/near/near-jsonrpc-client-rs/commit/c5e938a90703cb216e99d6f23a43ad9d3812df3d>
- `JsonRpcClient::connect` is now generic over any url-like type. [`Url`](https://docs.rs/url/*/url/struct.Url.html), `&str`, `String` and `&String` are all supported. <https://github.com/near/near-jsonrpc-client-rs/pull/35>
- `JsonRpcClient` now defaults to the `Unauthenticated` state, easing a type specification pain point. <https://github.com/near/near-jsonrpc-client-rs/pull/36>

## [0.2.0] - 2021-12-22

- Updated nearcore version to `0.11.0` (<https://github.com/near/nearcore/pull/5943>).
- Fixed `chunk` method serialization. <https://github.com/near/near-jsonrpc-client-rs/commit/f40ad743653ad2a1a9eb5eaa96c302c9b531bf40>
- `Client::call` no longer consumes the client. <https://github.com/near/near-jsonrpc-client-rs/commit/471a53be062e0880c6bc5c2721d123da2a9e0c2e>
- Implemented workaround for partially serialized server responses. <https://github.com/near/near-jsonrpc-client-rs/pull/29>
- Dropped base64 API token support in favor of a generic key-value approach. <https://github.com/near/near-jsonrpc-client-rs/commit/dd7761b51e1775350be1782370aa22c0b0fe98d7>
- Added examples to repo. <https://github.com/near/near-jsonrpc-client-rs/pull/32>
- Ensured `None`-typed wrapped errors are actual errors (i.e. have all traits attributed with errors, especially `fmt::Display`). <https://github.com/near/near-jsonrpc-client-rs/pull/34>

## [0.1.0] - 2021-11-11

> Release Page: <https://github.com/near/near-jsonrpc-client-rs/releases/tag/v0.1.0>

[unreleased]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.3.0...v0.4.0
[0.4.0-beta.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.3.0...v0.4.0-beta.0
[0.3.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/near/near-jsonrpc-client-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/near/near-jsonrpc-client-rs/releases/tag/v0.1.0
