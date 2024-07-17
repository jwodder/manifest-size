#[macro_use]
mod validstr;

mod component;
mod manifest;
use crate::manifest::Manifest;
use anyhow::Context;
use clap::Parser;
use get_size::GetSize;
use std::io::Read;
use thousands::Separable;
use url::Url;

/// Display the raw & parsed sizes of a Zarr manifest
///
/// See <https://github.com/jwodder/manifest-size> for more information.
#[derive(Clone, Debug, Eq, Parser, PartialEq)]
struct Arguments {
    /// Output JSON
    #[arg(short = 'J', long)]
    json: bool,

    /// An HTTP(S) URL pointing to a Zarr manifest file
    url: Url,
}

fn main() -> anyhow::Result<()> {
    let args = Arguments::parse();
    let mut r = ureq::get(args.url.as_str())
        .call()
        .context("GET request failed")?
        .into_reader();
    let mut body = Vec::new();
    r.read_to_end(&mut body)
        .context("failed to read response body")?;
    let body_len = body.len();
    let parsed =
        serde_json::from_slice::<Manifest>(&body).context("failed to deserialize response")?;
    let parsed_size = parsed.get_size();
    drop(parsed);
    if args.json {
        println!(
            "{:#}",
            serde_json::json!({
                "url": args.url,
                "raw_bytes": body_len,
                "parsed_bytes": parsed_size,
            })
        );
    } else {
        let body_len_str = body_len.separate_with_spaces();
        let parsed_size_str = parsed_size.separate_with_spaces();
        let width = body_len_str.len().max(parsed_size_str.len());
        println!("Raw    response: {body_len_str:>width$} bytes");
        println!("Parsed response: {parsed_size_str:>width$} bytes");
    }
    Ok(())
}
