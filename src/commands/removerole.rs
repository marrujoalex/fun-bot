use serenity::builder::{
    CreateApplicationCommand,
    EditMember
};
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::guild::Member;

pub fn run(options: &[CommandDataOption]) -> String {
    "Hey, I'm here to remove a role!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("removerole").description("A command to remove a role")
}