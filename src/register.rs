use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use scraper::{Html, Selector};

use crate::cookie;
use crate::error::{RegError, RegResult};
use crate::{config::CONFIG, webdrive};

pub(crate) async fn register(sln: &str) -> RegResult<String> {
    println!("Registering for SLN: {}", sln);
    let url = format!(
        "https://sdb.admin.uw.edu/students/UWNetID/register.asp?INPUTFORM=UPDATE&PAC=0&MAXDROPS=0&_CW={}&QTR={}&YR={}&sln1={}&entCode1=&credits1=&gr_sys1=",
        CONFIG.reg.cw, CONFIG.reg.quarter, CONFIG.reg.year, sln
    );
    println!("Requesting URL: {}", url);

    let cookie = cookie::get_cookie_str("sdb.admin.uw.edu".to_string()).await?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?;
    let body = response.text().await?;

    if body.contains("Schedule updated.") {
        Ok("Schedule updated.".to_string())
    } else {
        let reason = parse_status(&body)?;
        Err(RegError::RegFailedError(reason))?
    }
}

fn parse_status(res: &str) -> RegResult<String> {
    let document = Html::parse_document(res);
    
    for index in 1..11 {
        let selector_str = format!("input[name=\"dup{}\"] + td", index);
        let selector = Selector::parse(&selector_str).unwrap();
        let next_selector_str = format!("input[name=\"dup{}\"] + td", index + 1);
        let next_selector = Selector::parse(&next_selector_str).unwrap();
        println!("Selector: {}", selector_str);

        if let Some(next_td_element) = document.select(&next_selector).next() {
            if next_td_element
                .select(&Selector::parse("b").unwrap())
                .next()
                .is_none()
            {
                if let Some(td_element) = document.select(&selector).next() {
                    return Ok(td_element.text().collect::<Vec<_>>().join(" "))
                }
            }
        }
    }
    Err(RegError::ElementNotFound("status text".to_string()))
}

pub(crate) async fn refresh_shib_session() -> RegResult<String> {
    println!("Refreshing Shibboleth session...");
    let url = "https://sdb.admin.uw.edu/students/UWNetID/register.asp";
    println!("Requesting URL: {}", url);

    let cookie = cookie::get_cookie_str("sdb.admin.uw.edu".to_string()).await?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await?;
    let body = response.text().await?;

    if body.contains("Stale Request") {
        return webdrive::saml_refresh().await;
    } else {
        Ok("shib_session kept alive ".to_string())
    }
}