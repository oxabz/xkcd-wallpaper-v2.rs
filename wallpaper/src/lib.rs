pub mod xkcd;
pub mod wallpaper;

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

        let wallpaper = wallpaper::generate_wallpaper_hex(image, "#E94B3CFF", "#2D2926", (1366, 768), (20, 20, 20, 20));

        File::create("test.png").unwrap().write_all(&wallpaper).unwrap();
    }
}