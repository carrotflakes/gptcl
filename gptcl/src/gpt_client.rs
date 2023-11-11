use crate::http_client::HttpClient;

// https://platform.openai.com/docs/api-reference/chat/create

#[derive(Debug)]
pub struct GptClient<C: HttpClient> {
    client: C,
    pub api_key: String,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub stop: Option<Vec<String>>,
    pub response_format: Option<gpt_model::ResponseFormat>,
}

impl<C: HttpClient> GptClient<C> {
    pub fn new(client: C, api_key: String, model: &str) -> Self {
        Self {
            client,
            api_key,
            model: model.to_string(),
            max_tokens: None,
            temperature: None,
            top_p: None,
            stop: None,
            response_format: None,
        }
    }

    pub async fn call(
        &self,
        messages: &[gpt_model::ChatMessage],
    ) -> Result<gpt_model::ChatMessage, Box<dyn std::error::Error + Send + Sync>> {
        let res = self.call_(messages, None, &vec![]).await?;
        Ok(res.choices[0].message.clone())
    }

    pub async fn call_n(
        &self,
        messages: &[gpt_model::ChatMessage],
        n: u32,
    ) -> Result<Vec<gpt_model::ChatMessage>, Box<dyn std::error::Error + Send + Sync>> {
        let res = self.call_(messages, Some(n), &vec![]).await?;
        Ok(res.choices.iter().map(|c| c.message.clone()).collect())
    }

    pub async fn call_with_functions(
        &self,
        messages: &[gpt_model::ChatMessage],
        functions: &Vec<gpt_model::Function>,
    ) -> Result<gpt_model::ChatMessage, Box<dyn std::error::Error + Send + Sync>> {
        let res = self.call_(messages, None, functions).await?;
        Ok(res.choices[0].message.clone())
    }

    pub async fn call_(
        &self,
        messages: &[gpt_model::ChatMessage],
        n: Option<u32>,
        functions: &Vec<gpt_model::Function>,
    ) -> Result<gpt_model::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let req_body = gpt_model::ChatRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            n,
            stop: self.stop.clone(),
            response_format: self.response_format.clone(),
            functions,
        };

        self.client
            .post(
                "https://api.openai.com/v1/chat/completions",
                &self.api_key,
                serde_json::to_string(&req_body).unwrap(),
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
