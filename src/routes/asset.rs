/*! 静态资源文件
 *
 */

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::RustEmbed;

#[derive(Debug, RustEmbed)]
#[folder = "web/dist/"]
struct Asset;

// 首页
#[get("/")]
pub async fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

// 静态资源
#[get("/static/<file..>")]
pub fn serve_embedded_file(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = format!("static/{}", file.display());
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}
