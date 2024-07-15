#[macro_use]
mod validstr;

mod component;
mod manifest;
use crate::manifest::Manifest;
use anyhow::Context;
use get_size::GetSize;
use std::io::Read;
use thousands::Separable;

fn main() -> anyhow::Result<()> {
    let Some(url) = std::env::args().nth(1) else {
        anyhow::bail!("No URL argument supplied");
    };
    let mut r = ureq::get(&url)
        .call()
        .context("GET request failed")?
        .into_reader();
    let mut body = Vec::new();
    r.read_to_end(&mut body)
        .context("failed to read response body")?;
    let body_len_str = body.len().separate_with_spaces();
    let parsed =
        serde_json::from_slice::<Manifest>(&body).context("failed to deserialize response")?;
    let parsed_size_str = parsed.get_size().separate_with_spaces();
    let width = body_len_str.len().max(parsed_size_str.len());
    println!("Raw    response: {body_len_str:>width$} bytes");
    println!("Parsed response: {parsed_size_str:>width$} bytes");
    Ok(())
}
