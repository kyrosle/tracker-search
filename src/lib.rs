use std::{error::Error, fs, io::Write};

pub mod metainfo;
pub mod tracker;
pub mod matcher;
pub mod unit;
pub mod error;

pub use error::*;

pub fn fix_content(title: Vec<&str>) -> Vec<String> {
  title
    .into_iter()
    .map(|t| t.trim())
    .map(eliminate)
    .filter(|t| !t.is_empty())
    .collect()
}

fn eliminate(s: &str) -> String {
  s.replace(|c: char| -> bool { c == '\n' || c == '\t' }, "")
}

#[allow(dead_code)]
fn write_into_file(s: &[u8]) {
  let mut fs = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./index.html")
    .unwrap();
  fs.write_all(s).unwrap();
}
