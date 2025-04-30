#![feature(io_error_more)]
#[derive(Debug,thiserror::Error)]
pub enum CustomError {
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    FsIOError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("eof")]
    EOF,
    #[error("invalid offset")]
    InvalidOffset,
    #[error("unknow char {0}")]
    UnknowChar(String),
    #[error("invalid syntax")]
    InvalidSyntax,
    #[error("func not exist {0}")]
    FuncNotExist(String)
}