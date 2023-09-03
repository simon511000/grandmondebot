use config::Config;
use poise::serenity_prelude as serenity;
use server::Server;
use std::io::Result;
use tokio::sync::Mutex;

mod commands;
mod config;
mod server;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    server: Mutex<Server>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new().unwrap();

    poise::Framework::builder()
        .token(&config.token)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::start::start(),
                commands::stop::stop(),
                commands::reload_config::reload_config(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!(
                    "Connecté aux serveurs Discord en tant que {}!",
                    ready.user.name
                );
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    server: Mutex::new(Server::new(&config.server).await.unwrap()),
                })
            })
        })
        .intents(serenity::GatewayIntents::privileged() | serenity::GatewayIntents::GUILDS)
        .run()
        .await
        .unwrap();

    Ok(())
}
