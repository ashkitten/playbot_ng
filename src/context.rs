use regex::Regex;
use std::sync::Arc;
use failure::Error;
use crate::message::{Message, IrcMessage};

#[derive(Clone)]
pub struct Context<'a> {
    body: &'a str,
    message: &'a IrcMessage<'a>,
}

impl<'a> Context<'a> {
    pub fn new(message: &'a IrcMessage) -> Option<Self> {
        Some(Self {
            body: message.body(),
            message,
        })
    }

    pub fn body(&self) -> &'a str {
        self.body
    }

    /// Wether the message was aimed directetly at the bot,
    /// either via private message or by prefixing a channel message with
    /// the bot's name, followed by ',' or ':'.
    pub fn is_directly_addressed(&self) -> bool {
        self.message.is_directly_addressed()
    }

    pub fn reply<S: AsRef<str>>(&self, message: S) -> Result<(), Error> {
        self.message.reply(message)
    }

    pub fn source_nickname(&self) -> &'a str {
        self.message.source_nickname()
    }

    pub fn current_nickname(&self) -> Arc<String> {
        self.message.current_nickname()
    }

    pub fn inline_contexts<'b>(&'b self) -> impl Iterator<Item = Context<'a>> + 'b {
        lazy_static! {
            static ref INLINE_CMD: Regex = Regex::new(r"\{(.*?)}").unwrap();
        }

        let body = if self.is_directly_addressed() { "" } else { self.body };

        let contexts = INLINE_CMD
            .captures_iter(body)
            .flat_map(|caps| caps.get(1))
            .map(move |body| Context {
                body: body.as_str(),
                .. self.clone()
            });
        
        Box::new(contexts)
    }
}
