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



#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::wallpaper;

    use super::xkcd;

    #[tokio::test]
    async fn test_xkcd() {
        let xkcd = xkcd::Xkcd::get_newest().await;

        println!("{:?}", xkcd);

        assert!(xkcd.is_ok());
    }

    #[tokio::test]
    async fn test_xkcd_id() {
        let xkcd = xkcd::Xkcd::get(1).await;

        println!("{:?}", xkcd);

        assert!(xkcd.is_ok());
    }

    #[tokio::test]
    async fn test_wallpaper() {
        let xkcd = xkcd::Xkcd::get(146).await.unwrap();
        let image =  xkcd.get_image().await.unwrap();

        let _wallpaper = wallpaper::generate_wallpaper_hex(image, "#E94B3C", "#2D2926", (1366, 768), (20, 20, 20, 20));

        //File::create("test.png").unwrap().write_all(&wallpaper).unwrap();
    }
}