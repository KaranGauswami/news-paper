use anyhow::Result;
mod common;
use common::{create_pdf_from_urls, send_to_telegram_chats};
mod providers;
use dotenv::dotenv;
use providers::{GujaratSamachar, Sandesh};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    //env_logger::init();
    tracing_subscriber::fmt::init();

    let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN must be set in .env file");
    let chat_ids: Vec<String> = env::var("CHAT_IDS")
        .expect("CHAT_IDS must be set in .env file")
        .split(',')
        .map(|id| id.trim().to_string())
        .collect();

    let gujarat_samachar_task = async {
        let gujarat_samachar = GujaratSamachar::new();
        let all_urls = gujarat_samachar.fetch_urls().await?;
        let file_name = gujarat_samachar.get_filename();
        create_pdf_from_urls(&all_urls, &file_name).await?;
        info!("PDF generation completed: {}", file_name);
        send_to_telegram_chats(&chat_ids, &bot_token, &file_name).await?;
        info!("Sending file complete {:?}", &file_name);
        Ok::<_, anyhow::Error>(())
    };

    let sandesh_task = async {
        let sandesh = Sandesh::new();
        let all_urls = sandesh.fetch_urls().await?;
        let file_name = sandesh.get_filename();
        create_pdf_from_urls(&all_urls, &file_name).await?;
        info!("PDF generation completed: {}", file_name);
        send_to_telegram_chats(&chat_ids, &bot_token, &file_name).await?;
        info!("Sending file complete {:?}", &file_name);
        Ok::<_, anyhow::Error>(())
    };

    let (gujarat_result, sandesh_result) = tokio::join!(gujarat_samachar_task, sandesh_task);

    // Handle results
    if let Err(e) = gujarat_result {
        eprintln!("Error in Gujarat Samachar block: {:?}", e);
    }

    if let Err(e) = sandesh_result {
        eprintln!("Error in Sandesh block: {:?}", e);
    }

    Ok(())
}
