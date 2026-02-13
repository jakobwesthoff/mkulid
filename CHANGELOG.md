# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## 1.0.0 - 2026-02-13

### Added

- Generate one or more ULIDs from the command line
- Monotonic ordering when generating multiple ULIDs (`-n`)
- Lowercase output (`-l` / `--lowercase`)
- Pin timestamp via Unix milliseconds (`--timestamp`)
- Pin timestamp via RFC 3339 datetime (`--datetime`)
- Inspect an existing ULID to display its timestamp and random payload
  (`--inspect`)
- Pre-built binaries for macOS (arm64, x86_64), Linux (x86_64, aarch64),
  and Windows (x86_64)
