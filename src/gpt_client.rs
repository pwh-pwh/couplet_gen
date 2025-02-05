use std::str::FromStr;
use serde_json::{json, Value};
use crate::image_gen::Couplet;

// https://api.chatanywhere.tech  sk-r0VilKsVIiBnCcWJaH18GdsXQQwO5ez6Jl8w2MrdXv7IQWc3
pub fn gen_couplet_by_gpt(theme:&str) -> Couplet{
    let api_key = "sk-r0VilKsVIiBnCcWJaH18GdsXQQwO5ez6Jl8w2MrdXv7IQWc3";
    let url = "https://api.chatanywhere.tech/v1/chat/completions";
    let template = "请根据我的提示生成一组春联，包含上联、下联各一句，和横批，其中上联、下联各7个字，横批4个字，\
    春联主题为：{theme};\\n
直接给出可以被 JSON.parse 解析的字符串，不需要解释内容
     以json格式返回，返回示例如下：\
    {\
        \"banner\": \"新春快乐\",\
        \"top\": \"万事顺遂福满门\",\
        \"bottom\": \"春风得意百花开\"\
    }";
    let content = template.replace("{theme}", theme);
    // println!("content: {content}");
    let response: serde_json::Value = ureq::post(url)
        .set("Authorization", &format!("Bearer {}", api_key))
        .set("Content-Type", "application/json")
        .send_json(json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                { "role": "user", "content": content }
            ]
        }))
        .unwrap()
        .into_json()
        .unwrap();
    let value = &response["choices"][0]["message"]["content"];
    println!("response:{}",response);
    println!("res:{}", value.as_str().unwrap());
    let rv:Value = serde_json::from_str(&value.as_str().unwrap()).unwrap();
    Couplet::new(rv["banner"].as_str().unwrap().to_string(), rv["top"].as_str().unwrap().to_string(), rv["bottom"].as_str().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt() {
        let couplet = gen_couplet_by_gpt("发财");
        println!("{:?}", couplet);
    }

}