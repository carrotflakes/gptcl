use gptcl::{
    model::{ChatMessage, Function},
    GptClient,
};
use gptcl_hyper::HyperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(
        Box::new(HyperClient::new()),
        openai_api_key,
        gptcl::MODEL_GPT_3_5_TURBO,
    );

    let functions = vec![Function {
        name: "search_word".to_string(),
        description: Some("You can search about the word from web".to_string()),
        parameters: json!({
            "properties": {
                "word": {
                    "type": "string",
                    "description": "word to search",
                },
            },
            "type": "object",
        }),
    }];

    let mut messages = vec![ChatMessage::from_user(
        "Tell me what 'LK-99' is".to_string(),
    )];
    let res = client.call_with_functions(&messages, &functions).await?;

    println!("Response: {:?}", &res);

    messages.push(res.clone());
    if let Some(_) = res.function_call {
        messages.push(ChatMessage::from_function_response(
            "search_word".to_owned(),
            "LK-99 (from the Lee-Kim 1999 research)[2] is a potential room-temperature superconductor with a gray-black appearance.[3]: 8  It is said to have a hexagonal structure that is slightly modified from lead‒apatite by adding small amounts of copper. A team from Korea University led by Sukbae Lee (이석배) and Ji-Hoon Kim (김지훈) began studying this material in 1999.[3]: 1  According to their claims, LK-99 acts as a superconductor at temperatures below 400 K (127 °C; 260 °F) and at ambient pressure.[2][4][3]: 1 ".to_owned(),
        ));
    }

    let res = client.call_with_functions(&messages, &functions).await?;
    println!("Response: {:?}", &res);

    Ok(())
}
