# Pumice

[![MIT licensed][mit-badge]][mit-url]
[![Latest version](https://img.shields.io/crates/v/pumice.svg)](https://crates.io/crates/pumice)
[![Documentation](https://docs.rs/pumice/badge.svg)](https://docs.rs/pumice)

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/K83FJ3M4/pumice/blob/main/LICENSE

## 🚧 This project is at an early stage of development 🚧

Pumice currently exposes an `unsafe` Vulkan API. There are plans to add API validation
in the future, which should return errors for invalid API usage, while
maintaining the current similarity to the original C API.

## Changes to the original API

- All method parameters or struct fields that represent a pair of `count` and `ptr`
  have been combined into a single `slice`.
- Array pointers without a direct count are also represented as slices.
- `VkAllocationCallbacks` are always set to `null`.
- Struct fields that are either unused or only have one valid value are omitted.
  This specifically applies to `pNext` and `sType` fields.
- Structs that have no fields due to the above are also omitted.
- All `Vk` prefixes have been removed from identifiers.
- Methods are always associated with a handle. The respective method name is shortened
  to no longer include the handle type.
- Optional pointers in structs are represented as `Option<T>`.
- Methods that write their results to a pointer will return the value directly. This also
  applies to methods that return multiple values.
- `str` references are used instead of c strings.
- `bool` is used instead of `VkBool32`.
- `Result` is used instead of `VkResult`.

## Versioning

Version 1.0.0 of the Vulkan spec is fully implemented. This functionality is exposed
through the project root. Any future extensions will be guarded by feature flags
and live in their own modules.