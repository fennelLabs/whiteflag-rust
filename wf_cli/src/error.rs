use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhiteflagCLIError {
    #[error("must authenticate using `wf auth`")]
    AuthenticationRequired,
    #[error("whiteflag error")]
    WFError(#[from] fennel_whiteflag::WhiteflagError),
    #[error("wf_field::error")]
    WFFieldError(#[from] wf_field::Error),
    #[error("issue serializing struct")]
    SerdeJsonError(#[from] serde_json::error::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
