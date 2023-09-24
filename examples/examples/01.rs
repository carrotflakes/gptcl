use gptcl::{model::ChatMessage, GptClient};
use gptcl_hyper::HyperClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(
        Box::new(HyperClient::new()),
        openai_api_key,
        gptcl::MODEL_GPT_3_5_TURBO,
    );

    let res = client
        .call(&[
            ChatMessage::from_user("Hello! How are you today?".to_string()),
            ChatMessage::from_assistant("Hi there! I'm just a computer program, so I don't have feelings, but I'm here to help you with any questions or chat topics you have. What's on your mind today?".to_string()),
            ChatMessage::from_user(
                "That's alright! I just wanted to chat about some interesting travel destinations. Do you have any recommendations for a summer vacation?".to_string(),
            ),
        ])
        .await;

    match res {
        Ok(res) => {
            println!("Response: {}", &res.content.unwrap());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
