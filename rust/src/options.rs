use std::time::Duration;

use reqwest::RequestBuilder;

const URL_PREFIX: &str = "https://business-api.tiktok.com/open_api/v1.3";
const ENV_KEY: &str = "TICTOK_BUSINESS_PREFIX_API";

#[derive(Debug, Clone, Default)]
pub struct TiktokOptions {
    pub prefix_url: Option<String>,
    pub timeout_duration: Option<Duration>,
    pub try_count: Option<u8>,
    pub retry_duration: Option<Duration>,
}

pub fn clear_prefix_url() {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, URL_PREFIX) };
}

pub fn setup_prefix_url(url: &str) {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { std::env::set_var(ENV_KEY, url) };
}

pub(crate) fn make_url(postfix_url: &str, options: &Option<TiktokOptions>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(URL_PREFIX.to_owned()),
        options,
        postfix_url,
    )
}

fn make_url_with_prefix(
    default_perfix_url: &str,
    options: &Option<TiktokOptions>,
    postfix_url: &str,
) -> String {
    let prefix_url = if let Some(options) = options {
        if let Some(prefix_url) = options.prefix_url.as_ref() {
            prefix_url
        } else {
            default_perfix_url
        }
    } else {
        default_perfix_url
    };
    format!("{}{}", prefix_url, postfix_url)
}

pub(crate) fn apply_timeout(
    builder: RequestBuilder,
    options: &Option<TiktokOptions>,
) -> RequestBuilder {
    if let Some(options) = options {
        if let Some(timeout) = options.timeout_duration {
            builder.timeout(timeout)
        } else {
            builder
        }
    } else {
        builder
    }
}
