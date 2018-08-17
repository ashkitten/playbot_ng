use crate::module::prelude::*;
use futures::prelude::*;
use futures::future::FutureObj;

pub enum Help {}

impl Module for Help {
    fn init(commands: &mut CommandRegistry) {
        commands.set_named_handler("help", help_handler);
    }
}

fn help_handler<'a>(ctx: &'a Context, _args: &'a [&str]) -> FutureObj<'a, Flow> {
    FutureObj::new((async move || {
        display_help(ctx);
        Flow::Break
    })().boxed())
}

pub fn display_help(ctx: &Context) {
    ctx.reply("Usage help can be found here: https://github.com/panicbit/playbot_ng/tree/master/README.md");
}
