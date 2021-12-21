use anyhow::{Error, Result};
use std::io::BufRead;

pub(crate) fn parse_header(input: &mut impl BufRead) -> Result<usize> {
    let header = read_header(input)?;
    let content_length = header
        .lines()
        .find(|line| line.starts_with("Content-Length"))
        .ok_or(Error::msg("bad header"))?;

    let len: usize = content_length
        .split(":")
        .nth(1)
        .ok_or(Error::msg("bad header"))?
        .parse()?;

    Ok(len)
}

fn read_header(input: &mut impl BufRead) -> Result<String> {
    let mut line = String::new();
    let mut header = String::new();
    while line != "\r\n" {
        line.clear();
        input.read_line(&mut line)?;
        header.push_str(line.as_str());
    }
    return Ok(header);
}
