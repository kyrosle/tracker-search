use reqwest::Url;
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::{
  fix_content,
  matcher::{MatchMetaPatternsBuilder, MatchTablePatterns, Matcher},
  metainfo::{TorrentMetaInfo, TorrentMetaInfoBuilder},
};

use super::error::Error::*;
use super::Result;

#[derive(Debug, Clone)]
pub struct Tracker {
  pub name: String,
  pub url: Url,
  pub matcher: Matcher,
}

impl Tracker {
  pub fn new(
    name: &str,
    url: &str,
    match_table: MatchTablePatterns,
    match_meta: MatchMetaPatternsBuilder,
  ) -> Result<Self> {
    let url = url.trim_end_matches(&['/']);
    let url = Url::from_str(url)?;
    Ok(Tracker {
      name: name.into(),
      url,
      matcher: Matcher::new(match_table, match_meta)?,
    })
  }
  pub fn search_url(&self, param: &str) -> Result<Url> {
    let url = format!("{}search?q={}", self.url, param);
    Ok(Url::parse(&url)?)
  }

  pub fn href_url(&self, href: &str) -> Result<Url> {
    let href = href.trim_matches('/');
    let url = format!("{}/{}", self.url, href);
    Ok(Url::parse(&url)?)
  }

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

  pub async fn search(&self, param: &str, limit: usize) -> Result<Vec<TorrentMetaInfo>> {
    // request the html page
    let resp = reqwest::get(self.search_url(param)?).await?;
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);

    let Matcher {
      table_matcher,
      meta_matcher,
    } = self.matcher.clone();

    let table_pattern = table_matcher.table.pattern;
    let table_selector = Selector::parse(&table_pattern)?;
    let table = doc.select(&table_selector);

    let mut search_match_vec = vec![];

    for content in table.take(limit) {
      let mut match_result_builder = TorrentMetaInfoBuilder::default();

      let link = content
        .value()
        .attr("href")
        .ok_or(SelectorElementError("no found href".into()))?;
      match_result_builder.info_url(&self.url, link)?;

      let mut content = fix_content(content.text().collect::<Vec<_>>());
      let name = content.drain(..1).collect::<Vec<String>>();
      let name = name
        .first()
        .ok_or(SelectorElementError("match href is none".into()))?;
      match_result_builder.name(name);

      let info = if content.is_empty() {
        None
      } else {
        Some(content.join(" "))
      };
      match_result_builder.information(info);

      match_result_builder.storage(43);

      search_match_vec.push(match_result_builder.build()?);
    }
    Ok(search_match_vec)
  }
}
