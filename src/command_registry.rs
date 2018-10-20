use failure::Error;
use std::collections::HashMap;
use super::{Context, Flow, Command};
use std::iter;
use std::sync::Arc;
use futures::prelude::*;
use crate::Message;

pub(crate) struct CommandRegistry {
    command_prefix: String,
    named_handlers: HashMap<String, Box<Fn(&Context, &[&str]) -> Flow>>,
    fallback_handlers: Vec<Box<Fn(&Context) -> Flow>>,
}

impl CommandRegistry {
    pub fn new(command_prefix: impl Into<String>) -> Self {
        Self {
            command_prefix: command_prefix.into(),
            named_handlers: HashMap::new(),
            fallback_handlers: Vec::new(),
        }
    }

    pub fn set_named_handler(
        &mut self,
        name: impl Into<String>,
        handler: impl Fn(&Context, &[&str]) -> Flow + 'static,
    ) {
        self.named_handlers.insert(name.into(), Box::new(handler));
    }

    pub fn add_fallback_handler(
        &mut self,
        handler: impl Fn(&Context) -> Flow + 'static,
    ) {
        self.fallback_handlers.push(Box::new(handler));
    }

    pub fn handle_message(&self, message: &Message) {
        let context = match Context::new(message) {
            Some(context) => context,
            None => return,
        };

        if self.execute_commands(&context) == Flow::Break {
            return;
        }

        if self.execute_inline_commands(&context) == Flow::Break {
            return;
        }

        self.handle_fallback(&context);
    }

    fn execute_commands(&self, context: &Context) -> Flow {
        if let Some(command) = Command::parse(&self.command_prefix, context.body()) {
            if let Some(handler) = self.named_handlers.get(command.name()) {
                return handler(&context, command.args());
            }
        }
        
        Flow::Continue
    }

    fn execute_inline_commands(&self, context: &Context) -> Flow {
        let mut flow = Flow::Continue;
        let contexts = iter::once(context.clone()).chain(context.inline_contexts());

        for context in contexts.take(3) {
            if let Some(command) = Command::parse(&self.command_prefix, context.body()) {
                if let Some(handler) = self.named_handlers.get(command.name()) {
                    if handler(&context, command.args()) == Flow::Break {
                        flow = Flow::Break;
                    }
                }
            }
        }

        flow
    }

    fn handle_fallback(&self, context: &Context) {
        for handler in &self.fallback_handlers {
            if handler(&context) == Flow::Break {
                return;
            }
        }
    }
}
