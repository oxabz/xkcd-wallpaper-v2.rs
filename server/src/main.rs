use warp;
use warp::Filter;
use xkcd_wallpaper_routes::passthrough;

#[tokio::main]
async fn main() {
    let routes = warp::any().and(warp::path::end()).map(|| "Hello, World!")
        .or(passthrough())
        .or(xkcd_wallpaper_routes::wallpaper());

    warp::serve(routes.with(warp::log("warp")))
        .run(([127, 0, 0, 1], 3030))
        .await;
}