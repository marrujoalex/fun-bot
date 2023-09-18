use serenity::builder::{
    CreateApplicationCommand,
};
use serenity::{
    async_trait,
    model::{
        application::{ 
            command::Command,
            interaction::{
                application_command::{
                    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
                },
                Interaction,
                InteractionResponseType
            }
        },
        channel::Message,
        guild::Member,
    },
    prelude::*
};

pub async fn run(ctx: &Context, options: &[CommandDataOption], command: &ApplicationCommandInteraction) {
    //let mut user = ctx.guild().unwrap().member(ctx);
    command
        .create_interaction_response(&ctx, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Hey, I'm here to remove a role!".to_string()))
        })
        .await
        .expect("Remove role failure");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("removerole").description("A command to remove a role")
}
