use std::convert::Infallible;

use thiserror::Error;
use warp::reply::WithStatus;
#[derive(Debug, Error)]
pub enum Error{
    #[error("Invalid color: {0}")]
    ColorParseError(String),
    #[error("Couldnt load image : {0}")]
    ImageError(#[from] image::ImageError),
    #[error("XKCD not found")]
    XkcdNotFound,
    #[error("xkcd returned status code {0}")]
    XkcdStatusCode(u16),
    #[error("reqwest error: {0}")]
    ReqwestError(reqwest::Error),  
    #[error("invalid xkcd id: {0}")]
    InvalidXkcdId(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if let Some(status) = err.status(){
            match status.as_u16() {
                404 => Error::XkcdNotFound,
                _ => Error::XkcdStatusCode(status.as_u16()),
            }
        } else {
            Error::ReqwestError(err)
        }
    }
}

impl warp::reject::Reject for Error {}

impl Error{
    fn reply(&self) -> WithStatus<String> {
        match self {
            Error::ColorParseError(color) => warp::reply::with_status(
                format!("Bad Request: Invalid color: {}", color),
                warp::http::StatusCode::BAD_REQUEST,
            ),
            Error::ImageError(err) => warp::reply::with_status(
                format!("Internal Server Error: {}", err),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Error::XkcdNotFound => warp::reply::with_status(
                "XKCD Not Found".to_string(),
                warp::http::StatusCode::NOT_FOUND,
            ),
            Error::XkcdStatusCode(code) => warp::reply::with_status(
                format!("XKCD returned status code {}", code),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Error::ReqwestError(err) => 
                warp::reply::with_status(
                    format!("Internal Server Error: {}", err),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ),
            Error::InvalidXkcdId(id) => warp::reply::with_status(
                format!("Bad Request: Invalid xkcd id: {}, expect a number or \"newest\"", id),
                warp::http::StatusCode::BAD_REQUEST,
            )
        }
    }
}

pub async fn rejection_handler(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    if err.is_not_found() {
        return Ok(warp::reply::with_status(
            "Not Found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
    if let Some(e) = err.find::<Error>() {
        return Ok(e.reply());
    }
    Ok(warp::reply::with_status(
        "Uknown Internal Server Error".to_string(),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}