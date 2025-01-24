use std::sync::Arc;

use gptcl::{
    model::{ChatMessage, ChatRequest, Function, Tool},
    GptClient,
};
use gptcl_hyper::HyperClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let client = GptClient::new(HyperClient::new(), openai_api_key);

    let mut req = ChatRequest::from_model(gptcl::MODEL_GPT_4O_MINI.to_owned());

    req.tools = Arc::new(vec![Tool::Function {
        function: Function {
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
        },
    }]);

    req.messages = vec![ChatMessage::from_user(
        "Tell me what 'LK-99' is".to_string(),
    )];
    let res = client.call(&req).await?;

    println!("Response: {:?}", &res);

    if let Some(tool_call) = res.choices[0].message.tool_calls.first() {
        req.messages.push(res.choices[0].message.clone());
        req.messages.push(ChatMessage::from_tool_response(
            tool_call.id.to_owned(),
            "LK-99 (from the Lee-Kim 1999 research)[2] is a potential room-temperature superconductor with a gray-black appearance.[3]: 8  It is said to have a hexagonal structure that is slightly modified from lead‒apatite by adding small amounts of copper. A team from Korea University led by Sukbae Lee (이석배) and Ji-Hoon Kim (김지훈) began studying this material in 1999.[3]: 1  According to their claims, LK-99 acts as a superconductor at temperatures below 400 K (127 °C; 260 °F) and at ambient pressure.[2][4][3]: 1 ".to_owned(),
        ));

        let res = client.call(&req).await?;
        println!("Response: {:?}", &res);
    }

    Ok(())
}
