use std::pin::Pin;

use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;

#[derive(Debug, Clone)]
pub struct HyperClient {
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl HyperClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().build(HttpsConnector::new()),
        }
    }

    pub async fn post(
        &self,
        url: &str,
        api_key: &str,
        body: String,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let req = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", api_key))
            .body(Body::from(body))?;

        let res = self.client.request(req).await?;
        Ok(hyper::body::to_bytes(res.into_body()).await?.to_vec())
    }
}

impl gptcl::http_client::HttpClient for HyperClient {
    fn post<'a>(
        &'a self,
        url: &'a str,
        api_key: &'a str,
        body: String,
    ) -> Pin<
        Box<
            dyn ::core::future::Future<
                    Output = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(self.post(url, api_key, body))
    }
}
