use std::collections::HashMap;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

pub struct RequestParam<'a> {
    url: Option<String>,
    headers: Option<HeaderMap>,
    body: Option<String>,
    form: Option<HashMap<&'a str, &'a str>>,
}
impl <'a> RequestParam<'a> {
    pub fn new() -> RequestParam<'a> {
        RequestParam {
            url: None,
            headers: None,
            body: None,
            form: None,
        }
    }
    pub fn url(&mut self, url: &str) {
        self.url = Some(url.to_string());
    }
    pub fn headers(&mut self, headers: HeaderMap) {
        self.headers = Some(headers);
    }
    pub fn body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }
    pub fn form(&mut self, form: HashMap<&'a str, &'a str>) {
        self.form = Some(form);
    }
}

/// 发送(同步)get请求
pub fn get_blocking(url: &str, query_param: Option<&[&str]>) -> Option<String>{
    let mut target_url = String::new();
    target_url.push_str(url);
    // merge query params to url
    if let Some(query_params) = query_param {
        for param in query_params {
            target_url.push_str(&format!("{}{}", "&", param));
        }
    }

    let client = reqwest::blocking::Client::new();
    if let Ok(response) = client.get(url).send() {
        if let Ok(text) = response.text() {
            return Some(text);
        }
    }
    None
}

/// 发送(同步)post请求
pub fn post_blocking(param: RequestParam) -> Response {
    let RequestParam {
        url,
        headers,
        body,
        form,
    } = param;

    if url.is_none() {
        panic!("url must not be empty!");
    }

    let client = reqwest::blocking::Client::new();
    let mut builder = client.post(url.unwrap());

    if let Some(headers) = headers {
        builder = builder.headers(headers);
    }

    if let Some(body) = body {
        builder = builder.body(body);
    }

    if let Some(form) = form {
        builder = builder.form(&form);
    }

    builder.send().unwrap()
}

pub async fn post_async(param: RequestParam<'_>) -> reqwest::Response {
    let RequestParam {
        url,
        headers,
        body,
        form,
    } = param;

    if url.is_none() {
        panic!("url must not be empty!");
    }

    let client = reqwest::Client::new();
    let mut builder = client.post(url.unwrap());

    if let Some(headers) = headers {
        builder = builder.headers(headers);
    }

    if let Some(body) = body {
        builder = builder.body(body);
    }

    if let Some(form) = form {
        builder = builder.form(&form);
    }

    builder.send().await.unwrap()
}