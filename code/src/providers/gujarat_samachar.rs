use anyhow::Result;
use chrono::Local;
use fantoccini::{ClientBuilder, Locator};
use tracing::info;

use crate::common::capabilities;

pub struct GujaratSamachar {
    cities: Vec<String>,
}
impl GujaratSamachar {
    pub fn new() -> Self {
        GujaratSamachar {
            cities: vec!["rajkot-saurashtra".into()],
        }
    }
    pub fn get_filename(&self) -> String {
        let today = Local::now().format("%Y-%m-%d").to_string();
        format!("GujaratSamchar_Rajkot_{today}.pdf")
    }
    async fn get_image_urls(&self, city: &str, date: &str) -> Result<Vec<String>> {
        let client = ClientBuilder::native()
            .capabilities(capabilities())
            .connect("http://localhost:4444")
            .await
            .expect("Failed to connect to WebDriver");
        info!("going to the page");
        client
            .goto(&format!(
                "https://epaper.gujaratsamachar.com/{city}/{date}/1"
            ))
            .await?;
        info!("gone to the page");
        client
            .wait()
            .for_element(Locator::Css(".mb-4.thumb_image.thumb_list"))
            .await?;
        info!("done waiting for element.");

        let elements = client
            .find_all(Locator::Css(".mb-4.thumb_image.thumb_list"))
            .await?;
        let mut urls = vec![];

        for element in elements.iter() {
            if let Some(src) = element.find(Locator::Css("img")).await?.attr("src").await? {
                let src = src.replace("/thumbnail", "");
                println!("{:?}", src);
                urls.push(src);
            }
        }
        client.close().await?;

        Ok(urls)
    }
    pub async fn fetch_urls(&self) -> Result<Vec<String>> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let mut all_urls = vec![];

        for city in &self.cities {
            let urls = self.get_image_urls(city, &today).await?;
            all_urls.extend(urls);
        }

        Ok(all_urls)
    }
}
