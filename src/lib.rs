use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ImgDetail {
    pub src: String,
    pub aspect_ratio: f32,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Picture {
    pub id: String,
    pub title: String,
    pub url: String,
    pub srcs: Vec<ImgDetail>,
    pub star: u8,
    pub collect: bool,
    pub download: bool,
    pub deleted: bool,
}

#[derive(Debug,Clone, Serialize)]
pub struct PageInfo {
   pub title: String,
   pub href: String,
}