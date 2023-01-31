
#[derive(thiserror::Error, Debug)]
pub enum UserError {

}

#[derive(thiserror::Error, Debug)]
pub enum InternalError {
  #[error("file error: {0}")]
  FileError(#[from] std::io::Error),

  #[error("error: {0}")]
  SomeError(&'static str, String),
}