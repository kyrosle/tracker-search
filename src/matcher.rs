use std::fmt;

use crate::Result;
use scraper::ElementRef;

pub type Pattern = Box<dyn Fn(ElementRef) -> Result<String>>;

/// A structure to Help Tracker match the html meta information,
/// through the method provided.
///
/// The matching of name, link, storage_size is necessary, otherwise
/// would set as default None.
pub struct MatcherMethod {
  pub match_name: Pattern,
  pub match_link: Pattern,
  pub match_information: Pattern,
  pub match_storage_size: Pattern,
  pub match_tag: Option<Pattern>,
  pub match_seeds_number: Option<Pattern>,
  pub match_leeches_number: Option<Pattern>,
  pub match_upload_time: Option<Pattern>,
}

impl MatcherMethod {
  pub fn new(
    match_name: Pattern,
    match_link: Pattern,
    match_information: Pattern,
    match_storage_size: Pattern,
  ) -> Self {
    MatcherMethod {
      match_name,
      match_link,
      match_storage_size,
      match_information,
      match_tag: None,
      match_seeds_number: None,
      match_leeches_number: None,
      match_upload_time: None,
    }
  }

  pub fn set_tag_pattern(&mut self, pattern: Pattern) {
    self.match_tag = Some(pattern);
  }

  pub fn set_seeds_number_pattern(&mut self, pattern: Pattern) {
    self.match_seeds_number = Some(pattern);
  }

  pub fn set_leeches_number_pattern(&mut self, pattern: Pattern) {
    self.match_leeches_number = Some(pattern);
  }

  pub fn set_upload_time(&mut self, pattern: Pattern) {
    self.match_upload_time = Some(pattern);
  }
}

impl fmt::Debug for MatcherMethod {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut builder = f.debug_struct("MatcherMethod");
    builder.field("match_name", &"true".to_string());
    builder.field("match_link", &"true".to_string());
    builder.field("match_storage_size", &"true".to_string());
    if self.match_tag.is_some() {
      builder.field("match_tag", &"true".to_string());
    } else {
      builder.field("match_tag", &"false".to_string());
    }
    if self.match_seeds_number.is_some() {
      builder.field("match_seeds_number", &"true".to_string());
    } else {
      builder.field("match_seeds_number", &"false".to_string());
    }
    if self.match_leeches_number.is_some() {
      builder.field("match_leeches_number", &"true".to_string());
    } else {
      builder.field("match_leeches_number", &"false".to_string());
    }
    if self.match_upload_time.is_some() {
      builder.field("match_upload_time", &"true".to_string());
    } else {
      builder.field("match_upload_time", &"false".to_string());
    }
    builder.finish()
  }
}
