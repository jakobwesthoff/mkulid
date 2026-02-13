# mkulid

A command-line ULID generator — like `uuidgen`, but for
[ULIDs](https://github.com/ulid/spec).

## Why

I needed sorted, random identifiers in shell scripts and pipelines.
I couldn't find a standalone ULID CLI, so I wrote one.

If you don't specifically need ULIDs, UUID v7 is a similar
time-sortable alternative — though its spec leaves sub-millisecond
ordering as an implementation choice rather than mandating strict
monotonicity.

## Installation

### From crates.io

```sh
cargo install mkulid
```

### From GitHub releases

Pre-built binaries are available on the
[Releases](https://github.com/jakobwesthoff/mkulid/releases) page.
Download the archive for your platform, extract it, and place the
`mkulid` binary somewhere on your `PATH`.

## Usage

Generate a single ULID:

```console
$ mkulid
01JMCX2F5GKQJ3YZT4BXNRP8WH
```

Generate a batch of five (monotonically ordered):

```console
$ mkulid -n 5
01JMCX2F5GKQJ3YZT4BXNRP8WH
01JMCX2F5GKQJ3YZT4BXNRP8WJ
01JMCX2F5GKQJ3YZT4BXNRP8WK
01JMCX2F5GKQJ3YZT4BXNRP8WM
01JMCX2F5GKQJ3YZT4BXNRP8WN
```

Lowercase output:

```console
$ mkulid -l
01jmcx2f5gkqj3yzt4bxnrp8wh
```

Pin the timestamp to a specific Unix epoch value (milliseconds):

```console
$ mkulid --timestamp 1716214200123
01HYG2ZS1VKQJ3YZT4BXNRP8WH
```

Pin the timestamp to an RFC 3339 datetime:

```console
$ mkulid --datetime "2024-05-20T14:30:00Z"
01HYG2ZS00KQJ3YZT4BXNRP8WH
```

Inspect an existing ULID:

```console
$ mkulid --inspect 01JMCX2F5GKQJ3YZT4BXNRP8WH
ULID:      01JMCX2F5GKQJ3YZT4BXNRP8WH
Timestamp: 2025-02-13T12:34:56.789+00:00
Unix ms:   1739446496789
Random:    0x003a7b4e5f6c8d9e0a1b
```

## Development

Building from source requires a Rust toolchain (1.85+, edition 2024).

```sh
git clone https://github.com/jakobwesthoff/mkulid.git
cd mkulid
cargo build --release
```

The binary will be at `target/release/mkulid`.

To install directly from a local checkout:

```sh
cargo install --path .
```

## License

[MPL-2.0](LICENSE)
