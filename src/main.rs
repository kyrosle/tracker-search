use std::{error::Error, fs, io::Write};

use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let param = "apex";
  let url = r#"https://0mag.net"#;

  let query = format!("{}/search?q={}", url, param);
  let resp = reqwest::get(query).await?;

  let body = resp.text().await?;

  let doc = Html::parse_fragment(&body);
  let table_selector = Selector::parse("table tbody tr td").unwrap();
  // let tobdy = Selector::parse("tbody").unwrap();

  let table = doc.select(&table_selector);

  let href_selector = Selector::parse("a").unwrap();
  // let name_selector = Selector::parse("b").unwrap();

  for e in table {
    if let Some(name) = e.select(&href_selector).next() {
      let name = name.text().collect::<Vec<_>>();
      println!("{}", fix_title(name));
    }
    if let Some(href) = e.select(&href_selector).next() {
      println!("{}", href.value().attr("href").unwrap());
    }
  }
  // .select(&tobdy)
  // .next()
  // .unwrap();
  // let s = table.html();
  // write_into_file(s.as_bytes());

  Ok(())
}

fn fix_title(title: Vec<&str>) -> String {
  title
    .into_iter()
    .map(|t| t.trim())
    .map(eliminate)
    .filter(|t| !t.is_empty())
    .collect::<Vec<_>>()
    .join(" ")
}

fn eliminate(s: &str) -> String {
  s.replace(|c: char| -> bool { c == '\n' || c == '\t' }, "")
}

fn write_into_file(s: &[u8]) {
  let mut fs = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./index.html")
    .unwrap();
  fs.write_all(s).unwrap();
}
