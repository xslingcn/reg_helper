use crate::config::CONFIG;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};

pub async fn register(sln: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://sdb.admin.uw.edu/students/UWNetID/register.asp?INPUTFORM=UPDATE&PAC=0&MAXDROPS=0&_CW={}&QTR={}&YR={}&sln1={}&entCode1=&credits1=&gr_sys1=",
        CONFIG.reg.cw, CONFIG.reg.quarter, CONFIG.reg.year, sln
    );

    let cookie = format!("{}={}", CONFIG.reg.shibsession_name, CONFIG.reg.shibsession_content);
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;

    println!("Response Status: {}", response.status());
    Ok(())
}