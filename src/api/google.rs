use serde_json::Value;

use crate::api::rest::*;

const GOOGLE_TRANSLATE_ADDRESS: &'static str = "http://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=${target_lang}&dt=t&q=${input}";

///
/// 调用谷歌(免费)API来获取简单的翻译结果
///
pub fn translate(input: &str) -> Option<String> {
    translate_to(input, "zh")
}

pub fn translate_to(input_text: &str, target_lang: &str) -> Option<String> {
    let target_lang = if target_lang.is_empty() { "zh" } else { target_lang };
    let url = GOOGLE_TRANSLATE_ADDRESS.replace("${target_lang}", target_lang).replace("${input}", input_text);

    let result_json = get_blocking(&url, None);
    let text = result_json.unwrap();
    let Ok(json) = serde_json::from_str::<Value>(&text) else {
        println!("翻译失败！");
        return None;
    };
    let Some(json_arr1) = json.as_array() else {
        println!("翻译失败！");
        return None;
    };
    let Some(json_arr_data_1) = json_arr1.get(0) else {
        println!("翻译失败！");
        return None;
    };
    let Some(json_arr_2) = json_arr_data_1.as_array() else {
        println!("翻译失败！");
        return None;
    };

    let mut result = String::new();
    for json in json_arr_2 {
        if let Some(text) = json.get(0) {
            if let Some(text) = text.as_str() {
                result.push_str(text);
            }
        }
    }
    Some(result)
}