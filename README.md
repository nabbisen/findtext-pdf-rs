# Find Text PDF

[![crates.io](https://img.shields.io/crates/v/findtext_pdf?label=latest)](https://crates.io/crates/findtext_pdf)
[![Documentation](https://docs.rs/findtext_pdf/badge.svg?version=latest)](https://docs.rs/findtext_pdf/latest)
[![Dependency Status](https://deps.rs/crate/findtext_pdf/latest/status.svg)](https://deps.rs/crate/findtext_pdf/latest)
[![Releases Workflow](https://github.com/nabbisen/findtext-pdf-rs/actions/workflows/release.yml/badge.svg)](https://github.com/nabbisen/findtext-pdf-rs/actions/workflows/)
[![License](https://img.shields.io/github/license/nabbisen/findtext-pdf-rs)](https://github.com/nabbisen/findtext-pdf-rs/blob/main/LICENSE)

## Summary

Search text in PDF

## Development

First, add dependency:

```sh
cargo add findtext_pdf
```

Usage:

```rust
use findtext_pdf::search;

fn awesome_fn(keyword: &str, filepath: &str) {
    let ret = search(keyword, filepath);
}
```

## Executable Usage

Available in [Assets](https://github.com/nabbisen/findtext-pdf-rs/releases/latest) in Releases. Cross-platform supported.

```sh
findtext_pdf <keyword> <filepath>
# will print out like:
# p.2: hej
```

## Acknowledgements

Depends on:

- [J-F-Liu/lopdf](https://github.com/J-F-Liu/lopdf)
