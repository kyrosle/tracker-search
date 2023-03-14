use std::{fs, io::Write};

pub mod error;
// pub mod matcher;
pub mod metainfo;
pub mod tracker;
pub mod trackers;
pub mod unit;
pub mod matcher;

pub use error::*;

/// A trait to Help Tracker match the html pattern.
///
/// The Tracker should implement this trait.
// pub trait TrackerMetaMatch {
//   fn match_name(pattern: &str) -> String;
//   fn match_link(pattern: &str) -> String;
//   fn match_storage_size(pattern: &str) -> String;
//   fn match_tag(_pattern: &str) -> Option<String> {
//     None
//   }
//   fn match_seeds_number(_pattern: &str) -> Option<String> {
//     None
//   }
//   fn match_leeches_number(_pattern: &str) -> Option<String> {
//     None
//   }
//   fn match_upload_time(_pattern: &str) -> Option<String> {
//     None
//   }
// }

// --------- //
// handle the storage size string getting from html, and then standardize it.

/// convert a storage size string into the value and the size degree pattern string.
///
/// ## Error
/// the standardize failed, the format should like: `number-value degree-type`
fn convert_size_string(size_str: &str) -> Result<u64> {
  // Split the size string into numeric and alphabetic parts.
  let (size, unit_str) = split_size_string(size_str)?;

  // Convert the alphabetic part to a multiplier.
  let multiplier = match unit_str.to_uppercase().as_str() {
    "B" => 1.0,
    "KIB" | "KB" | "K" => 1024.0,
    "MIB" | "MB" | "M" => 1024.0 * 1024.0,
    "GIB" | "GB" | "G" => 1024.0 * 1024.0 * 1024.0,
    _ => {
      return Err(Error::ParseStorageSizeError(format!(
        "can not identify the string type: {}",
        unit_str
      )))
    }
  };

  // Multiply the numeric part by the multiplier.
  Ok((size * multiplier) as u64)
}

fn split_size_string(size_str: &str) -> Result<(f64, &str)> {
  // Find the index where the alphabetic part starts.
  let index = size_str
    .find(|c: char| c.is_alphabetic())
    .unwrap_or(size_str.len());

  // Extract the numeric and alphabetic parts.
  let size = size_str[..index]
    .parse::<f64>()
    .map_err(|e| Error::ParseStorageSizeError(e.to_string()))?;
  let unit = &size_str[index..];

  // Return the tuple with the two parts.
  Ok((size, unit))
}

// ----- //

// testing in order to help writing into the file convenient.
#[allow(dead_code)]
fn write_into_file(s: &[u8]) {
  let mut fs = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./index.html")
    .unwrap();
  fs.write_all(s).unwrap();
}
