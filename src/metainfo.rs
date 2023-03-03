use crate::{unit::Unit, Result};
use derive_builder::Builder;
use reqwest::Url;

#[derive(Clone, Builder)]
pub struct TorrentMetaInfo {
  #[builder(setter(into))]
  pub name: String,
  #[builder(setter(custom = true))]
  pub info_url: Url,
  #[builder(default = "Option::None")]
  pub date: Option<String>,
  #[builder(default = "Option::None")]
  pub seeds: Option<usize>,
  #[builder(default = "Option::None")]
  pub leeches: Option<usize>,
  #[builder(default = "Option::None")]
  pub tag: Option<String>,
  #[builder(setter(custom = true))]
  pub storage: Unit,
  #[builder(default = "Option::None")]
  pub information: Option<String>,
}

impl std::fmt::Debug for TorrentMetaInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Search Match Result")
      .field("file name: ", &self.name)
      .field("info url: ", &self.info_url.to_string())
      .field(
        "information: ",
        &self.information.clone().unwrap_or(String::new()),
      )
      .finish()
  }
}

impl TorrentMetaInfo {
  fn url(&mut self, main_url: &Url, url: &str) -> Result<Url> {
    let url = format!("{}{}", main_url, url.trim_matches('/'));
    Url::parse(&url).map_err(|e| e.into())
  }
}

impl TorrentMetaInfoBuilder {
  pub fn info_url(&mut self, main_url: &Url, url: &str) -> Result<()> {
    let url = format!("{}{}", main_url, url.trim_matches('/'));
    self.info_url = Some(Url::parse(&url)?);
    Ok(())
  }
  pub fn storage(&mut self, size: u64) {
    self.storage = Some(Unit::new(size));
  }
}

// use crate `derive_builder` instead.
//
// /// Helper to Create a Matching Search Result.
// #[derive(Default, Debug)]
// pub struct SearchMatchBuilder {
//   pub name: Option<String>,
//   pub url: Option<Url>,
//   pub info: Option<String>,
// }

// impl SearchMatchBuilder {
//   pub fn name(&mut self, name: &str) {
//     self.name = Some(name.to_string());
//   }
//   pub fn url(&mut self, main_url: &Url, url: &str) -> Result<()> {
//     let url = format!("{}{}", main_url, url.trim_matches('/'));
//     self.url = Some(Url::parse(&url)?);
//     Ok(())
//   }
//   pub fn info(&mut self, info: Option<String>) {
//     self.info = info;
//   }
//   pub fn build(self) -> Result<SearchMatch> {
//     Ok(SearchMatch {
//       name: self.name.unwrap(),
//       info_url: self.url.unwrap(),
//       information: self.info,
//     })
//   }
// }
