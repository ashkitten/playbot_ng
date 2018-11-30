use crate::module::prelude::*;

pub(crate) enum Help {}

impl Module for Help {
    fn init(commands: &mut CommandRegistry) {
        commands.set_named_handler("help", help_handler);
    }
}

fn help_handler(ctx: &Context, _args: &[&str]) {
    display_help(ctx);
}

pub(crate) fn display_help(ctx: &Context) {
    ctx.reply("Out of order. The Rust community now lives at https://discordapp.com/invite/rust-lang");
    return;
    ctx.reply("Usage help can be found here: https://github.com/panicbit/playbot_ng/tree/master/README.md");
}
