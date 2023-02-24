use tracker_search::tracker::Tracker;
use tracker_search::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let param = "apex";
  let match_list = "table tbody tr td";
  let match_url = "a";
  let url = r#"https://0mag.net"#;

  let tracker = Tracker::new("0magnet", url)?;

  let result = tracker.search(param, match_list, match_url, 98).await?;
  // println!("{:#?}", result);
  println!("{}", result.len());

  // tracker.into_file_html(param).await?;

  Ok(())
}
