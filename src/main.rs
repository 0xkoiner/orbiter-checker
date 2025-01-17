use reqwest::{Client, header::{HeaderMap, HeaderName, HeaderValue}};
use serde_json::Value;
use std::fs;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let url = "https://api.orbiter.finance/sdk/opoints/user/";

    let mut headers = HeaderMap::new();
    let raw_headers = [
        ("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"),
        ("accept-language", "en,en-US;q=0.9,ru;q=0.8"),
        ("cache-control", "max-age=0"),
        ("priority", "u=0, i"),
        ("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#),
        ("sec-ch-ua-mobile", "?0"),
        ("sec-ch-ua-platform", "\"macOS\""),
        ("sec-fetch-dest", "document"),
        ("sec-fetch-mode", "navigate"),
        ("sec-fetch-site", "none"),
        ("sec-fetch-user", "?1"),
        ("upgrade-insecure-requests", "1"),
    ];

    for (key, value) in &raw_headers {
        let header_name = HeaderName::from_bytes(key.as_bytes()).unwrap();
        let header_value = HeaderValue::from_str(value).unwrap();
        headers.insert(header_name, header_value);
    }

    let wallets_content = fs::read_to_string("src/wallets.txt")
        .expect("Failed to read wallets.txt");

        let wallets = wallets_content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let total_tokens = Arc::new(Mutex::new(0.0));

    let mut tasks = vec![];

    for wallet in wallets.iter() {
        if wallet.len() != 42 || !wallet.starts_with("0x") {
            println!("Invalid wallet address: {} (expected 42 characters and '0x' prefix)", wallet);
            continue;
        }

        let total_tokens = Arc::clone(&total_tokens);
        let cloned_headers = headers.clone();
        let cloned_wallet = wallet.to_string();
        
        let task = tokio::spawn(async move {
            if let Err(e) = handle_wallet_request(url, &cloned_headers, &cloned_wallet, total_tokens).await {
                println!("Error processing wallet {}: {:?}", cloned_wallet, e);
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let total_tokens_value = total_tokens.lock().await;
    println!("----------------------------------------------------------------------------------------------");
    println!("[*]Total Tokens: {:.2}", *total_tokens_value);

    let potential_usd_05 = *total_tokens_value * 0.05;  
    let potential_usd_03 = *total_tokens_value * 0.03;  
    
    println!("[*]Potential in USD if 0.05$ per token: {:.2}$", potential_usd_05);
    println!("[*]Potential in USD if 0.03$ per token: {:.2}$", potential_usd_03);
}

async fn handle_wallet_request(
    url: &str,
    headers: &HeaderMap,
    wallet: &str,
    total_tokens: Arc<Mutex<f64>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get(format!("{}/{}", url, wallet))
        .headers(headers.clone()) 
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        if let Some(points) = json.pointer("/result/points").and_then(|p| p.as_f64()) {
            if points < 200.0 {
                println!("{} is not eligible (points: {})", wallet, points);
            } else {
                let value = points * 5.5970149254;
                println!("----------------------------------------------------------------------------------------------");
                println!(
                    "[*]{} is eligible (points: {}, value: {:.2})",
                    wallet, points, value
                );
                println!("----------------------------------------------------------------------------------------------");
                let mut total_tokens = total_tokens.lock().await;
                *total_tokens += value;
            }
        } else {
            println!("{}: Unable to fetch points from the response.", wallet);
        }
    } else {
        println!(
            "Failed to fetch data for wallet {}. HTTP Status: {}",
            wallet,
            response.status()
        );
    }

    Ok(())
}
