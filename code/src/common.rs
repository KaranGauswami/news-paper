use anyhow::Result;
use futures::future::join_all;
use jpeg_to_pdf::JpegToPdf;
use reqwest::Client;
use std::{fs::File, io::BufWriter};
use url::Url;

/// Download images and create a PDF
pub async fn create_pdf_from_urls(urls: &[String], output_file_path: &str) -> Result<()> {
    let jpeg = JpegToPdf::new();
    let client = Client::new();

    let images: Vec<_> = join_all(urls.iter().map(|url| async {
        let mut parsed_url = Url::parse(url)?;
        parsed_url.set_query(None);

        let response = client.get(parsed_url).send().await?;
        let bytes = response.bytes().await?;
        Ok::<_, anyhow::Error>(bytes.to_vec())
    }))
    .await
    .into_iter()
    .collect::<Result<_>>()?;

    let out_file = File::create(output_file_path)?;
    jpeg.add_images(images)
        .create_pdf(&mut BufWriter::new(out_file))?;

    Ok(())
}

/// Send the generated PDF to multiple Telegram chats
pub(crate) async fn send_to_telegram_chats(
    chat_ids: &[String],
    token: &str,
    file_path: &str,
) -> Result<()> {
    let client = Client::new();

    for chat_id in chat_ids {
        let form = reqwest::multipart::Form::new()
            .text("chat_id", chat_id.clone())
            .file("document", file_path)
            .await?;

        let response = client
            .post(format!("https://api.telegram.org/bot{token}/sendDocument"))
            .multipart(form)
            .send()
            .await?;

        tracing::info!("Sent to {}: {:?}", chat_id, response.status());
    }

    Ok(())
}

pub(crate) fn capabilities() -> serde_json::map::Map<String, serde_json::Value> {
    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({ "args": ["--headless","--disable-gpu"] });
    caps.insert("moz:firefoxOptions".to_string(), opts);
    caps
}
