pub extern crate gpt_model as model;

mod gpt_client;
pub mod http_client;

pub use gpt_client::GptClient;
