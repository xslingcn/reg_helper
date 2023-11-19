use thiserror::Error;

pub type RegResult<T> = Result<T, RegError>;

#[derive(Debug, Error)]
pub enum RegError {
    #[error("fail to load config: {0}")]
    ConfigLoadError(String),
    #[error("cookie not found for: {0}")]
    CookieNotFound(String),
    #[error("schedule not updated: {0}")]
    RegFailedError(String),
    #[error("element not found: {0}")]
    ElementNotFound(String),
    #[error("webdriver error: {0}")]
    WebDriverError(#[from] thirtyfour::error::WebDriverError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("imap error: {0}")]
    IMAPError(#[from] imap::error::Error),
    #[error("native_tls error: {0}")]
    TLSError(#[from] native_tls::Error),
}

pub(crate) trait PassCookieNotFound {
    fn pass_cookie_not_found(self) -> RegResult<String>; 
}

impl PassCookieNotFound for RegResult<String> {
    fn pass_cookie_not_found(self) -> RegResult<String> {
        match self {
            Err(RegError::CookieNotFound(_)) => Ok("Cookie not found".to_string()),
            _ => self,
        }
    }
}