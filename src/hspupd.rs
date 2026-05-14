use encoding_rs::SHIFT_JIS;

use crate::hs;

const HSPURL: &str = "https://www.onionsoft.net/hsp/update/";
const HSPINDEX: &str = "win32/index.hspupd";

pub async fn get_hsp_official_index() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", HSPURL, HSPINDEX);
    let bytes = reqwest::get(url).await?.bytes().await?;
    let (text, _, _) = SHIFT_JIS.decode(&bytes);
    Ok(text.into_owned())
}

pub async fn get_hsp_official_platform() -> Result<hs::Platform, Box<dyn std::error::Error>> {
    Ok(hs::parse_hsp_index(get_hsp_official_index().await?)) // パース結果を返す
}
