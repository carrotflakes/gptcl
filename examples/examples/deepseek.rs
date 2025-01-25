use gptcl::{
    model::{ChatMessage, ChatRequest},
    GptClient,
};
use gptcl_hyper::HyperClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap();

    let mut client = GptClient::new(HyperClient::new(), api_key);
    client.endpoint = "https://api.deepseek.com/chat/completions".to_string();

    let mut request = ChatRequest::from_model("deepseek-chat".to_string());
    request.messages = vec![ChatMessage::from_user(
        "How do you say 'cat' in different languages?".to_string(),
    )];

    let res = client.call(&request).await;

    match res {
        Ok(res) => {
            println!(
                "Response: {}",
                res.choices[0].message.content.as_ref().unwrap()
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    Ok(())
}
