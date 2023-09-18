mod commands;

use std::env;
use dotenv::dotenv;

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
        gateway::Ready,
        id::GuildId
    },
    prelude::*
};

#[derive(Clone)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => self.dispatch_command(ctx, command).await,
            _ => return,
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

impl Handler {
    async fn dispatch_command(&self, ctx: Context, command: ApplicationCommandInteraction) {
        match command.data.name.as_str() {
            "ping" => commands::ping::run(&ctx, &command.data.options, &command).await,
            "id" => commands::id::run(&ctx, &command.data.options, &command).await,
            "addrole" => commands::addrole::run(&ctx, &command.data.options, &command).await,
            "removerole" => commands::removerole::run(&ctx, &command.data.options, &command).await,
            _ => (),
        };
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
