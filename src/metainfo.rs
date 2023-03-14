use crate::{unit::Unit, Result};
use derive_builder::Builder;
use reqwest::Url;

/// Torrent meat info
/// 
/// Maybe the display format would like: 
/// ```
/// +-------------------------+
/// +name                     +
/// +date                  tag+
/// +storage     seeds leeches+
/// ```
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
      .field("storage size: ", &format!("{}", self.storage))
      .finish()
  }
}

// impl TorrentMetaInfo {
//   fn url(&mut self, main_url: &Url, url: &str) -> Result<Url> {
//     let url = format!("{}{}", main_url, url.trim_matches('/'));
//     Url::parse(&url).map_err(|e| e.into())
//   }
// }

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