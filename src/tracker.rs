use reqwest::Url;
use scraper::{Html, Selector};
use std::str::FromStr;

use crate::fix_content;

use super::Result;

#[derive(Debug, Clone)]
pub struct Tracker {
  pub name: String,
  pub url: Url,
}

#[derive(Default, Debug)]
pub struct SearchMatchBuilder {
  pub name: Option<String>,
  pub url: Option<Url>,
  pub info: Option<String>,
}

impl SearchMatchBuilder {
  pub fn name(&mut self, name: &str) {
    self.name = Some(name.to_string());
  }
  pub fn url(&mut self, main_url: &Url, url: &str) -> Result<()> {
    let url = format!("{}{}", main_url, url.trim_matches('/'));
    self.url = Some(Url::parse(&url)?);
    Ok(())
  }
  pub fn info(&mut self, info: Option<String>) {
    self.info = info;
  }
  pub fn build(self) -> Result<SearchMatch> {
    Ok(SearchMatch {
      name: self.name.unwrap(),
      url: self.url.unwrap(),
      info: self.info,
    })
  }
}

#[derive(Clone)]
pub struct SearchMatch {
  pub name: String,
  pub url: Url,
  pub info: Option<String>,
}

impl std::fmt::Debug for SearchMatch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Search Match Result")
      .field("file name: ", &self.name)
      .field("url: ", &self.url.to_string())
      .field("info: ", &self.info.clone().unwrap_or(String::new()))
      .finish()
  }
}

impl SearchMatch {
  pub fn new(name: String, url: String, info: Option<String>) -> Result<Self> {
    Ok(SearchMatch {
      name,
      url: Url::parse(&url)?,
      info,
    })
  }
}

impl Tracker {
  pub fn new(name: &str, url: &str) -> Result<Self> {
    let url = url.trim_end_matches(&['/']);
    let url = Url::from_str(url)?;
    Ok(Tracker {
      name: name.into(),
      url,
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

  pub async fn search(
    &self,
    param: &str,
    match_list: &str,
    match_url: &str,
    limit: usize,
  ) -> Result<Vec<SearchMatch>> {
    let resp = reqwest::get(self.search_url(param)?).await?;
    let body = resp.text().await?;
    let doc = Html::parse_fragment(&body);
    let table_selector = Selector::parse(match_list).unwrap();
    let table = doc.select(&table_selector);

    let href_selector = Selector::parse(match_url).unwrap();

    let mut search_match_vec = vec![];

    // FIXME: when the limit is 98, here iteration only return counts of 49
    // when the limit is 100000 (bigger than the element total count), here will return 98(which is the correct count).j
    for e in table.take(limit) {
      if let Some(content) = e.select(&href_selector).next() {
        let mut match_result_builder = SearchMatchBuilder::default();

        let link = content
          .value()
          .attr("href")
          .map_or(Err("no found href"), Ok)?;
        match_result_builder.url(&self.url, link)?;

        let mut content = fix_content(content.text().collect::<Vec<_>>());
        let name = content.drain(..1).collect::<Vec<String>>();
        let name = name.first().map_or(Err("match href is none"), Ok)?;
        match_result_builder.name(name);

        let info = if content.is_empty() {
          None
        } else {
          Some(content.join(" "))
        };
        match_result_builder.info(info);

        search_match_vec.push(match_result_builder.build()?);
      }
    }
    Ok(search_match_vec)
  }
}
