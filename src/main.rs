mod database;
use std::time::Instant;
use tokio;
use log::Level;
mod async_spider;
use spider::PageInfo;


#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    simple_logger::init_with_level(Level::Info).unwrap();
        let hrefs = vec![
        PageInfo{
            title:"美腿女神楊晨晨白皙嬌體羞澀裸露 (58P) ".to_string(),
            href:"https://dibjrh.xyz/pw/html_data/14/2404/7304851.html".to_string()},
       PageInfo{
            title:"美腿女神楊晨晨白皙嬌體羞澀裸露 (58P) ".to_string(),
            href:"https://dibjrh.xyz/pw/html_data/14/2404/7304847.html".to_string()},

       PageInfo{
            title:"[小鳥遊るい, 近藤沙瑛子] 白嫩完美身材的美少女組合 (27P)".to_string(),
            href:"https://dibjrh.xyz/pw/html_data/14/2404/7304855.html".to_string()},
        PageInfo{
            title:"韓模金美靜（김미경）大尺度人體私拍套圖-04 (117P)".to_string(),
            href:"https://dibjrh.xyz/pw/html_data/14/2404/7304825.html".to_string()},
        ];

    // let url = "https://dibjrh.xyz/pw/thread6.php?fid=14";

    // let mut hrefs = Vec::new();
    // match async_spider::spider_href(url).await {
    //     Ok(info) => {
    //         // print!("{:#?}",hrefs)
    //         hrefs = info;
    //     }
    //     Err(e) => {
    //         println!("spider_href err {}", e);
    //     }
    // }

    let pictures = async_spider::concurrent_fetch(hrefs).await;

    print!("{:#?}", pictures);

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("程序运行时间: {} 秒", duration.as_secs());
}

