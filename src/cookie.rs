use lazy_static::lazy_static;
use std::{sync::Arc, collections::HashMap};
use thirtyfour::Cookie;
use tokio::sync::Mutex;

use crate::error::{RegResult, RegError};

lazy_static! {
    static ref GLOBAL_COOKIE: Arc<Mutex<HashMap<String, Vec<Cookie>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub(crate) async fn update_cookies(domain: String, new_cookies: Vec<Cookie>) {
    let mut cookie = GLOBAL_COOKIE.lock().await;
    cookie.insert(domain, new_cookies);
}

pub(crate) async fn get_cookies(domain: String) -> Option<Vec<Cookie>> {
    let cookie = GLOBAL_COOKIE.lock().await;
    return cookie.get(&domain).cloned();
}

// pub async fn delete_cookies(domain: String) {
//     let mut cookie = GLOBAL_COOKIE.lock().await;
//     cookie.remove(&domain);
// }

// pub async fn delete_all_cookies() {
//     let mut cookie = GLOBAL_COOKIE.lock().await;
//     cookie.clear();
// }

pub(crate) async fn get_cookie_str(domain: String) -> RegResult<String> {
    if let Some(cookies) = get_cookies(domain.clone()).await {
        return Ok(cookies_to_str(&cookies));
    } else {
        Err(RegError::CookieNotFound(domain))?
    }
}

fn cookies_to_str(cookies: &Vec<Cookie>) -> String {
    return cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<String>>()
        .join("; ");
}