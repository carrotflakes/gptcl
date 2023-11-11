use gptcl::{model::ChatMessage, GptClient};
use gptcl_hyper::HyperClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let mut client = GptClient::new(
        HyperClient::new(),
        openai_api_key,
        "gpt-3.5-turbo-1106",
    );
    client.response_format = Some(gptcl::model::ResponseFormat::Json);

    let res = client
        .call(&[
            ChatMessage::from_user("Please convert the following yaml to json.\n\nname: John\nage: 20".to_string()),
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
