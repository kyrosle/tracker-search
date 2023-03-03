use crate::{matcher::MatchMetaPatternsBuilderError, metainfo::TorrentMetaInfoBuilderError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error<'static>>;

#[derive(Debug, Error)]
pub enum Error<'a> {
  #[error("{0}")]
  MatchPatternsError(MatchMetaPatternsBuilderError),

  #[error("{0}")]
  UrlParseError(url::ParseError),

  #[error("{0}")]
  ReqwestError(reqwest::Error),

  #[error("{0}")]
  SelectorParseError(scraper::error::SelectorErrorKind<'a>),
  #[error("{0}")]
  SelectorElementError(String),

  #[error("{0}")]
  TorrentMetaInfoBuildError(TorrentMetaInfoBuilderError),
}

impl<'a> From<MatchMetaPatternsBuilderError> for Error<'a> {
  fn from(value: MatchMetaPatternsBuilderError) -> Self {
    Self::MatchPatternsError(value)
  }
}

impl<'a> From<reqwest::Error> for Error<'a> {
  fn from(value: reqwest::Error) -> Self {
    Self::ReqwestError(value)
  }
}

impl<'a> From<url::ParseError> for Error<'a> {
  fn from(value: url::ParseError) -> Self {
    Self::UrlParseError(value)
  }
}

impl<'a> From<TorrentMetaInfoBuilderError> for Error<'a> {
  fn from(value: TorrentMetaInfoBuilderError) -> Self {
    Self::TorrentMetaInfoBuildError(value)
  }
}

impl<'a> From<scraper::error::SelectorErrorKind<'a>> for Error<'a> {
  fn from(value: scraper::error::SelectorErrorKind<'a>) -> Self {
    Self::SelectorParseError(value)
  }
}
