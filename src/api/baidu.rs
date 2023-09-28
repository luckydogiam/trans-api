use std::collections::HashMap;

use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::api::rest::*;

#[derive(Serialize, Deserialize, Debug)]
struct BaiduResponse {
    from: Option<String>,
    to: Option<String>,
    trans_result: Option<Vec<BaiduTransResult>>,
    error_code: Option<i16>
}
#[derive(Serialize, Deserialize, Debug)]
struct BaiduTransResult {
    src: Option<String>,
    dst: Option<String>,
}

///
/// 调用百度通用翻译API，需要先注册开通(免费，每月限额度)
/// 注册开通后，可以得到appid，密钥
///
pub fn translate(input_text: &str) -> Option<String> {
    let appid = "#replace with your appid#";
    let salt = "1233211234567";
    let private_key = "#replace with your private_key#";

    let url = "https://fanyi-api.baidu.com/api/trans/vip/translate";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

    let sign = format!("{}{}{}{}", appid, input_text, salt, private_key);
    let mut hasher = Md5::new();
    hasher.input_str(&sign);
    let sign = hasher.result_str();

    let mut form = HashMap::new();
    form.insert("q", input_text);
    form.insert("from", "auto");
    form.insert("to", "zh");
    form.insert("appid", appid);
    form.insert("salt", salt);
    form.insert("sign", &sign);

    let mut param = RequestParam::new();
    param.url(url);
    param.headers(headers);
    param.form(form);
    let response = post_blocking(param);
    let result = response.text().unwrap();

    let baidu_response: BaiduResponse = serde_json::from_str(result.as_str()).unwrap();
    if let Some(baidu_trans_results) = baidu_response.trans_result {
        let result = baidu_trans_results.iter()
            .map(|result| result.dst.clone().unwrap())
            .reduce(|acc, result| format!("{}, {}", acc, result));
        return Some(result.unwrap());
    };

    None
}