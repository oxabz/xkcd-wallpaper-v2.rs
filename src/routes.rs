use crate::error::Error;
use crate::wallpaper;
use crate::xkcd;
use serde::{Deserialize, Serialize};
use warp::{filters::BoxedFilter, Filter};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WallpaperParams {
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

#[derive(Debug)]
pub(crate) struct AnyhowReject(pub(crate)  anyhow::Error);

impl From<anyhow::Error> for AnyhowReject {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl warp::reject::Reject for AnyhowReject {}

pub fn wallpaper() -> BoxedFilter<(Vec<u8>,)> {
    warp::path!("wallpaper" / String)
        .and(warp::get())
        .and(warp::query::<WallpaperParams>())
        .and_then(|id: String, params: WallpaperParams| async move {
            let xkcd = match id.as_str() {
                "newest" => xkcd::Xkcd::get_newest().await,
                _ => xkcd::Xkcd::get(id.parse::<u64>().map_err(|_err| Error::InvalidXkcdId(id))?).await,
            }?;

            let image = xkcd.get_image().await?;

            let WallpaperParams {
                foreground,
                background,
                width,
                height,
                pt,
                pr,
                pb,
                pl,
            } = params;

            let wallpaper = wallpaper::generate_wallpaper_hex(
                image,
                &foreground,
                &background,
                (width, height),
                (pt, pr, pb, pl),
            )?;

            return Ok::<_, warp::Rejection>(wallpaper);
        })
        .boxed()
}