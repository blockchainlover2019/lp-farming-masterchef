use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CustomError {
  #[error("Unauthorized")]
  Unauthorized {},
}