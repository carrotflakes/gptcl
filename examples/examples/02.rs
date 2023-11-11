use gptcl::{
    model::{ChatMessage, ChatRequest},
    GptClient,
};
use gptcl_hyper::HyperClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(HyperClient::new(), openai_api_key);

    let mut request = ChatRequest::from_model("gpt-3.5-turbo-1106".to_string());
    request.response_format = Some(gptcl::model::ResponseFormat::Json);
    request.messages = vec![ChatMessage::from_user(
        "Please convert the following yaml to json.\n\nname: John\nage: 20".to_string(),
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
