use warp::{Filter, filters::BoxedFilter};
use xkcd_wallpaper::xkcd;
use serde::{Serialize, Deserialize};

pub fn passthrough() -> BoxedFilter<(Vec<u8>,)>{
    warp::path!("xkcd" / u64)
        .and(warp::get())
        .and_then(|id| async move {
            let xkcd = xkcd::Xkcd::get(id).await;
            let xkcd = match xkcd {
                Ok(ok) => {ok},
                Err(_err) => {
                    return Err(warp::reject::reject());
                },
            };
            let image = xkcd.get_image().await.unwrap();
            Ok::<_, warp::Rejection>(image)
        })
        .boxed()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WallpaperParams{
    pub foreground: String,
    pub background: String,
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub pt: usize,
    #[serde(default)]
    pub pr: usize,
    #[serde(default)]
    pub pb: usize,
    #[serde(default)]
    pub pl: usize,
}

pub fn wallpaper() -> BoxedFilter<(Vec<u8>,)>{
    warp::path!("wallpaper" / String)
        .and(warp::get())
        .and(warp::query::<WallpaperParams>())
        .and_then(|id: String, params| async move {
            let xkcd = match id.as_str() {
                "newest" => {
                    xkcd::Xkcd::get_newest().await
                },
                _ => {
                    xkcd::Xkcd::get(id.parse::<u64>().unwrap()).await
                },
            };

            let xkcd = match xkcd {
                Ok(ok) => {ok},
                Err(_err) => {
                    return Err(warp::reject::reject());
                },
            };

            let image = xkcd.get_image().await.unwrap();

            let WallpaperParams{
                foreground,
                background,
                width,
                height,
                pt,
                pr,
                pb,
                pl,
            } = params;
            let wallpaper = xkcd_wallpaper::wallpaper::generate_wallpaper_hex(image, &foreground, &background, (width, height), (pt, pr, pb, pl));

            return Ok::<_, warp::Rejection>(wallpaper);
        })
        .boxed()
}