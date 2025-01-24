use serde_json::json;

// https://api.chatanywhere.tech  sk-r0VilKsVIiBnCcWJaH18GdsXQQwO5ez6Jl8w2MrdXv7IQWc3
pub fn gen_couplet_by_gpt(theme:&str){
    let api_key = "sk-r0VilKsVIiBnCcWJaH18GdsXQQwO5ez6Jl8w2MrdXv7IQWc3";
    let url = "https://api.chatanywhere.tech/v1/chat/completions";
    let content = "请根据我的提示生成一组春联，包含上联、下联各一句，和横批，其中上联、下联各7个字，横批4个字，\
    春联主题为：{}\
    以json格式返回，json字段如下：\
    {{\
        \"title\": \"春风得意百花开\",\
        \"top\": \"万事顺遂福满门\",\
        \"bottom\": \"新春快乐\"\
    }}";
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

    println!("{}", response["choices"][0]["message"]["content"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt() {
        gen_couplet_by_gpt("rust技术");
    }
}