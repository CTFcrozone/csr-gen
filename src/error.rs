use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
    #[from]
    OpenSSL(openssl::error::ErrorStack),
    #[from]
    Utf8(std::string::FromUtf8Error),
    #[from]
    Io(std::io::Error),

    #[display("All fields are required.")]
    AllFieldsRequired,
}

impl std::error::Error for Error {}
