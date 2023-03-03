use std::{
  cell::Ref,
  ops::{Deref, DerefMut},
};

use crate::Result;
use derive_builder::Builder;

/// The Pattern using in `Selector::parse()`
///
/// the pattern value maybe like the [`table`] in following:
/// ```html
/// <table class="table table-hover file-list">
///   <tbody>
///     <tr>
///       <td>
///         <a href="/!g7DJ">
///          "Arma 3"
///           <b>Apex</b>
///           "Edtion"
///           <p class="sample">data2.bin</p>
///         </a>
///      </td>
///     </td>
///   </tbody>
/// </table>
/// ```
/// Here is to match the element of `table`
#[derive(Debug, Clone)]
pub struct Pattern {
  pub pattern: String,
}

impl Pattern {
  pub fn new(pattern: impl Into<String>) -> Self {
    Pattern {
      pattern: pattern.into(),
    }
  }
}

impl From<String> for Pattern {
  fn from(value: String) -> Self {
    Pattern::new(value)
  }
}

impl<'a> From<&'a str> for Pattern {
  fn from(value: &'a str) -> Self {
    Pattern::new(value.to_string())
  }
}

impl std::ops::Add for Pattern {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Pattern::new(format!("{}{}", self.pattern, rhs.pattern))
  }
}

impl Deref for Pattern {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.pattern
  }
}

/// The matcher pattern obtain the list table(before the torrent metainfo).
#[derive(Debug, Clone)]
pub struct MatchTablePatterns {
  pub table: Pattern,
}

impl Deref for MatchTablePatterns {
  type Target = String;

  fn deref(&self) -> &Self::Target {
    &self.table.pattern
  }
}

/// The torrent metainfo format would like:
/// +-------------------------+
/// +name                     +
/// +date                  tag+
/// +storage     seeds leeches+
#[derive(Debug, Clone, Builder)]
pub struct MatchMetaPatterns {
  pub href: Pattern,
  pub name: Pattern,
  pub tag: Pattern,
  // maybe should set validate function to check
  pub storage_size: Pattern,
  pub seeds_number: Pattern,
  pub leeches_number: Pattern,
  // maybe use `std::Duration::Instant` instead?
  pub upload_time: Pattern,
}

/// Matcher contains `table_matcher` matching the list table,
/// and the `meta_matcher` matching the torrent meta info.
///
///
/// what if accepting the patterns which wrapping a string, and then
/// build the series of `Selector` and then return the SearchResult?
#[derive(Debug, Clone)]
pub struct Matcher {
  pub table_matcher: MatchTablePatterns,
  pub meta_matcher: MatchMetaPatterns,
}

/// apply get the reference of fields for `Matcher`
impl Matcher {
  pub fn new(
    table_matcher: MatchTablePatterns,
    meta_matcher: MatchMetaPatternsBuilder,
  ) -> Result<Self> {
    Ok(Matcher {
      table_matcher,
      meta_matcher: meta_matcher.build()?,
    })
  }

  // -----------getter function------------
  pub fn get_href(&self) -> &Pattern {
    &self.meta_matcher.href
  }
  pub fn get_name(&self) -> &Pattern {
    &self.meta_matcher.name
  }
  pub fn get_tag(&self) -> &Pattern {
    &self.meta_matcher.tag
  }
  pub fn get_storage_size(&self) -> &Pattern {
    &self.meta_matcher.storage_size
  }
  pub fn get_leeches(&self) -> &Pattern {
    &self.meta_matcher.leeches_number
  }
  pub fn get_seeds(&self) -> &Pattern {
    &self.meta_matcher.seeds_number
  }
  pub fn get_upload_time(&self) -> &Pattern {
    &self.meta_matcher.upload_time
  }
}
