use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request<Message, Context> {
    pub max_retries: u64,
    pub ctx: Context,
    pub message: Message,
}

impl<Message, Context: Default> Request<Message, Context> {
    pub fn new(message: Message, max_retries: u64) -> Self {
        Self {
            max_retries,
            message,
            ctx: Context::default(),
        }
    }

    pub fn new_with_ctx(ctx: Context, message: Message, max_retries: u64) -> Self {
        Self {
            max_retries,
            message,
            ctx,
        }
    }
}
