#[cfg(feature = "schemars")]
pub extern crate schemars;
#[cfg(feature = "schemars")]
pub use schemars::{schema_for, JsonSchema};

use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub functions: Arc<Vec<Function>>,
}

impl ChatRequest {
    pub fn from_model(model: String) -> Self {
        Self {
            model,
            messages: vec![],
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            stop: None,
            response_format: None,
            functions: Arc::new(vec![]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

impl ChatMessage {
    pub fn from_user(content: String) -> Self {
        Self {
            role: ChatRole::User,
            content: Some(content),
            name: None,
            function_call: None,
        }
    }

    pub fn from_assistant(content: String) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: Some(content),
            name: None,
            function_call: None,
        }
    }

    pub fn from_system(content: String) -> Self {
        Self {
            role: ChatRole::System,
            content: Some(content),
            name: None,
            function_call: None,
        }
    }

    pub fn from_function_response(function_name: String, content: String) -> Self {
        Self {
            role: ChatRole::Function,
            content: Some(content),
            name: Some(function_name),
            function_call: None,
        }
    }

    #[cfg(feature = "schemars")]
    /// Parse the content of the message as a struct
    pub fn to<'a, T: Deserialize<'a>>(&'a self) -> Option<T> {
        self.content
            .as_ref()
            .and_then(|c| serde_json::from_str(c).ok())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatRole {
    System,
    User,
    Assistant,
    Function,
}

impl Serialize for ChatRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            ChatRole::System => "system",
            ChatRole::User => "user",
            ChatRole::Assistant => "assistant",
            ChatRole::Function => "function",
        })
    }
}

impl<'de> Deserialize<'de> for ChatRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "system" => Ok(ChatRole::System),
            "user" => Ok(ChatRole::User),
            "assistant" => Ok(ChatRole::Assistant),
            _ => Err(serde::de::Error::unknown_variant(
                s.as_str(),
                &["system", "user", "assistant"],
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub logprobs: Option<u64>,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum ResponseFormat {
    Text,
    Json,
    JsonSchema(serde_json::Value),
}

impl Serialize for ResponseFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        match self {
            ResponseFormat::Text => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "text")?;
                map.end()
            }
            ResponseFormat::Json => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "json_object")?;
                map.end()
            }
            ResponseFormat::JsonSchema(schema) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "json_schema")?;
                map.serialize_entry("json_schema", schema)?;
                map.end()
            }
        }
    }
}

#[cfg(feature = "schemars")]
#[macro_export]
macro_rules! response_format_from_struct {
    ($type:ty) => {
        {
            let schema = $crate::schema_for!($type);
            let schema_value = ::serde_json::to_value(&schema).unwrap();
            $crate::ResponseFormat::JsonSchema(
                ::serde_json::json!({
                    "name": schema.schema.metadata.map(|m| m.title).map(|t| t.clone()).unwrap(),
                    "schema": schema_value
                })
            )
        }
    };
}
