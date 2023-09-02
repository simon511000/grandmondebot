use config::CONFIG;
use poise::serenity_prelude as serenity;
use std::io::Result;

mod commands;
mod config;
mod server;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() -> Result<()> {
    poise::Framework::builder()
        .token(&CONFIG.token)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::start::start(),
                commands::reload_config::reload_config(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!(
                    "Connecté aux serveurs Discord en tant que {}!",
                    _ready.user.name
                );
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(())
            })
        })
        .intents(serenity::GatewayIntents::privileged() | serenity::GatewayIntents::GUILDS)
        .run()
        .await
        .unwrap();

    Ok(())
}
