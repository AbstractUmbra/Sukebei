use scraper::{Html, Selector};
use serde::Deserialize;

/// An image representation.
#[derive(Deserialize, Debug)]
struct Image {
    /// 'h' is image height.
    h: i32,
    /// 't' is image type.
    t: String,
    /// 'w' is image width.
    w: i32,
}

/// nHentai's API returns an array of Image.
#[derive(Deserialize, Debug)]
struct Images {
    cover: Image,
    pages: Vec<Image>,
    thumbnail: Image,
}

/// nHentai tags
#[derive(Deserialize, Debug)]
struct Tag {
    count: i32,
    id: i32,
    name: String,
    r#type: String,
    url: String,
}

/// Title is two of three formats, English OR Japanese AND pretty.
#[derive(Deserialize, Debug)]
struct Title {
    english: String,
    japanese: String,
    pretty: String,
}

/// The full API response per Gallery.
#[derive(Deserialize, Debug)]
struct Response {
    id: i32,
    images: Images,
    media_id: String,
    num_favorites: i32,
    num_pages: i32,
    scanlator: String,
    tags: Vec<Tag>,
    title: Title,
    upload_date: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://nhentai.net/api/gallery/177013")
        .await?
        .json::<Response>()
        .await?;
    println!("{:#?}", resp);
    gallery().await
}

async fn gallery() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://nhentai.net/g/177013")
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&resp);
    let body_selector = Selector::parse(r#"img[class="lazyload"]"#).unwrap();

    for element in document.select(&body_selector) {
        println!("{:#?}", element.value().attr("data-src").unwrap())
    }

    Ok(())
}
