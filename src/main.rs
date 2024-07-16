#[macro_use]
mod validstr;

mod component;
mod manifest;
use crate::manifest::Manifest;
use anyhow::Context;
use get_size::GetSize;
use std::io::{self, BufReader, Read};
use thousands::Separable;

fn main() -> anyhow::Result<()> {
    let Some(url) = std::env::args().nth(1) else {
        anyhow::bail!("No URL argument supplied");
    };
    let mut counter = CountingReader::new(
        ureq::get(&url)
            .call()
            .context("GET request failed")?
            .into_reader(),
    );
    let parsed = serde_json::from_reader::<_, Manifest>(BufReader::new(&mut counter))
        .context("failed to deserialize response")?;
    let body_len_str = counter.bytes_read().separate_with_spaces();
    let parsed_size_str = parsed.get_size().separate_with_spaces();
    let width = body_len_str.len().max(parsed_size_str.len());
    println!("Raw    response: {body_len_str:>width$} bytes");
    println!("Parsed response: {parsed_size_str:>width$} bytes");
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CountingReader<R> {
    inner: R,
    bytes_read: usize,
}

impl<R> CountingReader<R> {
    fn new(inner: R) -> Self {
        CountingReader {
            inner,
            bytes_read: 0,
        }
    }

    fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let qty = self.inner.read(buf)?;
        self.bytes_read += qty;
        Ok(qty)
    }
}
