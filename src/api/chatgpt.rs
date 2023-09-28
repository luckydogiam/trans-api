use std::collections::HashMap;

use reqwest::header::{CONTENT_TYPE, HeaderMap, ORIGIN, REFERER, USER_AGENT};

use crate::api::rest::{post_async, post_blocking, RequestParam};

const CHAT_GPT_URL: &'static str  = "https://api.aichatos.cloud/api/generateStream";

fn build_input(input: &str) -> RequestParam {
    let mut param = RequestParam::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ORIGIN, "https://chat1.yqcloud.top".parse().unwrap());
    headers.insert(REFERER, "https://chat1.yqcloud.top".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36".parse().unwrap());

    let trans_prompt = format!("翻译以下内容为中文: {}", input);
    let mut json_map = HashMap::new();
    json_map.insert("prompt", trans_prompt.as_str());
    json_map.insert("network", "true");
    json_map.insert("stream", "false");
    json_map.insert("system", "");
    json_map.insert("userId", "");
    json_map.insert("withoutContext", "true");

    param.url(CHAT_GPT_URL);
    param.body(&serde_json::to_string::<HashMap<&str, &str>>(&json_map).unwrap());
    param.headers(headers);
    param
}

pub fn translate(input: &str) -> Option<String> {
    let param = build_input(input);
    let mut response = post_blocking(param);
    if let Ok(text) = response.text() {
        return Some(text);
    }
    None
}

pub async fn translate_async(input: &str) -> reqwest::Response {
    let param = build_input(input);
    post_async(param).await
}