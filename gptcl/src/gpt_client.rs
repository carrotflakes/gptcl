use model::ChatRequest;

use crate::http_client::HttpClient;

// https://platform.openai.com/docs/api-reference/chat/create

#[derive(Debug)]
pub struct GptClient<C: HttpClient> {
    client: C,
    pub api_key: String,
}

impl<C: HttpClient> GptClient<C> {
    pub fn new(client: C, api_key: String) -> Self {
        Self { client, api_key }
    }

    pub async fn call(
        &self,
        request: &ChatRequest,
    ) -> Result<gpt_model::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.client
            .post(
                "https://api.openai.com/v1/chat/completions",
                &self.api_key,
                serde_json::to_string(request).unwrap(),
            )
            .await
            .and_then(|res| {
                // println!("Response: {:?}", &res);
                serde_json::from_slice(&res).or_else(|_| {
                    Err(Box::new(UnexpectedResponseError {
                        body: String::from_utf8_lossy(&res).to_string(),
                    })
                        as Box<dyn std::error::Error + Send + Sync>)
                })
            })
    }
}

pub struct UnexpectedResponseError {
    pub body: String,
}

impl std::fmt::Debug for UnexpectedResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnexpectedResponseError")
            .field("body", &self.body)
            .finish()
    }
}

impl std::fmt::Display for UnexpectedResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("UnexpectedResponseError: body: {}", &self.body))
    }
}

impl std::error::Error for UnexpectedResponseError {}
