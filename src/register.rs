use crate::config::CONFIG;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::error::Error;

pub async fn register(sln: &str) -> Result<String, Box<dyn Error>> {
    println!("Registering for SLN: {}", sln);
    let url = format!(
        "https://sdb.admin.uw.edu/students/UWNetID/register.asp?INPUTFORM=UPDATE&PAC=0&MAXDROPS=0&_CW={}&QTR={}&YR={}&sln1={}&entCode1=&credits1=&gr_sys1=",
        CONFIG.reg.cw, CONFIG.reg.quarter, CONFIG.reg.year, sln
    );
    println!("Requesting URL: {}", url);

    let cookie = format!(
        "{}={}",
        CONFIG.reg.shibsession_name, CONFIG.reg.shibsession_content
    );
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;
    let body = response.text().await?;

    println!("Response Body: {:?}", &body);

    if body.contains("Schedule updated.") {
        Ok("Schedule updated.".to_string())
    } else {
        Err("Schedule not updated.".to_string())?
    }
}
pub async fn refresh_shib_session() -> Result<String, Box<dyn Error>> {
    println!("Refreshing Shibboleth session...");
    let url = "https://sdb.admin.uw.edu/students/UWNetID/register.asp";
    println!("Requesting URL: {}", url);

    let cookie = format!(
        "{}={}",
        CONFIG.reg.shibsession_name, CONFIG.reg.shibsession_content
    );
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await?;
    let body = response.text().await?;

    println!("Response Body: {:?}", &body);

    if body.contains("Stale Request") {
        Err("Stale Request. Please update cookie manually")?
    } else {
        Ok("shib_session kept alive ".to_string())
    }
}
