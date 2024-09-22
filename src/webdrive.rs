use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

use crate::config::CONFIG;
use crate::cookie;
use crate::error::{PassCookieNotFound, RegError, RegResult};

async fn create_session() -> RegResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let server_url = format!("http://localhost:{}", CONFIG.webdrive.port);
    let driver = WebDriver::new(&server_url, caps).await?;
    Ok(driver)
}

pub(crate) async fn saml_login() -> RegResult<String> {
    let driver = create_session().await?;

    add_cookie(&driver, "idp.u.washington.edu".to_string())
        .await
        .pass_cookie_not_found()?;

    driver
        .goto("https://sdb.admin.uw.edu/students/uwnetid/register.asp")
        .await?;
    let netid = driver.find(By::Id("weblogin_netid")).await?;
    netid.send_keys(&CONFIG.reg.netid).await?;

    let passwd = driver.find(By::Id("weblogin_password")).await?;
    passwd.send_keys(&CONFIG.reg.password).await?;
    sleep(Duration::from_millis(500)).await;

    let submit_button = driver.find(By::Id("submit_button")).await?;
    submit_button.click().await?;

    loop {
        let current_url = driver.current_url().await?;
        if current_url.as_str() == "https://sdb.admin.uw.edu/students/uwnetid/register.asp" {
            break;
        }
        sleep(Duration::from_secs(2)).await;
    }

    let cookies = driver.get_all_cookies().await?;
    cookie::update_cookies("sdb.admin.uw.edu".to_string(), cookies).await;

    driver.goto("https://idp.u.washington.edu/404").await?;
    let cookies = driver.get_all_cookies().await?;
    cookie::update_cookies("idp.u.washington.edu".to_string(), cookies).await;
    driver.quit().await?;

    Ok("Logged in!".to_string())
}

pub(crate) async fn saml_refresh() -> RegResult<String> {
    let driver = create_session().await?;

    add_cookie(&driver, "sdb.admin.uw.edu".to_string())
        .await
        .pass_cookie_not_found()?;
    add_cookie(&driver, "idp.u.washington.edu".to_string())
        .await
        .pass_cookie_not_found()?;

    driver
        .goto("https://sdb.admin.uw.edu/students/uwnetid/register.asp")
        .await?;

    loop {
        if let Ok(_) = driver.find(By::Id("weblogin_netid")).await {
            driver.quit().await?;
            return saml_login().await;
        }

        let current_url = driver.current_url().await?;
        if current_url.as_str() == "https://sdb.admin.uw.edu/students/uwnetid/register.asp" {
            break;
        }
        sleep(Duration::from_secs(2)).await;
    }

    let cookies = driver.get_all_cookies().await?;
    cookie::update_cookies("sdb.admin.uw.edu".to_string(), cookies).await;
    driver.quit().await?;

    Ok("SDB session refreshed!".to_string())
}


pub(crate) async fn create_switch_webdriver() -> RegResult<WebDriver> {
    let driver = create_session().await?;

    add_cookie(&driver, "sdb.admin.uw.edu".to_string()).await?;

    driver.goto("https://sdb.admin.uw.edu/students/UWNetID/register.asp").await?;
    Ok(driver)
}

async fn add_cookie(driver: &WebDriver, domain: String) -> RegResult<String> {
    if let Some(cookies) = cookie::get_cookies(domain.clone()).await {
        let url = format!("https://{}/404", domain);
        println!("Adding cookies for {}", &url);
        driver.goto(&url).await?;
        for cookie in cookies {
            driver.add_cookie(cookie.clone()).await?;
        }
        Ok("Cookies added!".to_string())
    } else {
        Err(RegError::CookieNotFound(domain))?
    }
}