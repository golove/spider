use std::fmt;
use scraper::{Html, Selector};
use reqwest::{header::CONTENT_TYPE, Client, Error as ReqwestError};
use bytes::Bytes;
use uuid::Uuid;
use spider::{ImgDetail, PageInfo, Picture};
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::error::Error;


#[derive(Debug)]
pub enum MyError {
    ContentTypeError,
    ReqwestError(ReqwestError),
    StdError(Box<dyn Error>),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred: {:?}", self)
    }
}
impl Error for MyError {}

unsafe impl Send for MyError {}

// If needed, implement Sync as well
unsafe impl Sync for MyError {}



async fn fetch_page(url: &str) -> Result<Html, MyError> {
    let client = Client::new();
    match client.get(url).send().await {
        Ok(response) => {
            let content_type = response.headers().get(CONTENT_TYPE)
                .ok_or(MyError::ContentTypeError)?
                .to_str()
                .map_err(|_| MyError::ContentTypeError)?;
            
            if content_type.starts_with("text/html") {
                let body = response.text().await.map_err(MyError::ReqwestError)?;
                let html = Html::parse_document(&body);
                Ok(html)
            } else{
                Err(MyError::ContentTypeError)
            }
        },
        Err(e) => Err(MyError::ReqwestError(e)),
    }
}

async fn fetch_img(src: &str)->Result<Bytes, MyError>{
    let client = Client::new();
    match client.get(src).send().await {
        Ok(response) => {
            let content_type = response.headers().get(CONTENT_TYPE)
            .ok_or(MyError::ContentTypeError)?
            .to_str()
            .map_err(|_| MyError::ContentTypeError)?;
        if content_type.starts_with("image/") {
            let bytes = response.bytes().await.map_err(MyError::ReqwestError)?;
            Ok(bytes)
        } else{
            Err(MyError::ContentTypeError)
        }
        }
        
        Err(e) => Err(MyError::ReqwestError(e)),
    }
}


pub async fn spider_href(url: &str)->Result<Vec<PageInfo>, MyError> {
    match fetch_page(url).await {
        Ok(html) => {
            let href = parse_page(&html);
            Ok(href)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }   
    }
}

async fn spider_src(url: &str)->Result<Vec<String>, MyError> {
    match fetch_page(url).await {
        Ok(html) => {
            let srcs = parse_picture(&html);
            Ok(srcs)
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }   
    }

}

async fn get_img_detail(url: &str)->Result<ImgDetail, MyError> {
    match fetch_img(url).await {
       Ok(bytes) => {
           match get_image_dimensions(&bytes){
            Ok(aspect_ratio) => {
                Ok(ImgDetail{
                    src:url.to_string(),
                    aspect_ratio,
                })
           }
           Err(e) => {
               println!("Error: {}", e);
               Err(MyError::StdError(e))
           }
        }
           
       }
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }   
    }
}


pub async fn concurrent_fetch(pages_info: Vec<PageInfo>)->Vec<Picture>{
    let mut results = vec![];
    let mut tasks = vec![];
    for info in pages_info {
        let task = tokio::spawn(async move {
            match spider_src(&info.href).await {
                Ok(srcs) => {
                  let img_details = concurrent_fetch_img(srcs).await;
                  Picture{
                    id:Uuid::new_v4().to_string(),
                    title:info.title,
                    url:info.href,
                    srcs:img_details,
                    star:3,
                    collect:false,
                    download:false,
                    deleted:false,
                  }

                }

                Err(e) => {
                    println!("Error: {}", e);
                    Picture{
                        id:Uuid::new_v4().to_string(),
                        title:info.title,
                        url:info.href,
                        srcs:vec![],
                        star:3,
                        collect:false,
                        download:false,
                        deleted:false,
                    }
                }
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        let result = task.await.unwrap();
        results.push(result);
    }

    results

}


async fn concurrent_fetch_img(srcs: Vec<String>)->Vec<ImgDetail>{
    let mut tasks = vec![];
    let mut results = vec![];
    for src in srcs {
        let task = tokio::spawn(async move {
            match get_img_detail(&src).await {
                Ok(img_detail) => {
                    println!("{:?}", img_detail);
                    
                    results.push(img_detail);
                }

                Err(e) => {
                    println!("Error: {}", e);
                    results.push(ImgDetail{
                        src:src.to_string(),
                        aspect_ratio:0.0,
                    })
                }
            }
        });
        tasks.push(task);
    }

    for task in tasks {
        let result = task.await.unwrap();
        results.push(result);
    }

    results
}
 







fn parse_page(html: &Html) -> Vec<PageInfo> {
    let selector = Selector::parse("tr:nth-of-type(n+8) h3 a").expect("Failed to parse selector");
    let mut pages_info = Vec::new();
    for element in html.select(&selector) {
        let title = element.inner_html();
        if let Some(href) = element.value().attr("href") {
            let page_info: PageInfo = PageInfo {
                title: title.to_string(),
                href: "https://dibjrh.xyz/pw/".to_string() + href,
            };
            pages_info.push(page_info);
        }
    }
    pages_info
}


fn parse_picture(html: &Html) -> Vec<String> {
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


fn get_image_dimensions(bytes: &Bytes) -> Result<f32,Box<dyn std::error::Error>> {
    let image = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let (width, height) = image.dimensions();
    Ok((width as f32) / (height as f32))
}