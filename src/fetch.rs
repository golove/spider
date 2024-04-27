use crate::database::Database;
// use crate::{ImgDetail, Picture,PageInfo};
use chrono::Local;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use rand::Rng;

use bytes::Bytes;
use reqwest::{Client, Error};
use scraper::{html, Html, Selector};

use spider::{ImgDetail, PageInfo};


// use anyhow::{Result, anyhow};

pub async fn fetch_page(url: &str) -> Result<Html,Error> {
    let client = Client::new();
    // 尝试发送 HTTP 请求
    let response = client.get(url).send().await?;

    // 尝试读取响应体作为字符串
    let body = response.text().await?;

    // 解析页面内容
    Ok(Html::parse_document(&body))
}

pub fn parse_page(html: &Html) -> Vec<PageInfo> {
    let selector = Selector::parse("tr:nth-of-type(n+8) h3 a").expect("Failed to parse selector");
    let mut pages_info = Vec::new();
    for element in html.select(&selector) {
        let title = element.inner_html();
        if let Some(href) = element.value().attr("href") {
            let page_info: PageInfo = PageInfo {
                title: title.to_string(),
                href: "http://dkleh8.xyz/pw/".to_string() + href,
            };
            pages_info.push(page_info);
        }
    }
    pages_info
}

pub fn parse_picture(html: &Html) -> Vec<String> {
    let selector = Selector::parse("div.f14 img").expect("Failed to parse selector");
    let mut srcs = Vec::new();
    for element in html.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            // print!("src: {}", src);
            srcs.push(src.to_string());
        }
    }
    srcs
}

pub async fn fetch_img(src: &str) -> Result<Bytes,Error> {
    let client = Client::new();
    // 尝试发送 HTTP 请求
    let response = client.get(src).send().await?;

    // 尝试读取响应体作为字节数组
    let bytes = response.bytes().await?;

    // 解析图片内容
    Ok(bytes)
}

pub fn get_image_dimensions(bytes: &Bytes) -> Result<f32,Box<dyn std::error::Error>> {
    let image = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let (width, height) = image.dimensions();
    Ok((width as f32) / (height as f32))
}
