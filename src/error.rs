use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    API(String),

    #[error("{0}")]
    SignUp(String),

    #[error("{0}")]
    SignIn(String),

    #[error("{0}")]
    User(String),

    #[error("{0}")]
    Token(String),
}

impl std::convert::From<reqwest_middleware::reqwest::Error> for Error {
    fn from(err: reqwest_middleware::reqwest::Error) -> Self {
        Error::API(err.to_string())
    }
}

impl std::convert::From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        Error::API(err.to_string())
    }
}
