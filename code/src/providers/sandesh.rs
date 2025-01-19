use anyhow::Result;
use chrono::Local;
use dotenv::dotenv;
use fantoccini::{ClientBuilder, Locator};
use tracing::info;

use crate::common::capabilities;
pub struct Sandesh {
    cities: Vec<String>,
}
impl Sandesh {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            cities: vec!["rajkot".into(), "morbi".into()],
        }
    }
    pub fn get_filename(&self) -> String {
        let today = Local::now().format("%Y-%m-%d").to_string();
        format!("Sandesh_Rajkot_Morbi_{today}.pdf")
    }
    /// Fetch image URLs for multiple cities
    pub(crate) async fn fetch_urls(&self) -> Result<Vec<String>> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        let mut all_urls = vec![];

        for city in &self.cities {
            let urls = self.get_image_urls(city, &today).await?;
            all_urls.extend(urls);
        }

        Ok(all_urls)
    }
    async fn get_image_urls(&self, city: &str, date: &str) -> Result<Vec<String>> {
        let client = ClientBuilder::native()
            .capabilities(capabilities())
            .connect("http://localhost:4445")
            .await
            .expect("Failed to connect to WebDriver");
        info!("going to the page");
        client
            .goto(&format!("https://sandesh.com/epaper/{city}?date={date}"))
            .await?;
        info!("gone to the page");
        client
            .wait()
            .for_element(Locator::Css("div.carousel-item"))
            .await?;
        info!("done waiting for element.");

        let elements = client.find_all(Locator::Css("div.carousel-item")).await?;
        let mut urls = vec![];

        for element in elements.iter() {
            if let Some(src) = element.find(Locator::Css("img")).await?.attr("src").await? {
                urls.push(src);
            }
        }
        client.close().await?;

        Ok(urls)
    }
}
