use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    API(String),

    #[error("{0}")]
    NetworkMiddleware(#[from] reqwest_middleware::Error),

    #[error("{0}")]
    Network(#[from] reqwest_middleware::reqwest::Error),

    #[error("{0}")]
    SignUp(String),

    #[error("{0}")]
    SignIn(String),

    #[error("{0}")]
    User(String),

    #[error("{0}")]
    Token(String),
}
