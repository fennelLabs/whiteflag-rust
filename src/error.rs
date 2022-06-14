use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhiteflagError {
    #[error("error")]
    Serde(serde_json::Error),
}
