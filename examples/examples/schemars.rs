use gptcl::{
    model::{response_format_from_struct, ChatMessage, ChatRequest},
    schemars, GptClient, JsonSchema,
};
use gptcl_hyper::HyperClient;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(HyperClient::new(), openai_api_key);

    let mut request = ChatRequest::from_model(gptcl::MODEL_GPT_4O_MINI.to_string());
    request.messages = vec![ChatMessage::from_user(
        "Hello! How are you today?".to_string(),
    )];
    request.response_format = Some(response_format_from_struct!(Response));

    let res = client.call(&request).await;

    match res {
        Ok(res) => {
            let parsed = res.choices[0].message.to::<Response>().unwrap();
            println!("Response: {:?}", parsed);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}

#[derive(JsonSchema, Debug, Deserialize)]
struct Response {
    #[allow(dead_code)]
    message: String,
}
