use tracker_search::matcher::MatcherMethod;
use tracker_search::tracker::Tracker;
use tracker_search::Result;

use tracker_search::trackers::magnet;

#[tokio::main]
async fn main() -> Result<()> {
  // tracker pattern:
  let search_param = "apex";
  let table_pattern = "table tbody tr";
  let url = r#"https://0mag.net"#;

  // tracker matcher
  // using the module([crate::trackers::magnet])
  let matcher = MatcherMethod::new(
    Box::new(magnet::take_name),
    Box::new(magnet::take_href_link),
    Box::new(magnet::take_information),
    Box::new(magnet::take_storage_size),
  );

  // create a new tracker for the website https://0mag.net
  let tracker = Tracker::new("0magnet", url, table_pattern, matcher)?;

  // startup searching and limitedly taking the first 5 result.
  let result = tracker.search(search_param, 5).await?;

  // if you want to check the table form;
  // tracker.into_file_html(param).await?;

  println!("Return the results count: {}", result.len());
  println!("Results are fellow: \n{:#?}", result);

  Ok(())
}

