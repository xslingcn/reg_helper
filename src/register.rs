use crate::config;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, SET_COOKIE};

pub fn register(sln: &str) -> Result<String, reqwest::Error> {
    println!("Registering for SLN: {}", sln);
    let conf = config::get_config();
    let url = format!(
        "https://sdb.admin.uw.edu/students/UWNetID/register.asp?INPUTFORM=UPDATE&PAC=0&MAXDROPS=0&_CW={}&QTR={}&YR={}&sln1={}&entCode1=&credits1=&gr_sys1=",
        conf.reg.cw, conf.reg.quarter, conf.reg.year, sln
    );
    println!("Requesting URL: {}", &url);

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&conf.reg.cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(headers).send()?;

    println!("Response Status: {}", response.status());
    println!("Response Body: {:?}", response.text()?);
    Ok("Request Complete!".to_string())
}

pub fn update_cookie() -> Result<String, Box<dyn std::error::Error>>{
    println!("Updating cookie...");
    let conf = config::get_config();

    let url = "https://sdb.admin.uw.edu/students/uwnetid/register.asp";
    println!("Requesting URL: {}", &url);
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&conf.reg.cookie).unwrap());
    println!("Headers: {:?}", headers);

    let client = reqwest::blocking::Client::new();
    let response = client.get(url).headers(headers).send()?;    

    if let Some(cookie) = response.headers().get(SET_COOKIE) {
        let cookie_str = cookie.to_str()?;
        println!("Set-Cookie: {}", cookie_str);
        config::update_global_cookie(cookie_str.to_string());
    } else {
        return Err("No Set-Cookie header found in response")?;
    }

    Ok(format!("Cookie updated to: {}", conf.reg.cookie))
}