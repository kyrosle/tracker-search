use crate::metainfo::TorrentMetaInfoBuilderError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("{0}")]
  UrlParseError(url::ParseError),

  #[error("{0}")]
  ReqwestError(reqwest::Error),

  #[error("Selector Parse Error")]
  SelectorParseError,
  #[error("{0}")]
  SelectorElementError(String),

  #[error("{0}")]
  TorrentMetaInfoBuildError(TorrentMetaInfoBuilderError),
  #[error("{0}")]
  TorrentMetaInfoSettingError(String),

  #[error("{0}")]
  OtherError(String),

  #[error("{0}")]
  ParseStorageSizeError(String),
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::ReqwestError(value)
  }
}

impl From<url::ParseError> for Error {
  fn from(value: url::ParseError) -> Self {
    Self::UrlParseError(value)
  }
}

impl From<TorrentMetaInfoBuilderError> for Error {
  fn from(value: TorrentMetaInfoBuilderError) -> Self {
    Self::TorrentMetaInfoBuildError(value)
  }
}

impl From<scraper::error::SelectorErrorKind<'_>> for Error {
  fn from(_: scraper::error::SelectorErrorKind<'_>) -> Self {
    Self::SelectorParseError
  }
}
