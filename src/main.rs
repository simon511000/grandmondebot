use crate::config::Config;
use poise::serenity_prelude::{self as serenity, Activity};
use server::Server;
use std::{
    io::Result,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::{sync::Mutex, time::interval};

mod commands;
mod config;
mod server;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    server: Arc<Mutex<Server>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new().unwrap();
    let server = Arc::new(Mutex::new(Server::new(&config.server)));
    let server_clone = server.clone();

    poise::Framework::builder()
        .token(&config.token)
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::start::start(),
                commands::stop::stop(),
            ],
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!(
                    "Connecté aux serveurs Discord en tant que {}!",
                    ready.user.name
                );
                ctx.set_activity(Activity::playing("Grand Monde")).await;
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                tokio::spawn(async move {
                    let mut interval = interval(Duration::from_secs(30));
                    let mut last_player_connected = SystemTime::now();
            
                    loop {
                        interval.tick().await;
            
                        // Verrouiller le Mutex du serveur
                        let mut server = server_clone.lock().await;
            
                        // Vérifier s'il n'y a aucun joueur connecté
                        if let Ok(nb_players) = server.nb_players().await {
                            if nb_players > 0 {
                                last_player_connected = SystemTime::now();
                                continue;
                            }
            
                            if last_player_connected.elapsed().unwrap() >= Duration::from_secs(300) {
                                let _ = server.stop().await;
                            }
                        } else {
                            // Si le serveur est éteint, on redémarre le compteur
                            last_player_connected = SystemTime::now();
                        }
                    }
                });

                Ok(Data {
                    server: server.clone(),
                })
            })
        })
        .intents(serenity::GatewayIntents::privileged() | serenity::GatewayIntents::GUILDS)
        .run()
        .await
        .unwrap();

    Ok(())
}
