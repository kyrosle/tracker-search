use tracker_search::matcher::MatchPatternsBuilder;
use tracker_search::tracker::Tracker;
use tracker_search::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let param = "apex";
  let match_list = "table tbody tr td a";
  let url = r#"https://0mag.net"#;

  let matcher = MatchPatternsBuilder::new();

  let tracker = Tracker::new("0magnet", url, match_list)?;

  let result = tracker.search(param, 5).await?;
  println!("{:#?}", result);
  // println!("{}", result.len());

  // tracker.into_file_html(param).await?;

  Ok(())
}
