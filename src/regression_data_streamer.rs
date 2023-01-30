use std::cmp::min;

use futures_util::StreamExt;
use reqwest::Client;

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub async fn download_file(client: &Client, url: &str) -> Result<String, String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let mut owned_data: String = "".to_owned();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        owned_data.push_str(std::str::from_utf8(&chunk).unwrap());
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
    }

    return Ok(owned_data);
}
