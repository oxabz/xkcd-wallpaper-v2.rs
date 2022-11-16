use warp::Filter;
use warp::Reply;

#[shuttle_service::main]
async fn warp() -> shuttle_service::ShuttleWarp<(impl Reply,)> {
    let hello = warp::any().and(warp::path::end()).map(|| "Hello, World!");
    let wallpaper = xkcd_wallpaper_routes::wallpaper();
    let route = hello.or(wallpaper);
    Ok(route.boxed())
}
