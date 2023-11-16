use crate::config::CONFIG;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

pub fn register(sln: &str) -> Result<String, reqwest::Error> {
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

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url).headers(headers).send()?;
    let body = response.text()?;

    println!("Response Body: {:?}", &body);

    if body.contains("Schedule updated.") {
        Ok("Schedule updated.".to_string())
    } else {
        Ok("Schedule not updated.".to_string())
    }
}
