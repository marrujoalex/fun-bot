use serenity::builder::CreateApplicationCommand;
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
    },
    prelude::*
};

pub async fn run(ctx: &Context, options: &[CommandDataOption], command: &ApplicationCommandInteraction) {
    //let mut user = ctx.guild().unwrap().member(ctx);
    command
        .create_interaction_response(&ctx, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Hey, I'm alive!".to_string()))
        })
        .await
        .expect("Ping failure");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
