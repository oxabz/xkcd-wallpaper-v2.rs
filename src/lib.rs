use warp::Reply;
use warp::Filter;

pub mod xkcd;
pub mod wallpaper;
mod routes;

//Serve routes using shuttle
#[shuttle_service::main]
async fn warp() -> shuttle_service::ShuttleWarp<(impl Reply,)> {

    let hello = warp::path!("hello" / String)
        .and(warp::get())
        .map(|name| format!("Hello, {}!", name));
    let route = hello.or(routes::wallpaper());
    Ok(route.with(warp::log("wallpaper::api")).boxed())
}
