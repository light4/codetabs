use async_std::prelude::*;
use serde::Deserialize;

const API_URL: &'static str = "https://api.codetabs.com/v1/loc?github=";

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
struct LocItem {
    language: String,
    files: String,
    lines: String,
    blanks: String,
    comments: String,
    #[serde(rename = "linesOfCode")]
    lines_of_code: String,
}

type Loc = Vec<LocItem>;

async fn get_statistics(
    repo: &str,
) -> Result<Loc, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = format!("{}{}", API_URL, repo);
    let data: Loc = surf::get(uri).recv_json().await?;
    Ok(data)
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    if let Some(repo) = std::env::args().nth(1) {
        let data = get_statistics(&repo).await?;
        for i in data {
            println!("{:#?}", i);
        }
    }
    Ok(())
}
