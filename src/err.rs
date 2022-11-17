use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Color Error: {0} is not a valid color \n details : {1}")]
    ColorError(String, anyhow::Error),
}

pub fn to_color_err<F>(color: String) -> impl FnOnce(anyhow::Error) -> Error {
    |e| Error::ColorError(color, e)
}