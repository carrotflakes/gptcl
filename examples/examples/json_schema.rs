use gptcl::{
    model::{ChatMessage, ChatRequest, ResponseFormat},
    GptClient,
};
use gptcl_hyper::HyperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(HyperClient::new(), openai_api_key);

    let mut request = ChatRequest::from_model(gptcl::MODEL_GPT_4O_MINI.to_string());
    request.messages = vec![ChatMessage::from_user(
        "Hello! How are you today?".to_string(),
    )];
    request.response_format = Some(ResponseFormat::JsonSchema(
        json!({"name": "response", "schema": {"type":"object","properties":{"message":{"type":"string",}}}}),
    ));

    let res = client.call(&request).await;

    match res {
        Ok(res) => {
            let content = res.choices[0].message.content.as_ref().unwrap();
            let parsed = serde_json::from_str::<serde_json::Value>(content);
            println!("Response: {:?}", parsed);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
