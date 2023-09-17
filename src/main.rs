mod commands;

use std::env;
use dotenv::dotenv;

use serenity::{
    async_trait,
    model::{
        application::{ 
            command::Command,
            interaction::{
                Interaction,
                InteractionResponseType
            }
        },
        channel::Message,
        gateway::Ready,
        id::GuildId
    },
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let response_content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                "addrole" => commands::addrole::run(&command.data.options),
                "removerole" => commands::removerole::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::addrole::register(command))
                .create_application_command(|command| commands::removerole::register(command))
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().ok();

    // handle .env vars
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the env")
        .parse()
        .expect("Application Id must be a u64");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
