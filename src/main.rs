// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use clap::Parser;
use ulid::{Generator, Ulid};

// =========================================================
// CLI Definition
// =========================================================

/// A command-line ULID generator — like uuidgen, but for ULIDs.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Parse and display the components of an existing ULID.
    #[arg(long, conflicts_with_all = ["timestamp", "datetime", "count", "lowercase"])]
    inspect: Option<String>,

    /// Pin the timestamp to a specific Unix epoch value in milliseconds.
    #[arg(long, conflicts_with = "datetime")]
    timestamp: Option<u64>,

    /// Pin the timestamp to an RFC 3339 / ISO 8601 datetime string.
    #[arg(long, conflicts_with = "timestamp")]
    datetime: Option<String>,

    /// Number of ULIDs to generate.
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: u32,

    /// Output in lowercase.
    #[arg(short, long)]
    lowercase: bool,
}

// =========================================================
// Main Dispatch
// =========================================================

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(ref input) = cli.inspect {
        inspect_ulid(input)?;
    } else {
        generate_ulids(&cli)?;
    }

    Ok(())
}

// =========================================================
// Inspect Mode
// =========================================================

/// Parses an existing ULID string and prints its components: the canonical
/// representation, the embedded timestamp (both as ISO 8601 and raw Unix
/// milliseconds), and the 80-bit random payload as a hex string.
fn inspect_ulid(input: &str) -> Result<()> {
    let ulid: Ulid = input
        .parse()
        .with_context(|| format!("parse `{input}` as ULID"))?;

    let timestamp_ms = ulid.timestamp_ms();

    // Convert the ULID's embedded SystemTime into a chrono DateTime for
    // human-readable ISO 8601 formatting.
    let datetime: DateTime<Utc> = ulid.datetime().into();

    // The random component is 80 bits wide.  We format it as a zero-padded
    // 20-digit hex string so the width is always consistent.
    let random = ulid.random();

    println!("ULID:      {ulid}");
    println!("Timestamp: {}", datetime.to_rfc3339());
    println!("Unix ms:   {timestamp_ms}");
    println!("Random:    0x{random:020x}");

    Ok(())
}

// =========================================================
// Generate Mode
// =========================================================

/// Generates one or more ULIDs, optionally pinned to a specific timestamp.
/// When multiple ULIDs are requested, a monotonic generator ensures each
/// successive value is strictly greater than the previous one — even when
/// they share the same millisecond timestamp.
fn generate_ulids(cli: &Cli) -> Result<()> {
    let pinned_time = resolve_timestamp(cli)?;
    let mut generator = Generator::new();

    for _ in 0..cli.count {
        let ulid = match pinned_time {
            Some(st) => generator
                .generate_from_datetime(st)
                .context("generate ULID from pinned timestamp (random bits overflow)"),
            None => generator
                .generate()
                .context("generate ULID (random bits overflow)"),
        }?;

        let formatted = if cli.lowercase {
            ulid.to_string().to_lowercase()
        } else {
            ulid.to_string()
        };

        println!("{formatted}");
    }

    Ok(())
}

// =========================================================
// Timestamp Resolution
// =========================================================

/// Resolves the optional `--timestamp` or `--datetime` flags into a
/// `SystemTime`.  Clap's `conflicts_with` attribute already prevents both
/// from being supplied simultaneously, so at most one branch is taken.
fn resolve_timestamp(cli: &Cli) -> Result<Option<SystemTime>> {
    if let Some(ms) = cli.timestamp {
        return Ok(Some(UNIX_EPOCH + Duration::from_millis(ms)));
    }

    if let Some(ref dt_str) = cli.datetime {
        let dt = DateTime::parse_from_rfc3339(dt_str)
            .with_context(|| format!("parse `{dt_str}` as RFC 3339 datetime"))?;

        // chrono DateTimes before the Unix epoch would underflow when
        // converting to SystemTime via Duration::from_millis.  Reject them
        // explicitly with a clear message.
        let millis = dt.timestamp_millis();
        if millis < 0 {
            bail!("datetime `{dt_str}` is before the Unix epoch, which ULIDs cannot represent");
        }

        return Ok(Some(UNIX_EPOCH + Duration::from_millis(millis as u64)));
    }

    Ok(None)
}
