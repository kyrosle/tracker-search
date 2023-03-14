use reqwest::Url;
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::{
  convert_size_string,
  matcher::MatcherMethod,
  metainfo::{TorrentMetaInfo, TorrentMetaInfoBuilder},
};

use super::Result;

/// A tracker adaptive the tracker-url.
#[derive(Debug)]
pub struct Tracker {
  /// The name of tracker.
  pub name: String,
  /// The source url from torrents website.
  pub url: Url,
  /// The matcher pattern to match the table form.
  table: String,
  /// The v
  pub matcher_method: MatcherMethod,
}

impl Tracker {
  pub fn new(name: &str, url: &str, table: &str, matcher: MatcherMethod) -> Result<Self> {
    // the postprocessing of url:
    // Accepting: https://example.com
    let url = url.trim_end_matches(&['/']);
    let url = Url::from_str(url)?;

    Ok(Tracker {
      name: name.to_string(),
      url,
      table: table.to_string(),
      matcher_method: matcher,
    })
  }

  /// Return the tracker name, default is the website url.
  pub fn name(&self) -> String {
    self.url.to_string()
  }

  /// Return the form url, which it to request the result with a param key word.
  ///
  /// It will return the Error if the param contain some illegality words.
  pub fn search_url(&self, param: &str) -> Result<Url> {
    let url = format!("{}search?q={}", self.url, param);
    Ok(Url::parse(&url)?)
  }

  /// Startup the searching stage.
  pub async fn search(&self, param: &str, limit: usize) -> Result<Vec<TorrentMetaInfo>> {
    // request the html page
    let resp = reqwest::get(self.search_url(param)?).await?;
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);

    // match the table form.

    let table_selector = Selector::parse(&self.table)?;
    let table = doc.select(&table_selector);
    // dbg!(&table.next().unwrap().html());
    let mut search_match_vec = vec![];

    for content in table.take(limit) {
      let mut result_builder = TorrentMetaInfoBuilder::default();

      // take the href link
      let match_link = self.matcher_method.match_link.as_ref()(content)?;
      // dbg!(&match_link);
      result_builder.info_url(&self.url, &match_link)?;


      // take name
      let match_name = self.matcher_method.match_name.as_ref()(content)?;
      // dbg!(&match_name);
      result_builder.name(match_name);


      // take the information
      let match_information = self.matcher_method.match_information.as_ref()(content)?;
      // dbg!(&match_information);
      result_builder.information(Some(match_information));

      // take storage size
      let match_storage_size = self.matcher_method.match_storage_size.as_ref()(content)?;
      // dbg!(&match_storage_size);
      result_builder.storage(convert_size_string(match_storage_size.trim())?);

      search_match_vec.push(result_builder.build()?);
    }
    Ok(search_match_vec)
  }

  /// A method using for testing, write the table form matched into file, conveniently to check.
  pub async fn into_file_html(&self, param: &str) -> Result<()> {
    let resp = reqwest::get(self.search_url(param)?).await?;
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);
    let table_selector = Selector::parse("table").unwrap();
    let mut table = doc.select(&table_selector);
    let table = table.next().unwrap();
    super::write_into_file(table.html().as_bytes());
    Ok(())
  }
}
