pub extern crate gpt_model as model;
#[cfg(feature = "schemars")]
pub use model::schemars;
#[cfg(feature = "schemars")]
pub use schemars::{schema_for, JsonSchema};

mod gpt_client;
pub mod http_client;

pub use gpt_client::GptClient;

pub const MODEL_GPT_3_5_TURBO: &str = "gpt-3.5-turbo";
pub const MODEL_GPT_4: &str = "gpt-4";
pub const MODEL_GPT_4O: &str = "gpt-4o";
pub const MODEL_GPT_4O_MINI: &str = "gpt-4o-mini";
