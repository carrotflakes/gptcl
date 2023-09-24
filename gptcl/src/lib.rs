pub extern crate gpt_model as model;

mod gpt_client;
pub mod http_client;

pub use gpt_client::GptClient;

pub const MODEL_GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
pub const MODEL_GPT_4: &str = "gpt-4";
