use crate::Result;
use scraper::{ElementRef, Selector};

pub fn take_href_link(content: ElementRef) -> Result<String> {
  let link_selector = Selector::parse("td a")?;

  Ok(
    content
      .select(&link_selector)
      .next()
      .unwrap()
      .value()
      .attr("href")
      .unwrap()
      .to_string(),
  )
}

pub fn take_name(content: ElementRef) -> Result<String> {
  let name_selector = Selector::parse("td a")?;
  Ok(
    content
      .select(&name_selector)
      .next()
      .unwrap()
      .text()
      .map(|s| format!("{} ", s.trim()))
      .take(3)
      .collect::<String>()
      .trim()
      .to_string(),
  )
}

pub fn take_information(content: ElementRef) -> Result<String> {
  let info_selector = Selector::parse("td a")?;
  Ok(
    content
      .select(&info_selector)
      .next()
      .unwrap()
      .text()
      .map(|s| s.trim().to_string())
      .skip(3)
      .collect::<String>()
      .trim()
      .to_string(),
  )
}

pub fn take_storage_size(content: ElementRef) -> Result<String> {
  let storage_selector = Selector::parse("td.td-size")?;
  Ok(
    content
      .select(&storage_selector)
      .next()
      .unwrap()
      .text()
      .collect::<String>(),
  )
}
