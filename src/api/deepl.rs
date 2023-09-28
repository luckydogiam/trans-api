//! not stable for long sentence

use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::api::rest::*;

#[derive(Serialize, Deserialize, Debug)]
struct DeepLResponse {
    result: DeepLResult,
}
#[derive(Serialize, Deserialize, Debug)]
struct DeepLResult {
    source_lang: String,
    source_lang_is_confident: bool,
    target_lang: String,
    translations: Vec<DeepLBeam>
}
#[derive(Serialize, Deserialize, Debug)]
struct DeepLBeam {
    beams: Vec<DeepLSentenceWrap>,
    quality: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct DeepLSentenceWrap {
    sentences: Vec<DeepLSentence>
}
#[derive(Serialize, Deserialize, Debug)]
struct DeepLSentence {
    text: String,
}

pub fn translate(input_text: &str) -> Option<String> {
    // request params
    let url = "https://www2.deepl.com/jsonrpc?method=LMT_handle_jobs";
    let body_json = r#"{"jsonrpc":"2.0","method" : "LMT_handle_jobs","params":{"jobs":[{"kind":"default","sentences":[{"text":"${input_text}","id":1,"prefix":""}],"raw_en_context_before":[],"raw_en_context_after":[],"preferred_num_beams":4}],"lang":{"target_lang":"ZH","preference":{"weight":{},"default":"default"},"source_lang_computed":"AUTO"},"priority":1,"commonJobParams":{"mode":"translate","textType":"plaintext","browserType":1},"timestamp":1695096132634},"id":31340005}"#;
    let body_json = body_json.replace("${input_text}", &input_text);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    let mut param = RequestParam::new();
    param.url(url);
    param.body(&body_json);
    param.headers(headers);

    // send request
    let response = post_blocking(param);
    let string = response.text().unwrap();
    //println!("{:?}", string);

    // handle response
    let Ok(deepl_response) = serde_json::from_str::<DeepLResponse>(string.as_str()) else {
        println!("翻译失败!");
        return None;
    };
    
    let mut result = String::new();
    for beam in &deepl_response.result.translations.get(0).unwrap().beams {
        let text = &beam.sentences.get(0).unwrap().text;
        //println!("{}", text);
        result.push_str(text);
        result.push_str(", ");
    }
    let len = result.len();
    result.drain(len-2..len);

    Some(result)
}