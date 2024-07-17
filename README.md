[![Project Status: Concept – Minimal or no implementation has been done yet, or the repository is only intended to be a limited example, demo, or proof-of-concept.](https://www.repostatus.org/badges/latest/concept.svg)](https://www.repostatus.org/#concept)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.74-orange)](https://www.rust-lang.org)
[![MIT License](https://img.shields.io/github/license/jwodder/nhmoon.svg)](https://opensource.org/licenses/MIT)

This is a Rust utility for measuring the sizes of individual [Zarr manifests][]
fetched by [`dandidav`][]: both the number of bytes in a manifest's raw JSON
source and the number of bytes used by the internal parsed representation.

Installation
============

Regardless of which installation method you choose, you need to first [install
Rust and Cargo](https://www.rust-lang.org/tools/install).

To install the `manifest-size` binary in `~/.cargo/bin`, run:

    cargo install --git https://github.com/jwodder/manifest-size

Alternatively, a binary localized to a clone of this repository can be built
with:

    git clone https://github.com/jwodder/manifest-size
    cd manifest-size
    cargo build  # or `cargo build --release` to enable optimizations
    # You can now run the binary with `cargo run -- <args>` while in this
    # repository.


Usage
=====

    manifest-size [-J|--json] <url>

or, if running a localized binary:

    cargo run [--release] -- [-J|--json] <url>

`manifest-size` takes a single positional argument, an HTTP(S) URL pointing to
a Zarr manifest file.  It downloads the file, parses it, and outputs the size
of the data before & after parsing.  If the `-J`/`--json` option is supplied,
the output will be in JSON.

Examples
========

```console
$ manifest-size https://datasets.datalad.org/dandi/zarr-manifests/zarr-manifests-v2-sorted/001/e3b/001e3b6d-26fb-463f-af28-520a25680ab4/326273bcc8730474323a66ea4e3daa49-113328--97037755426.json
Raw    response: 13 818 966 bytes
Parsed response: 18 473 917 bytes
```

```console
$ manifest-size --json https://datasets.datalad.org/dandi/zarr-manifests/zarr-manifests-v2-sorted/c7e/25d/c7e25dca-4dc9-4e83-a0d7-5fee56fa8773/c23f15b26134d808b072b8c93b1eeed8-48935--29709893986.json
{
  "url": "https://datasets.datalad.org/dandi/zarr-manifests/zarr-manifests-v2-sorted/c7e/25d/c7e25dca-4dc9-4e83-a0d7-5fee56fa8773/c23f15b26134d808b072b8c93b1eeed8-48935--29709893986.json",
  "raw_bytes": 5935826,
  "parsed_bytes": 7983735
}
```

[Zarr manifests]: https://github.com/dandi/dandidav/blob/main/doc/zarrman.md
[`dandidav`]: https://github.com/dandi/dandidav
