use warp::Reply;
use warp::Filter;

pub mod xkcd;
pub mod wallpaper;
mod routes;

async fn rejection_handler(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    if err.is_not_found() {
        return Ok(warp::reply::with_status(
            "Not Found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
    if let Some(e) = err.find::<routes::AnyhowReject>() {
        if let Some(err) = e.0.downcast_ref::<wallpaper::Error>(){
            match err {
                wallpaper::Error::ColorParseError(color) => {
                    return Ok(warp::reply::with_status(
                        format!("Bad Request: Invalid color: {}", color),
                        warp::http::StatusCode::BAD_REQUEST,
                    ))
                }
            }
        } else {
            return Ok(warp::reply::with_status(
                "Internal Server Error".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
    Ok(warp::reply::with_status(
        "Internal Server Error".to_string(),
        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

//Serve routes using shuttle
#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {

    let hello = warp::path!("hello" / String)
        .and(warp::get())
        .map(|name| format!("Hello, {}!", name));
    let route = hello.or(routes::wallpaper().recover(rejection_handler));
    Ok(route.with(warp::log("wallpaper::api")).boxed().into())
}
