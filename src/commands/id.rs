use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
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
    command
        .create_interaction_response(&ctx, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {

                let option = options
                    .get(0)
                    .expect("Expected user option")
                    .resolved
                    .as_ref()
                    .expect("Expected user object");

                    if let CommandDataOptionValue::User(user, _member) = option {
                        message.content(format!("{}'s id is {}", user.tag(), user.id))
                    } else {
                        message.content("Please provide a valid user".to_string())
                    }
                })
        })
        .await
        .expect("Id retrieval failure");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("id").description("Get a user id").create_option(|option| {
        option
            .name("id")
            .description("The user to lookup")
            .kind(CommandOptionType::User)
            .required(true)
    })
}
