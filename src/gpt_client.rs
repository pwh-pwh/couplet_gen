use async_openai::Client;
use async_openai::config::OpenAIConfig;

pub fn get_client()->Client<OpenAIConfig>  {
    let config = OpenAIConfig::default().with_api_base("https://api.openai.com/v1")
        .with_api_key("aa");
    Client::with_config(config)
}