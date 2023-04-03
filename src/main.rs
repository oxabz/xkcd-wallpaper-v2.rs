use warp::Filter;
use warp::Reply;

pub mod error;
mod routes;
pub mod wallpaper;
pub mod xkcd;

//Serve routes using shuttle
#[shuttle_runtime::main]
async fn warp() -> shuttle_warp::ShuttleWarp<(impl Reply,)> {
    let hello = warp::path!("hello" / String)
        .and(warp::get())
        .map(|name| format!("Hello, {}!", name));
    let route = hello.or(routes::wallpaper().recover(error::rejection_handler));
    Ok(route.with(warp::log("wallpaper::api")).boxed().into())
}
