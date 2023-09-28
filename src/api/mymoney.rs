use serde::{Deserialize, Serialize};

use crate::api::rest::get_blocking;

const MY_MONEY_TRANSLATE_ADDRESS: &'static str = "https://api.mymemory.translated.net/?q=${input}&langpair=${source_lang}|${target_lang}";

#[derive(Serialize, Deserialize, Debug)]
struct MyMoneyResponse {
    #[serde(rename = "responseData")]
    response_data: Option<MyMoneyResponseData>,
    matches: Option<Vec<MyMoneyMatchResult>>,
}
#[derive(Serialize, Deserialize, Debug)]
struct MyMoneyResponseData {
    #[serde(rename = "translatedText")]
    translated_text: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct MyMoneyMatchResult {
    segment: Option<String>,
    translation: Option<String>,
    source: Option<String>,
    target: Option<String>,
    match_rate: Option<f32>,
}

///
/// 调用my money api，免费
///
pub fn translate(input: &str) -> Option<String> {
    let url = MY_MONEY_TRANSLATE_ADDRESS
        .replace("${input}", input)
        .replace("${source_lang}", "en")
        .replace("${target_lang}", "zh");
    if let Some(text) = get_blocking(&url, None) {
        let my_money_response: MyMoneyResponse = serde_json::from_str(&text).unwrap();
        return Some(my_money_response.response_data.unwrap().translated_text.unwrap());
    }
    None
}